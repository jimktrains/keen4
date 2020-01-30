#![feature(non_ascii_idents)]

extern crate lrlex;
extern crate lrpar;
extern crate cfgrammar;

use std::io;
use std::io::Read;
use lrlex::lrlex_mod;
use lrpar::lrpar_mod;
use std::collections::HashMap;
use std::rc::Rc;

// Using `lrlex_mod!` brings the lexer for `calc.l` into scope.
lrlex_mod!(calc_l);
// Using `lrpar_mod!` brings the lexer for `calc.y` into scope.
lrpar_mod!(calc_y);

use calc_y::Program;

fn read_to_ast() -> Program {
    // We need to get a `LexerDef` for the `calc` language in order that we can lex input.
    let lexerdef = calc_l::lexerdef();
    let mut program_text = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut program_text).expect("Unable to read stdin");
    let mut lexer = lexerdef.lexer(&program_text);
    let (res, errs) = calc_y::parse(&mut lexer);

    if errs.len() != 0 {
        println!("Errors");
        for e in errs {
            println!("{}", e.pp(&lexer, &calc_y::token_epp));
        }
    }
    let res = res.expect("Unable to evaluate expression.");
    let prog = res.expect("No program evaluated");
    prog
}

type Identifier = String;

#[derive(Debug)]
struct SemProgram {
    scope: Scope,
    //reactives: Reactives
}


#[derive(Debug)]
struct EnumValue {
    value: Identifier,
    valueof: Rc<Enum>,
}

#[derive(Debug)]
struct Enum {
    name: Identifier,
    values: Vec<EnumValue>,
}

#[derive(Debug)]
enum Type {
    Untyped,
    Enum(Enum),
}

#[derive(Debug)]
struct Variable {
    name: Identifier,
    vartype: Type,
}

#[derive(Debug)]
struct Scope {
    symbols: HashMap<Identifier, Rc<EnumValue>>,
    types: HashMap<Identifier, Type>,
    variables: HashMap<Identifier, Variable>,
    parent: Option<Box<Scope>>,
}

fn ast_to_semtree(prog:Program) -> Result<SemProgram, ()> {
    let symbols = HashMap::<Identifier, Rc<EnumValue>>::new();
    let types = HashMap::<Identifier, Type>::new();
    let variables = HashMap::<Identifier, Variable>::new();
    let parent = None;
    let scope = Scope { symbols, types, variables, parent };
    for e in prog.enums {
        let name = e.name;
        let values = Vec::<EnumValue>::new();
        let en = Enum { name,  values };

        for v in e.values {
            let value = v;
            let valueof = Rc::new(en);
            let val = EnumValue{value, valueof};

            // Check if the value was previously defined.
            let oldv = scope.symbols.insert(value, Rc::clone(&val));
            match oldv {
                _    => return Err(()),
                None => {},
            }
            en.values.push(val);
        }
    }
    println!("{:?}", scope);
    let semprog = SemProgram { scope };
    Ok(semprog)
}


fn main() -> Result<(), ()> {
    let res = read_to_ast();
    println!("Result: {:?}", res);
    Ok(())
}

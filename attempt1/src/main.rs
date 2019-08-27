#![feature(non_ascii_idents)]

extern crate lrlex;
extern crate lrpar;
extern crate cfgrammar;

use std::io;
use std::io::Read;
use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

// Using `lrlex_mod!` brings the lexer for `calc.l` into scope.
lrlex_mod!(calc_l);
// Using `lrpar_mod!` brings the lexer for `calc.y` into scope.
lrpar_mod!(calc_y);

use calc_y::Program;

fn read_and_parse() -> Program {
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

fn main() -> Result<(), ()> {
    let res = read_and_parse();
    println!("Result: {:?}", res);
    Ok(())
}

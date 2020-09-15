pub mod ast;
#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub keen4); // synthesized by LALRPOP

fn main() {
    let parser = keen4::TermParser::new();
    let x = parser.parse("x");
    assert!(x.is_ok());
    println!("{:?}", x.unwrap());

    let parser = keen4::TermParser::new();
    let x = parser.parse("(y)");
    assert!(x.is_ok());
    println!("{:?}", x.unwrap());

    let x = parser.parse("((z))");
    assert!(x.is_ok());
    println!("{:?}", x.unwrap());

    let x = parser.parse("(x&((z)+y))");
    assert!(x.is_ok());
    println!("{:?}", x.unwrap());

    let x = parser.parse("(x&((z)+y))");
    assert!(x.is_ok());
    println!("{:?}", x.unwrap());

    let x = parser.parse("(((z)+y)&~x)");
    assert!(x.is_ok());
    println!("{:?}", x.unwrap());

    let parser = keen4::ConstraintsParser::new();
    let x = parser.parse("constraint(x,w){x+y,~x+z,w&z}");
    assert!(x.is_ok());
    println!("{:?}", x.unwrap());
}

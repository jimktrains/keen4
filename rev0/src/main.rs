pub mod ast;
#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub keen4); // synthesized by LALRPOP

fn main() {
    let parser = keen4::FactParser::new();
    let x = parser.parse("connected(X)");
    assert!(x.is_ok());
    println!("{:?}", x.unwrap());

    let x = parser.parse("connected(X,Y)");
    assert!(x.is_ok());
    println!("{:?}", x.unwrap());

    let parser = keen4::PredicateParser::new();
    let x = parser.parse("Approach($X, $Y)");
    assert!(x.is_ok());
    println!("{:?}", x.unwrap());

    let parser = keen4::ExprParser::new();
    let x = parser.parse("Approach($X, $Y) & connected($X, $Y)");
    assert!(x.is_ok());
    println!("{:?}", x.unwrap());

    let parser = keen4::ExprParser::new();
    let x = parser.parse("$X & $Y");
    assert!(x.is_ok());
    println!("{:?}", x.unwrap());
}

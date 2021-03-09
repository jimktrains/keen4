#![feature(box_patterns)]

pub mod ast;
pub mod sast;

use ast::{distribute, var};

fn main() -> Result<(), String> {
    let e = var("d") * !(var("e") + !var("b") + !var("d")) * (var("e") + var("c")) + var("a");
    println!("Original:    {}", e);
    let e = distribute(e);
    println!("Distributed: {}", e);
    let mut e = sast::from_ast(e)?;
    println!("SAST: {}", e);
    e.order_terms();
    println!("Ord:  {}", e);
    let e = e.simplify();
    println!("Simp: {}", e);

    Ok(())
}

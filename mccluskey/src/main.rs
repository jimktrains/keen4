#![feature(box_patterns)]

pub mod ast;
pub mod mccluskey;
pub mod sast;
use itertools::Itertools;

use ast::{distribute, var};

fn main() -> Result<(), String> {
    // let e = var("d") * !(var("e") + !var("b") + !var("d")) * (var("e") + var("c"))
    //     + var("a")
    //     + (var("b") * !var("c"));

    let e = (!var("a") * var("b") * !var("c") * !var("d"))
        + (var("a") * !var("b") * !var("c") * !var("d"))
        + (var("a") * !var("b") * var("c") * !var("d"))
        + (var("a") * !var("b") * var("c") * var("d"))
        + (var("a") * var("b") * !var("c") * !var("d"))
        + (var("a") * var("b") * var("c") * var("d"));
    println!("Original:    {}", e);
    let e = distribute(e);
    println!("Distributed: {}", e);
    let mut e = sast::from_ast(e)?;
    println!("SAST: {}", e);
    e.order_terms();
    println!("Ord:  {}", e);
    let e = e.simplify();
    println!("Simp: {}", e);

    println!("Terms: {:?}", e.terms());

    let t = mccluskey::minterms(&e)?;
    println!("MN: {:?}", t);

    let t10 = &t[0].1[0];
    let t11 = &t[0].1[1];

    println!(
        "{:?} {:?} diff {}",
        t10,
        t11,
        mccluskey::count_diff(t10, t11)
    );

    let mut it = t.iter();
    let first = it.next();

    let mut res = vec![];
    if let Some(mut prev) = first {
        for i in it {
            if i.0 == prev.0 || i.0 == (prev.0 + 1) {
                for j in &i.1 {
                    for k in &prev.1 {
                        let cnt = mccluskey::count_diff(&j, &k);
                        if cnt == 1 {
                            let diff = mccluskey::diff(&j, &k);
                            res.push(diff);
                        }
                    }
                }
            }
            prev = i;
        }
    }
    println!("{:?}", res);

    Ok(())
}

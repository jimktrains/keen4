#![feature(box_patterns)]

pub mod ast;
pub mod mccluskey;
pub mod sast;

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
    for (zero_count, g) in t.iter() {
        println!("{}", zero_count);
        for i in g {
            println!("\t{:02} {:?}", mccluskey::number(&i), i);
        }
    }

    let mut it = t.iter();
    let first = it.next();

    if let Some(mut prev) = first {
        let mut res = vec![];
        for i in it {
            if i.0 == prev.0 || i.0 == (prev.0 + 1) {
                for j in &i.1 {
                    for k in &prev.1 {
                        let cnt = mccluskey::count_diff(&j, &k);
                        if cnt == 1 {
                            let diff = mccluskey::diff(&j, &k);
                            res.push((mccluskey::number(&j), mccluskey::number(&k), diff));
                        }
                    }
                }
            }
            prev = i;
        }
        for (i, j, k) in res.iter() {
            println!("{:02},{:02}: {:?}", i, j, k);
        }

        let mut primes = vec![];
        for (i, j, diff) in res.iter() {
            let i_count = res
                .iter()
                .fold(0, |acc, (i2, j2, _)| acc + (i == i2 || i == j2) as i32);
            let j_count = res
                .iter()
                .fold(0, |acc, (i2, j2, _)| acc + (j == i2 || j == j2) as i32);

            if j_count == 1 || i_count == 1 {
                primes.push((i, j, diff));
            }
        }
        println!("Primes");
        for (i, j, k) in primes.iter() {
            println!("{:02},{:02}: {:?}", i, j, k);
        }

        let mut res2 = vec![];
        for (i, j, d) in res.iter() {
            let count = primes.iter().fold(0, |accum, (pi, pj, _)| {
                accum + (*pi == i || *pi == j || *pj == i || *pj == j) as i32
            });
            if count == 0 {
                res2.push((i, j, d));
            }
        }
        for (i, j, d) in res2.iter() {
            println!("{:02},{:02}: {:?}", i, j, d);
        }
        // recurse on finding the primes of res2
    }

    Ok(())
}

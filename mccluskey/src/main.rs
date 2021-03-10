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
    for (zero_count, g) in t.iter() {
        println!("{}", zero_count);
        for i in g {
            println!("\t{:02} {:?}", mccluskey::number(&i), i);
        }
    }

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
        let mut primes: Vec<usize> = vec![];
        for _ in 0..res.len() {
            '_i: for i in 0..res.len() {
                for k in primes.iter() {
                    if i == *k {
                        println!("    i == k");
                        continue '_i;
                    }
                }
                let mut imp1_count = 0;
                let mut imp2_count = 0;
                // Starting from 0 otherwise this won't pick up
                // the final entry or if the conflict happens earlier.
                // I'm sure there are better ways to handle this that aren't
                // quadratic -- it's late and I'm not thinking clearly.
                '_j: for j in 0..res.len() {
                    println!("{} {}", i, j);
                    if i == j {
                        println!("    i == j");
                        continue;
                    }
                    for k in primes.iter() {
                        if res[i].0 == res[*k].0 || res[i].0 == res[j].1 {
                            imp1_count += 1;
                        }
                    }
                    if res[i].0 == res[j].0 || res[i].0 == res[j].1 {
                        println!("    i.0");
                        imp1_count += 1;
                    }
                    if res[i].1 == res[j].0 || res[i].1 == res[j].1 {
                        println!("    i.1");
                        imp2_count += 1;
                    }
                }
                if imp1_count == 0 || imp2_count == 0 {
                    primes.push(i);
                }
            }
        }
        for i in primes {
            println!("{} {:02},{:02}", i, res[i].0, res[i].1);
        }
    }

    Ok(())
}

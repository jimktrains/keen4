#![feature(iterator_fold_self)]
#![feature(type_ascription)]
#![feature(box_patterns)]

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
    println!();

    let parser = keen4::ConstraintsParser::new();
    let x = parser.parse(
        " constraint(SB_stop,SB_appr,SB_clear) {
  (~A_occ & ~B_occ & ~C_occ) -> SB_stop,
  (A_occ | B_occ | C_occ) -> ~SB_stop,

  (A_occ & ~B_occ & ~C_occ) -> SB_clear,
  B_occ -> SB_stop,
  (A_occ & ~B_occ & C_occ)-> SB_appr,

    (SB_appr + SB_clear) + (~SB_clear & ~SB_appr),
    SB_stop + SB_appr + SB_clear,
    }",
    );
    "
  B_occ -> SB_stop,
  C_occ -> SC_stop,


  (~SA_appr & ~SA_stop) -> ~A_occ,
  ~SB_stop -> ~B_occ,
  ";
    println!("{:?}", x);
    assert!(x.is_ok());
    let x = x.unwrap();
    for (i, ftt) in x.free_truth_tables().iter().enumerate() {
        for tt in ftt {
            println!("\t{:?}/{:?} <-> {:?}", x.exprs[i], tt.free_map, tt.result)
        }
    }
    if let Err(errs) = x.assert_bound_used() {
        for e in errs {
            println!("{}", e)
        }
    }
    println!();

    let parser = keen4::ConstraintsParser::new();
    let x = parser.parse(
        "constraint(x){
        x+y,
    }",
    );
    assert!(x.is_ok());
    let x = x.unwrap();
    println!("{:?}", x);
    if let Err(errs) = x.assert_bound_used() {
        for e in errs {
            println!("{}", e)
        }
    }
    println!();
}

use crate::sast::Expr;
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum BoolOrDontCare {
    One,
    Zero,
    DontCare,
}

fn count_ones(a: &Vec<BoolOrDontCare>) -> usize {
    a.iter().filter(|ai| **ai == BoolOrDontCare::One).count()
}

// I can get rid of the Result once I can ensure we have DNF.
pub fn build_number(r: &Expr) -> Result<Vec<(usize, Vec<Vec<BoolOrDontCare>>)>, String> {
    let terms = r.terms();
    let mut numbers = vec![];

    // I should fold this into the types to ensure we're getting a DNF.
    if let Expr::Or(ands) = r {
        for (andi, and) in ands.iter().enumerate() {
            let mut number = vec![];
            // OK, so I don't know that they're only vars or nots yet :(
            match and {
                box Expr::Var(var) => {
                    for (ti, t) in terms.iter().enumerate() {
                        let mut v = BoolOrDontCare::DontCare;
                        if t == var {
                            v = BoolOrDontCare::One;
                        }
                        number.insert(ti, v);
                    }
                }
                box Expr::Not(not) => match not {
                    box Expr::Var(var) => {
                        for (ti, t) in terms.iter().enumerate() {
                            let mut v = BoolOrDontCare::DontCare;
                            if t == var {
                                v = BoolOrDontCare::One;
                            }
                            number.insert(ti, v);
                        }
                    }
                    _ => return Err("Something other than a variable in a not".to_string()),
                },
                box Expr::And(vars) => {
                    for (ti, t) in terms.iter().enumerate() {
                        let mut v = BoolOrDontCare::DontCare;
                        for (_vari, var) in vars.iter().enumerate() {
                            match var {
                                box Expr::Var(n) => {
                                    if n == t {
                                        v = BoolOrDontCare::One;
                                        break;
                                    }
                                }
                                box Expr::Not(j) => match j {
                                    box Expr::Var(n) => {
                                        if n == t {
                                            v = BoolOrDontCare::Zero;
                                            break;
                                        }
                                    }
                                    _ => {
                                        return Err(
                                            "Something besides a var is in a not".to_string()
                                        )
                                    }
                                },
                                _ => {
                                    return Err(
                                        "Something besides a Var or Not in an and".to_string()
                                    )
                                }
                            }
                        }
                        number.insert(ti, v);
                    }
                }
                _ => return Err("Something besides an And in the Or".to_string()),
            }
            numbers.insert(andi, number);
        }
    } else {
        return Err("Something bsdies an Or at the root".to_string());
    }
    numbers.sort_by(|a, b| count_ones(a).cmp(&count_ones(b)));
    Ok(numbers
        .into_iter()
        .group_by(|a| count_ones(a))
        .into_iter()
        .map(|k| (k.0, k.1.collect()))
        .collect())
}

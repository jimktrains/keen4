// Are there Set an Map traits I could use as parameters?
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
struct Var<'a>(&'a str);

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Expr<'a> {
    Var(Var<'a>),
    Not(Box<Expr<'a>>),
    And(Box<Expr<'a>>, Box<Expr<'a>>),
    Implication(Box<Expr<'a>>, Box<Expr<'a>>),
    Biconditional(Box<Expr<'a>>, Box<Expr<'a>>),
    Or(Box<Expr<'a>>, Box<Expr<'a>>),
    Xor(Box<Expr<'a>>, Box<Expr<'a>>),
}

impl<'a> Expr<'a> {
    pub fn cnf(self) -> Box<Expr<'a>> {
        return match self {
            Expr::Var(n) => Box::new(Expr::Var(n)),
            Expr::Not(x) => Box::new(Expr::Not(x.cnf())),
            Expr::And(x, y) => Box::new(Expr::And(x.cnf(), y.cnf())),
            Expr::Implication(p, q) => Box::new(Expr::Or(Box::new(Expr::Not(p)), q)),
            Expr::Biconditional(p, q) => {
                let p = p.cnf();
                let q = q.cnf();
                Box::new(Expr::And(
                    Box::new(Expr::Or(Box::new(Expr::Not(p.clone())), q.clone())),
                    Box::new(Expr::Or(p, Box::new(Expr::Not(q)))),
                ))
            }
            Expr::Xor(p, y) => Expr::And(
                Box::new(Expr::Or(p.clone(), y.clone())),
                Box::new(Expr::Not(Box::new(Expr::And(p, y)))),
            )
            .cnf(),
            Expr::Or(p, y) => match (*p, *y) {
                (p, Expr::And(q, r)) => {
                    let p = p.cnf();
                    let q = q.cnf();
                    let r = r.cnf();
                    Box::new(Expr::And(
                        Box::new(Expr::Or(p.clone(), q.clone())),
                        Box::new(Expr::Or(p.clone(), r.clone())),
                    ))
                }
                (Expr::And(q, r), y) => {
                    let y = y.cnf();
                    let q = q.cnf();
                    let r = r.cnf();
                    Box::new(Expr::And(
                        Box::new(Expr::Or(y.clone(), q.clone())),
                        Box::new(Expr::Or(y.clone(), r.clone())),
                    ))
                }
                (p, y) => Box::new(Expr::Or(p.cnf(), y.cnf())),
            },
        };
    }

    pub fn vars(&self) -> HashSet<Var<'a>> {
        let mut vars: HashSet<Var<'a>> = HashSet::new();
        match self {
            Expr::Var(n) => {
                vars.insert(*n);
                vars
            }
            Expr::Not(n) => vars.union(&n.vars()).map(|x| x.clone()).collect(),
            Expr::And(x, y)
            | Expr::Implication(x, y)
            | Expr::Biconditional(x, y)
            | Expr::Xor(x, y)
            | Expr::Or(x, y) => vars
                .union(&x.vars())
                .map(|x| x.clone())
                .collect::<HashSet<Var<'a>>>()
                .union(&y.vars())
                .map(|x| x.clone())
                .collect(),
        }
    }

    pub fn pp(&self) -> String {
        match self {
            Expr::Var(n) => String::from(n.0),
            Expr::Not(n) => String::from("~") + &n.pp(),
            Expr::And(x, y) => String::from("(") + &x.pp() + " & " + &y.pp() + ")",
            Expr::Implication(x, y) => String::from("(") + &x.pp() + " -> " + &y.pp() + ")",
            Expr::Biconditional(x, y) => String::from("(") + &x.pp() + " <-> " + &y.pp() + ")",
            Expr::Or(x, y) => String::from("(") + &x.pp() + " | " + &y.pp() + ")",
            Expr::Xor(x, y) => String::from("(") + &x.pp() + " ^ " + &y.pp() + ")",
        }
    }

    pub fn eval(&self, vals: &HashMap<Var<'a>, bool>) -> bool {
        match self {
            // Yes, right now let's just panic if not all vars are present.
            Expr::Var(n) => *vals.get(&n).unwrap(),
            Expr::Not(n) => !n.eval(vals),
            Expr::And(x, y) => x.eval(vals) & y.eval(vals),
            Expr::Implication(x, y) => !x.eval(vals) | y.eval(vals),
            Expr::Biconditional(x, y) => {
                let x = x.eval(vals);
                let y = y.eval(vals);
                (!x | y) & (x | !y)
            }
            Expr::Or(x, y) => x.eval(vals) | y.eval(vals),
            Expr::Xor(x, y) => x.eval(vals) ^ y.eval(vals),
        }
    }

    fn generate_vals_map(&self, vars: &Vec<Var<'a>>, vals: &Vec<bool>) -> HashMap<Var<'a>, bool> {
        vars.iter()
            .map(|x| x.clone())
            .zip(vals.iter().map(|x| *x))
            .collect()
    }

    fn truth_table_row(&self, vars: &Vec<Var<'a>>, vals: &mut Vec<bool>, col: usize) {
        if vars.len() == col {
            println!(
                "{} | {}",
                vals.iter()
                    .map(|x| match x {
                        true => String::from("t"),
                        false => String::from("f"),
                    })
                    .collect::<Vec<String>>()
                    .join(" "),
                self.eval(&self.generate_vals_map(vars, vals))
            )
        } else {
            vals[col] = false;
            self.truth_table_row(vars, vals, col + 1);
            vals[col] = true;
            self.truth_table_row(vars, vals, col + 1);
        }
    }

    pub fn print_truth_table(&self) {
        let vars: Vec<Var<'a>> = self.vars().iter().map(|x| x.clone()).collect();
        let mut vals: Vec<bool> = vars.iter().map(|_| false).collect();

        println!(
            "{} | Result",
            vars.iter()
                .map(|x| String::from(x.0))
                .collect::<Vec<String>>()
                .join(" ")
        );
        self.truth_table_row(&vars, &mut vals, 0);
    }
}

fn main() {
    let x = Var("x");
    let y = Var("y");
    let z = Var("z");
    let a = Var("a");
    let expr = Expr::Or(
        Box::new(Expr::Var(z)),
        Box::new(Expr::And(
            Box::new(Expr::Biconditional(
                Box::new(Expr::Var(y)),
                Box::new(Expr::Var(a)),
            )),
            Box::new(Expr::Implication(
                Box::new(Expr::Xor(Box::new(Expr::Var(x)), Box::new(Expr::Var(a)))),
                Box::new(Expr::Var(y)),
            )),
        )),
    );
    let cnf = expr.clone().cnf();
    println!("Expr: {:?}", expr);
    println!("      {}", expr.pp());
    println!("CNF:  {:?}", cnf);
    println!("      {}", cnf.pp());
    println!(
        "Vars: {}",
        expr.vars()
            .iter()
            .map(|x| String::from(x.0))
            .collect::<Vec<String>>()
            .join(", ")
    );
    expr.print_truth_table()
}

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
    pub fn var(s: &'a str) -> Box<Self> {
        Box::new(Expr::Var(Var(s)))
    }
    pub fn not(p: Box<Self>) -> Box<Self> {
        Box::new(Self::Not(p))
    }
    pub fn and(p: Box<Self>, q: Box<Self>) -> Box<Self> {
        Box::new(Self::And(p, q))
    }
    pub fn or(p: Box<Self>, q: Box<Self>) -> Box<Self> {
        Box::new(Self::Or(p, q))
    }
    pub fn xor(p: Box<Self>, q: Box<Self>) -> Box<Self> {
        Box::new(Self::Xor(p, q))
    }
    pub fn implication(p: Box<Self>, q: Box<Self>) -> Box<Self> {
        Box::new(Self::Implication(p, q))
    }
    pub fn biconditional(p: Box<Self>, q: Box<Self>) -> Box<Self> {
        Box::new(Self::Biconditional(p, q))
    }
    pub fn cnf(self) -> Box<Expr<'a>> {
        return match self {
            Expr::Var(n) => Expr::var(n.0),
            Expr::Not(x) => Expr::not(x.cnf()),
            Expr::And(x, y) => Expr::and(x.cnf(), y.cnf()),
            Expr::Implication(p, q) => Expr::or(Expr::not(p), q),
            Expr::Biconditional(p, q) => {
                let p = p.cnf();
                let q = q.cnf();
                Expr::and(
                    Expr::or(Expr::not(p.clone()), q.clone()),
                    Expr::or(p, Expr::not(q)),
                )
            }
            Expr::Xor(p, y) => {
                Expr::and(Expr::or(p.clone(), y.clone()), Expr::not(Expr::and(p, y))).cnf()
            }
            Expr::Or(p, y) => match (*p, *y) {
                (p, Expr::And(q, r)) => {
                    let p = p.cnf();
                    let q = q.cnf();
                    let r = r.cnf();
                    Expr::and(
                        Expr::or(p.clone(), q.clone()),
                        Expr::or(p.clone(), r.clone()),
                    )
                }
                (Expr::And(q, r), y) => {
                    let y = y.cnf();
                    let q = q.cnf();
                    let r = r.cnf();
                    Expr::and(
                        Expr::or(y.clone(), q.clone()),
                        Expr::or(y.clone(), r.clone()),
                    )
                }
                (p, y) => Expr::or(p.cnf(), y.cnf()),
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
            Expr::Not(n) => format!("~{}", n.pp()),
            Expr::And(x, y) => format!("({} & {})", x.pp(), y.pp()),
            Expr::Implication(x, y) => format!("({} -> {})", x.pp(), y.pp()),
            Expr::Biconditional(x, y) => format!("({} <-> {})", x.pp(), y.pp()),
            Expr::Or(x, y) => format!("({} | {})", x.pp(), y.pp()),
            Expr::Xor(x, y) => format!("({} ^ {})", x.pp(), y.pp()),
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
    let x = "x";
    let y = "y";
    let z = "z";
    let a = "a";
    let expr = Expr::or(
        Expr::var(z),
        Expr::and(
            Expr::biconditional(Expr::var(y), Expr::var(a)),
            Expr::implication(Expr::xor(Expr::var(x), Expr::var(a)), Expr::var(y)),
        ),
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

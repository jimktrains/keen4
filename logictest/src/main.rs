// Are there Set an Map traits I could use as parameters?
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy, PartialOrd, Ord)]
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
    True(),
    False(),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
enum CNFTerm<'a> {
    Var(Var<'a>),
    Not(Var<'a>),
}
impl<'a> PartialOrd for CNFTerm<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> Ord for CNFTerm<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (CNFTerm::Var(x), CNFTerm::Var(y))
            | (CNFTerm::Var(x), CNFTerm::Not(y))
            | (CNFTerm::Not(x), CNFTerm::Var(y))
            | (CNFTerm::Not(x), CNFTerm::Not(y)) => x.cmp(y),
        }
    }
}
impl<'a> CNFTerm<'a> {
    pub fn pp(&self) -> String {
        match *self {
            CNFTerm::Var(x) => format!("{}", x.0),
            CNFTerm::Not(x) => format!("~{}", x.0),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
struct CNF<'a>(Vec<Vec<CNFTerm<'a>>>);

impl<'a> CNF<'a> {
    pub fn pp(self) -> String {
        self.0
            .iter()
            .map(|line| {
                let mut line = line.clone();
                line.sort();
                format!(
                    "{}",
                    line.iter()
                        .map(|x| format!("{:>2}", x.pp()))
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
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
    pub fn truth() -> Box<Self> {
        Box::new(Self::True())
    }
    pub fn falsey() -> Box<Self> {
        Box::new(Self::False())
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

    pub fn simplify(self) -> Box<Expr<'a>> {
        match self {
            Expr::Var(n) => Expr::var(n.0),
            Expr::Not(x) => match *x {
                Expr::Var(n) => Expr::not(Expr::var(n.0)),
                Expr::Not(n) => n.simplify(),
                Expr::And(p, q) => {
                    Expr::or(Expr::not(p).simplify(), Expr::not(q).simplify()).simplify()
                }
                Expr::Or(p, q) => {
                    Expr::and(Expr::not(p).simplify(), Expr::not(q).simplify()).simplify()
                }
                Expr::True() => Expr::falsey(),
                Expr::False() => Expr::truth(),
                Expr::Xor(p, q) => Expr::not(Expr::xor(p.simplify(), q.simplify()).simplify()),
                Expr::Implication(p, q) => {
                    Expr::not(Expr::implication(p.simplify(), q.simplify()).simplify())
                }
                Expr::Biconditional(p, q) => {
                    Expr::not(Expr::biconditional(p.simplify(), q.simplify()).simplify())
                }
            },
            Expr::True() => Expr::truth(),
            Expr::False() => Expr::falsey(),
            Expr::And(p, y) => match (*p, *y) {
                (Expr::True(), p) | (p, Expr::True()) => Box::new(p),
                //(p, Expr::Or(q, r)) | (Expr::Or(q, r), p) => {
                //    let p = p.simplify();
                //    let q = q.simplify();
                //    let r = r.simplify();
                //    Expr::or(
                //        Expr::and(p.clone(), q.clone()),
                //        Expr::and(p.clone(), r.clone()),
                //    )
                //}
                (Expr::Var(v), Expr::And(p, q)) | (Expr::And(p, q), Expr::Var(v)) => Expr::or(
                    Expr::and(Expr::var(v.0), p.simplify()),
                    Expr::and(Expr::var(v.0), q.simplify()),
                ),
                (Expr::Var(v), Expr::Var(w)) => {
                    if v == w {
                        Expr::var(v.0)
                    } else {
                        Expr::and(Expr::var(v.0), Expr::var(w.0))
                    }
                }
                (Expr::Var(v), Expr::Not(w)) => match *w {
                    Expr::Var(x) => {
                        if v == x {
                            Expr::falsey()
                        } else {
                            Expr::and(Expr::var(v.0), Expr::not(w))
                        }
                    }
                    _ => Expr::and(Expr::var(v.0), Expr::not(w)),
                },
                (p, y) => Expr::and(p.simplify(), y.simplify()),
            },
            Expr::Implication(p, q) => Expr::or(Expr::not(p.simplify()), q.simplify()).simplify(),
            Expr::Biconditional(p, q) => Expr::biconditional(p.simplify(), q.simplify()),
            Expr::Xor(p, q) => Expr::xor(p.simplify(), q.simplify()),
            Expr::Or(p, y) => match (*p, *y) {
                (Expr::True(), _) | (_, Expr::True()) => Expr::truth(),
                (p, Expr::And(q, r)) | (Expr::And(q, r), p) => {
                    let p = p.simplify();
                    let q = q.simplify();
                    let r = r.simplify();
                    Expr::and(
                        Expr::or(p.clone(), q.clone()),
                        Expr::or(p.clone(), r.clone()),
                    )
                }
                (Expr::Var(v), Expr::Or(p, q)) | (Expr::Or(p, q), Expr::Var(v)) => Expr::or(
                    Expr::or(Expr::var(v.0), p.simplify()),
                    Expr::or(Expr::var(v.0), q.simplify()),
                ),
                (Expr::Var(v), Expr::Var(w)) => {
                    if v == w {
                        Expr::var(v.0)
                    } else {
                        Expr::or(Expr::var(v.0), Expr::var(w.0))
                    }
                }
                (Expr::Var(v), Expr::Not(w)) => match *w {
                    Expr::Var(x) => {
                        if v == x {
                            Expr::truth()
                        } else {
                            Expr::or(Expr::var(v.0), Expr::not(Expr::var(x.0)))
                        }
                    }
                    _ => Expr::or(Expr::var(v.0), Expr::not(w)),
                },
                (p, y) => Expr::or(p.simplify(), y.simplify()),
            },
        }
    }
    pub fn simplify_cnf(self) -> Box<Expr<'a>> {
        match *self.simplify() {
            Expr::Var(n) => Expr::var(n.0),
            Expr::Not(x) => Expr::not(x.simplify_cnf()),
            Expr::And(x, y) => Expr::and(x.simplify_cnf(), y.simplify_cnf()),
            Expr::Implication(p, q) => {
                Expr::or(Expr::not(p.simplify_cnf()), q.simplify_cnf()).simplify_cnf()
            }
            Expr::Biconditional(p, q) => {
                let p = p.simplify_cnf();
                let q = q.simplify_cnf();
                Expr::and(
                    Expr::or(Expr::not(p.clone()), q.clone()).simplify_cnf(),
                    Expr::or(p, Expr::not(q)).simplify_cnf(),
                )
            }
            Expr::Xor(p, y) => {
                Expr::and(Expr::or(p.clone(), y.clone()), Expr::not(Expr::and(p, y))).simplify_cnf()
            }
            Expr::True() => Expr::truth(),
            Expr::False() => Expr::falsey(),
            Expr::Or(p, q) => Expr::or(p.simplify_cnf(), q.simplify_cnf()),
        }
    }

    pub fn vars(&self) -> HashSet<Var<'a>> {
        let mut vars: HashSet<Var<'a>> = HashSet::new();
        match self {
            Expr::True() => vars,
            Expr::False() => vars,
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
            Expr::Xor(x, y) => format!("({} + {})", x.pp(), y.pp()),
            Expr::True() => String::from("T"),
            Expr::False() => String::from("F"),
        }
    }

    pub fn eval(&self, vals: &HashMap<Var<'a>, bool>) -> bool {
        match self {
            // Yes, right now let's just panic if not all vars are present.
            Expr::Var(n) => vals[&n],
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
            Expr::True() => true,
            Expr::False() => false,
        }
    }

    fn generate_vals_map(&self, vars: &Vec<Var<'a>>, vals: &Vec<bool>) -> HashMap<Var<'a>, bool> {
        vars.iter()
            .map(|x| x.clone())
            .zip(vals.iter().map(|x| *x))
            .collect()
    }

    fn truth_table_row(
        &self,
        vars: &Vec<Var<'a>>,
        vals: &mut Vec<bool>,
        col: usize,
    ) -> Vec<Vec<bool>> {
        if vars.len() == col {
            let mut x = vals.clone();
            x.push(self.eval(&self.generate_vals_map(vars, vals)));
            vec![x]
        } else {
            let mut x: Vec<Vec<bool>> = vec![];
            vals[col] = false;
            x.append(&mut self.truth_table_row(vars, vals, col + 1));
            vals[col] = true;
            x.append(&mut self.truth_table_row(vars, vals, col + 1));
            x
        }
    }

    pub fn truth_table(&self) -> (Vec<Var<'a>>, Vec<Vec<bool>>) {
        let mut vars: Vec<Var<'a>> = self.vars().into_iter().collect::<Vec<Var<'a>>>();
        vars.sort();
        let mut vals: Vec<bool> = vars.iter().map(|_| false).collect();

        (vars.clone(), self.truth_table_row(&vars, &mut vals, 0))
    }

    pub fn truth_table_pp(&self) -> String {
        let (vars, vals) = self.truth_table();
        let header = vars
            .into_iter()
            .map(|x| String::from(x.0))
            .collect::<Vec<String>>()
            .join(" ")
            + " Result";
        let body = vals
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .map(|x| String::from(if x { "T" } else { "F" }))
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");

        return header + "\n" + &body;
    }

    fn to_cnf(self) -> CNF<'a> {
        match self {
            Expr::And(p, q) => {
                let mut pr = p.to_cnf();
                let mut qr = q.to_cnf();
                pr.0.append(&mut qr.0);
                pr
            }
            Expr::Or(p, q) => {
                // Since there will be no further Ands, we can just take
                // the first vector and append it.
                let mut pr = p.to_cnf();
                let mut qr = q.to_cnf();
                // There should always be a value here because there is
                // a node here.
                pr.0[0].append(&mut qr.0[0]);
                pr.0[0] = pr.0[0]
                    .iter()
                    .map(|x| x.clone())
                    .collect::<HashSet<CNFTerm<'a>>>()
                    .iter()
                    .map(|x| x.clone())
                    .collect::<Vec<CNFTerm<'a>>>();
                pr
            }
            Expr::Var(v) => CNF(vec![vec![CNFTerm::Var(v)]]),
            Expr::Not(v) => match *v {
                Expr::Var(v) => CNF(vec![vec![CNFTerm::Not(v)]]),
                _ => panic!("There is a not with more than a variable"),
            },
            _ => panic!("There is more than and, or, var, and notvar!"),
        }
    }
    pub fn cnf(&self) -> CNF<'a> {
        let x = self.cnf_expr();

        x.to_cnf()
    }
    pub fn cnf_expr(&self) -> Self {
        let mut x = self.clone();
        let mut y = x.clone();

        x = *x.simplify_cnf();

        while x.pp() != y.pp() {
            y = x.clone();
            x = *x.simplify_cnf();
        }

        x
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
    let expr2 = Expr::or(
        Expr::var(a),
        Expr::or(Expr::var(x), Expr::or(Expr::var(y), Expr::var(z))),
    );
    println!("Expr: {}", expr.pp());
    let cnf = expr.cnf_expr();
    println!("CNF:  {}", cnf.pp());
    println!(
        "Vars: {}",
        expr.vars()
            .iter()
            .map(|x| String::from(x.0))
            .collect::<Vec<String>>()
            .join(", ")
    );

    println!();
    println!(
        "truth_table(Expr) == truth_table(CNF) => {}",
        cnf.truth_table() == expr.truth_table()
    );

    println!();
    println!("CNF Table:\n{}", cnf.cnf().pp());

    println!();
    println!("Truth Table:\n{}", cnf.truth_table_pp());

    println!("=====================");
    println!("Expr 2");
    println!("=====================");
    println!("Expr: {}", expr2.pp());
    let cnf = expr2.cnf_expr();
    println!("CNF:  {}", cnf.pp());
    println!(
        "Vars: {}",
        expr2
            .vars()
            .iter()
            .map(|x| String::from(x.0))
            .collect::<Vec<String>>()
            .join(", ")
    );

    println!();
    println!(
        "truth_table(Expr2) == truth_table(CNF2) => {}",
        cnf.truth_table() == expr2.truth_table()
    );

    println!();
    println!("CNF Table:\n{}", cnf.cnf().pp());

    println!();
    println!("Truth Table:\n{}", cnf.truth_table_pp());

    println!();
    println!(
        "truth_table(Expr) == truth_table(Expr2) => {}",
        expr.truth_table() == expr2.truth_table()
    );
}

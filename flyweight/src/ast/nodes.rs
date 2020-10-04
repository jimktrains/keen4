use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};

type TermName<'a> = &'a str;

#[derive(PartialEq, Eq, Clone)]
pub enum Expr<'a> {
    Term(TermName<'a>),
    And(Box<Expr<'a>>, Box<Expr<'a>>),
    Or(Box<Expr<'a>>, Box<Expr<'a>>),
    Implication(Box<Expr<'a>>, Box<Expr<'a>>),
    Biconditional(Box<Expr<'a>>, Box<Expr<'a>>),
    Xor(Box<Expr<'a>>, Box<Expr<'a>>),
    Not(Box<Expr<'a>>),
    True,
    False,
}

impl<'a> fmt::Display for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pp(f)
    }
}
impl<'a> fmt::Debug for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pp(f)
    }
}

impl<'a> Hash for Expr<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ppf().hash(state);
    }
}

#[derive(Debug)]
pub struct Constraints<'a> {
    pub bound: Vec<&'a str>,
    pub exprs: Vec<Expr<'a>>,
}

#[derive(PartialEq, Eq, Clone)]
pub struct VarMap<'a>(HashMap<&'a str, bool>);

impl<'a> fmt::Display for VarMap<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl<'a> fmt::Debug for VarMap<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TruthTable<'a> {
    pub free_map: VarMap<'a>,
    pub result: Expr<'a>,
}

impl<'a> Hash for VarMap<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut vars = self
            .0
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<(&str, bool)>>();
        vars.sort();

        vars.iter().for_each(|(k, v)| {
            k.hash(state);
            v.hash(state);
        });
    }
}

impl<'a> Constraints<'a> {
    pub fn assert_bound_used(&self) -> Result<(), Vec<String>> {
        let mut errs = vec![];

        let ftts = self.free_truth_tables();
        let mut bound_solvable_exprs = HashMap::new();
        for (i, ftt) in ftts.iter().enumerate() {
            for tt in ftt {
                let btt = self.bound_truth_table(&tt.result);
                let btt_true: Vec<&TruthTable> =
                    btt.iter().filter(|tt| tt.result != Expr::False).collect();
                if btt_true.len() == 0 {
                    errs.push(format!(
                        "{:?} has no solution when {:?}",
                        tt.result, tt.free_map
                    ))
                } else {
                    bound_solvable_exprs
                        .entry(&tt.free_map)
                        .or_insert(HashSet::new())
                        .insert(&tt.result);
                }
            }
            for (free_map, bse) in &bound_solvable_exprs {
                let all_solvable_expr = bse
                    .iter()
                    .map(|a| (*a).clone())
                    .collect::<HashSet<Expr>>()
                    .iter()
                    .map(|a| (*a).clone())
                    .fold_first(|a, b| Expr::And(Box::new(a), Box::new(b)).simplify())
                    .unwrap();
                let all_btt = self.bound_truth_table(&all_solvable_expr);
                println!();
                println!();
                println!();
                println!("free_map: {:?}", free_map);
                println!("bse: {:?}", bse);
                println!("all_solvable_expr: {:?}", all_solvable_expr);
                println!("all_btt: {:?}", all_btt);
                let bound_solutions: Vec<&TruthTable> = all_btt
                    .iter()
                    .filter(|tt| tt.result == Expr::True)
                    .collect();
                if bound_solutions.len() == 0 {
                    let msg = format!(
                        "{:?} has no solution when {:?}",
                        all_solvable_expr, free_map
                    );
                    println!("{}", msg);
                    errs.push(msg);
                } else if bound_solutions.len() != 1 {
                    let msg = format!(
                        "{:?}/{:?} has multiple solutions: {:?}",
                        all_solvable_expr,
                        free_map,
                        bound_solutions.iter().map(|tt| &tt.free_map).collect(): Vec<&VarMap>
                    );
                    println!("{}", msg);
                    errs.push(msg);
                }
                let bound = self.bound_vars_in_expr(&all_solvable_expr);

                let unused_bound: Vec<&str> = self
                    .bound
                    .iter()
                    .filter(|b| !bound.contains(b))
                    .map(|b| *b)
                    .collect();
                if unused_bound.len() != 0 {
                    errs.push(format!(
                        "Bound variables {:?} are never used in any constraint when {:?}",
                        unused_bound, free_map,
                    ));
                }
            }
        }
        let all_expr = self
            .exprs
            .iter()
            .map(|a| (*a).clone())
            .fold_first(|a, b| Expr::And(Box::new(a), Box::new(b)).simplify())
            .unwrap();
        let all_ftt = self.free_truth_table(&all_expr);
        let bound = self.bound_vars_in_expr(&all_expr);

        for tt in all_ftt {
            if tt.result == Expr::True || tt.result == Expr::False {
                errs.push(format!(
                    "When {:?} then {} is {}, hence bound vars {:?} have no solution",
                    tt.free_map, all_expr, tt.result, bound
                ))
            }
        }

        let unused_bound: Vec<&str> = self
            .bound
            .iter()
            .filter(|b| !bound.contains(b))
            .map(|b| *b)
            .collect();

        if unused_bound.len() != 0 {
            errs.push(format!(
                "Bound variables {:?} are never used in any constraint",
                unused_bound,
            ));
        }

        if errs.len() != 0 {
            return Err(errs);
        }

        Ok(())
    }
    pub fn bound_vars_in_expr(&self, e: &Expr<'a>) -> Vec<&'a str> {
        let vars = e.variables();
        self.bound
            .iter()
            .filter(|v| vars.contains(*v))
            .map(|v| *v)
            .collect()
    }

    pub fn bound_truth_table(&self, e: &Expr<'a>) -> Vec<TruthTable> {
        let vars = self.bound_vars_in_expr(e);
        (0..(1 << vars.len()))
            .map(|n| {
                let mut run = HashMap::<&'a str, bool>::new();
                for (i, v) in vars.iter().enumerate() {
                    let p = 1 << i;
                    run.insert(v, p & n != 0);
                }
                TruthTable {
                    free_map: VarMap(run.clone()),
                    result: e.evaluate(&run),
                }
            })
            .collect()
    }
    pub fn free_truth_table(&self, e: &Expr<'a>) -> Vec<TruthTable> {
        let vars: Vec<&str> = self
            .exprs
            .iter()
            .map(|e| e.variables())
            .flatten()
            .filter(|v| !self.bound.contains(v))
            .collect();

        (0..(1 << vars.len()))
            .map(|n| {
                let mut run = HashMap::<&'a str, bool>::new();
                for (i, v) in vars.iter().enumerate() {
                    let p = 1 << i;
                    run.insert(v, p & n != 0);
                }
                TruthTable {
                    free_map: VarMap(run.clone()),
                    result: e.evaluate(&run),
                }
            })
            .collect()
    }

    pub fn free_truth_tables(&self) -> Vec<Vec<TruthTable>> {
        self.exprs
            .iter()
            .map(|e| self.free_truth_table(e))
            .collect()
    }
}

impl<'a> Expr<'a> {
    fn simplify(self) -> Expr<'a> {
        match self {
            Expr::Not(box a) => match a {
                Expr::Not(b) => b.simplify(),
                Expr::True => Expr::False,
                Expr::False => Expr::True,
                _ => Expr::Not(Box::new(a.simplify())),
            },
            Expr::And(box a, box b) => match (a, b) {
                (Expr::True, c) | (c, Expr::True) => c,
                (Expr::False, _) | (_, Expr::False) => Expr::False,
                (c, d) => Expr::And(Box::new(c), Box::new(d)),
            },
            _ => self,
        }
    }

    fn pp(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Term(x) => write!(f, "{}", x),
            Expr::False => write!(f, "𝔽"),
            Expr::True => write!(f, "𝕋"),
            Expr::And(a, b) => write!(f, "({} & {})", a, b),
            Expr::Or(a, b) => write!(f, "({} | {})", a, b),
            Expr::Implication(a, b) => write!(f, "({} -> {})", a, b),
            Expr::Biconditional(a, b) => write!(f, "({} <-> {})", a, b),
            Expr::Xor(a, b) => write!(f, "({} + {})", a, b),
            Expr::Not(a) => write!(f, "~{}", a),
        }
    }
    fn ppf(&self) -> String {
        match self {
            Expr::Term(x) => format!("{}", x),
            Expr::False => format!("𝔽"),
            Expr::True => format!("𝕋"),
            Expr::And(a, b) => format!("({} & {})", a, b),
            Expr::Or(a, b) => format!("({} | {})", a, b),
            Expr::Implication(a, b) => format!("({} -> {})", a, b),
            Expr::Biconditional(a, b) => format!("({} <-> {})", a, b),
            Expr::Xor(a, b) => format!("({} + {})", a, b),
            Expr::Not(a) => format!("~{}", a),
        }
    }
    pub fn contains(&self, t: TermName<'a>) -> bool {
        match self {
            Expr::Term(x) => *x == t,
            Expr::False => false,
            Expr::True => false,
            Expr::And(a, b)
            | Expr::Or(a, b)
            | Expr::Implication(a, b)
            | Expr::Biconditional(a, b)
            | Expr::Xor(a, b) => a.contains(t) | b.contains(t),
            Expr::Not(a) => a.contains(t),
        }
    }

    fn _variables(&self) -> Vec<&'a str> {
        match self {
            Expr::Term(a) => vec![a],
            Expr::False => vec![],
            Expr::True => vec![],
            Expr::And(a, b)
            | Expr::Or(a, b)
            | Expr::Implication(a, b)
            | Expr::Biconditional(a, b)
            | Expr::Xor(a, b) => {
                let mut x = a._variables();
                x.append(&mut b._variables());
                x
            }
            Expr::Not(a) => a._variables(),
        }
    }

    pub fn variables(&self) -> HashSet<&'a str> {
        let mut x = HashSet::new();
        self._variables().iter().for_each(|v| {
            x.insert(*v);
        });
        x
    }

    pub fn evaluate(&self, m: &HashMap<TermName<'a>, bool>) -> Expr<'a> {
        match self {
            Expr::False => Expr::False,
            Expr::True => Expr::True,
            Expr::Term(x) => match m.get(x) {
                Some(v) => match v {
                    true => Expr::True,
                    false => Expr::False,
                },
                None => Expr::Term(x),
            },
            Expr::And(a, b) => {
                let a = a.evaluate(m);
                let b = b.evaluate(m);

                match (a, b) {
                    (Expr::False, _) | (_, Expr::False) => Expr::False,
                    (Expr::True, Expr::True) => Expr::True,
                    (Expr::True, x) | (x, Expr::True) => x,
                    (x, y) => Expr::And(Box::new(x), Box::new(y)),
                }
            }
            Expr::Or(a, b) => {
                let a = a.evaluate(m);
                let b = b.evaluate(m);

                match (a, b) {
                    (Expr::True, _) | (_, Expr::True) => Expr::True,
                    (Expr::False, x) | (x, Expr::False) => x,
                    (x, y) => Expr::Or(Box::new(x), Box::new(y)),
                }
            }
            Expr::Xor(a, b) => {
                let a = a.evaluate(m);
                let b = b.evaluate(m);

                match (a, b) {
                    (Expr::True, Expr::True) => Expr::False,
                    (Expr::False, x) | (x, Expr::False) => x,
                    (Expr::True, x) | (x, Expr::True) => Expr::Not(Box::new(x)).simplify(),
                    (x, y) => Expr::Xor(Box::new(x), Box::new(y)),
                }
            }
            Expr::Implication(a, b) => {
                let a = a.evaluate(m);
                let b = b.evaluate(m);

                match (a, b) {
                    (Expr::False, _) | (_, Expr::True) => Expr::True,
                    (Expr::True, a) => a,
                    (x, y) => Expr::Implication(Box::new(x), Box::new(y)),
                }
            }
            Expr::Biconditional(a, b) => {
                let a = a.evaluate(m);
                let b = b.evaluate(m);

                match (a, b) {
                    (Expr::True, x) | (x, Expr::True) => x,
                    (Expr::False, x) | (x, Expr::False) => Expr::Not(Box::new(x)).simplify(),
                    (x, y) => Expr::Biconditional(Box::new(x), Box::new(y)),
                }
            }
            Expr::Not(a) => {
                let a = a.evaluate(m);

                match a {
                    Expr::True => Expr::False,
                    Expr::False => Expr::True,
                    x => Expr::Not(Box::new(x)).simplify(),
                }
            }
        }
    }
}

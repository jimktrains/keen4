use crate::ast;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, Eq)]
pub enum Expr {
    And(Vec<Box<Expr>>),
    Or(Vec<Box<Expr>>),
    Not(Box<Expr>),
    Var(String),
    True,
    False,
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Ord for Expr {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Expr::True, _) => Ordering::Less,
            (_, Expr::True) => Ordering::Greater,
            (Expr::False, _) => Ordering::Less,
            (_, Expr::False) => Ordering::Greater,
            (Expr::Var(a), Expr::Var(b)) => a.cmp(b),
            (Expr::Var(_), Expr::And(b)) | (Expr::Var(_), Expr::Or(b)) => match b.get(0) {
                Some(box b) => self.cmp(b),
                None => Ordering::Less,
            },
            (_, Expr::Var(_)) => Ordering::Greater,
            (Expr::Not(box a), b) => a.cmp(b),
            (a, Expr::Not(b)) => a.cmp(b),
            (Expr::And(a), Expr::And(b)) | (Expr::Or(a), Expr::Or(b)) => {
                for (i, j) in a.iter().zip(b.iter()) {
                    let k = i.cmp(j);
                    if k != Ordering::Equal {
                        return k;
                    }
                }
                Ordering::Equal
            }
            (Expr::And(_), _) => Ordering::Less,
            (_, Expr::And(_)) => Ordering::Greater,
        }
    }
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Expr {
    pub fn terms(&self) -> Vec<String> {
        match self {
            Expr::True | Expr::False => vec![],
            Expr::Var(a) => vec![a.clone()],
            Expr::Not(a) => a.terms(),
            Expr::And(v) | Expr::Or(v) => {
                let mut v = v
                    .iter()
                    .map(|i| i.terms())
                    .flatten()
                    .collect::<Vec<String>>();
                v.sort();
                v.dedup();
                v
            }
        }
    }
    pub fn simplify(self) -> Box<Expr> {
        let me = self.clone();
        match self {
            Expr::True | Expr::False | Expr::Var(_) => Box::new(me),
            Expr::Not(v) => match v {
                // Double Negation Law
                box Expr::Not(w) => w,
                _ => Box::new(me),
            },
            Expr::And(v) => {
                // I'm not a huge fan of all the cloning going on here :(
                let mut v = v
                    .iter()
                    .map(|i| i.clone().simplify())
                    // Identity Law
                    .filter(|i| **i != Expr::True)
                    .collect::<Vec<Box<Expr>>>();
                // Idempotent Law
                v.dedup();
                let mut it = v.iter();
                if let Some(first) = it.next() {
                    if **first == Expr::False {
                        return Box::new(Expr::False);
                    }
                    let mut prev = first.clone();
                    for i in it {
                        // Annulment Law
                        if **i == Expr::False {
                            return Box::new(Expr::False);
                        }
                        // Complement Law
                        if *i == Expr::Not(prev).simplify() {
                            return Box::new(Expr::False);
                        }
                        prev = i.clone();
                    }
                }
                // if we couldn't get element 0, the vec is empty, and
                // we shoudl consider it as a True value.
                else {
                    return Box::new(Expr::True);
                }
                Box::new(Expr::And(v.to_vec()))
            }
            Expr::Or(v) => {
                let mut v = v
                    .iter()
                    .map(|i| i.clone().simplify())
                    // Identity Law
                    .filter(|i| **i != Expr::False)
                    .collect::<Vec<Box<Expr>>>();
                // Idempotent Law
                v.dedup();
                let mut it = v.iter();
                if let Some(first) = it.next() {
                    if **first == Expr::True {
                        return Box::new(Expr::True);
                    }
                    let mut prev = first.clone();
                    for i in it {
                        // Annulment Law
                        if **i == Expr::True {
                            return Box::new(Expr::True);
                        }
                        // Complement Law
                        if *i == Expr::Not(prev).simplify() {
                            return Box::new(Expr::True);
                        }
                        prev = i.clone();
                    }
                }
                // if we couldn't get element 0, the vec is empty, and
                // we shoudl consider it as a False value.
                else {
                    return Box::new(Expr::False);
                }
                Box::new(Expr::Or(v))
            }
        }
    }
    pub fn order_terms(&mut self) {
        match self {
            Expr::True | Expr::False | Expr::Not(_) | Expr::Var(_) => (),
            Expr::And(v) | Expr::Or(v) => {
                for i in v.iter_mut() {
                    i.order_terms();
                }
                v.sort();
            }
        };
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Expr::True => "T".to_string(),
                Expr::False => "F".to_string(),
                Expr::Not(a) => format!(
                    "!{}",
                    match a {
                        box Expr::Var(n) => n.clone(),
                        a => format!("({})", a),
                    }
                ),
                Expr::Var(a) => a.clone(),
                Expr::And(a) => a
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>()
                    .join(""),
                Expr::Or(a) => format!(
                    "({})",
                    a.iter()
                        .map(|f| f.to_string())
                        .collect::<Vec<String>>()
                        .join(") + (")
                ),
            }
        )
    }
}

pub fn from_ast(e: Box<ast::Expr>) -> Result<Box<Expr>, String> {
    match e {
        box ast::Expr::True => Ok(Box::new(Expr::True)),
        box ast::Expr::False => Ok(Box::new(Expr::False)),
        box ast::Expr::Var(n) => Ok(Box::new(Expr::Var(n))),
        // Fix me!
        box ast::Expr::Not(a) => Ok(Box::new(Expr::Not(from_ast(a)?))),
        box ast::Expr::And(a, b) => {
            let a = from_ast(a)?;
            let b = from_ast(b)?;

            let mut r = vec![];
            if let Expr::And(a2) = *a {
                r.extend(a2);
            } else {
                r.push(a);
            }
            if let Expr::And(b2) = *b {
                r.extend(b2);
            } else {
                r.push(b);
            }

            Ok(Box::new(Expr::And(r)))
        }
        box ast::Expr::Or(a, b) => {
            let a = from_ast(a)?;
            let b = from_ast(b)?;

            let mut r = vec![];
            if let Expr::Or(a2) = *a {
                r.extend(a2);
            } else {
                r.push(a);
            }
            if let Expr::Or(b2) = *b {
                r.extend(b2);
            } else {
                r.push(b);
            }

            Ok(Box::new(Expr::Or(r)))
        }
    }
}

use crate::ast;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr {
    And(Vec<Box<Expr>>),
    Or(Vec<Box<Expr>>),
    Not(Box<Expr>),
    Var(String),
    True,
    False,
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
            (Expr::And(a), Expr::And(b)) => {
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
            (Expr::Or(a), Expr::Or(b)) => {
                for (i, j) in a.iter().zip(b.iter()) {
                    let k = i.cmp(j);
                    if k != Ordering::Equal {
                        return k;
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Expr {
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

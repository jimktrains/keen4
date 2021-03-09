use crate::ast;

use std::fmt;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Expr {
    And(Vec<Box<Expr>>),
    Or(Vec<Box<Expr>>),
    Not(Box<Expr>),
    Var(String),
    True,
    False,
}

impl Expr {
    pub fn order_terms(&mut self) {
        match self {
            Expr::True | Expr::False | Expr::Not(_) | Expr::Var(_) => (),
            Expr::And(v) | Expr::Or(v) => {
                /*
                 * Expr::And(v) | Expr::Or(v) => {
                 *           - move occurs because `v` has type `&mut Vec<Box<sast::Expr>>`, which does not implement the `Copy` trait
                 *     for i in v {
                 *              -
                 *              |
                 *              `v` moved due to this implicit call to `.into_iter()`
                 *              help: consider borrowing to avoid moving into the for loop: `&v`
                 *
                 *     v.sort();
                 *     ^ value borrowed here after move
                 *
                 * note: this function takes ownership of the receiver `self`, which moves `v`
                 */
                for i in v {
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

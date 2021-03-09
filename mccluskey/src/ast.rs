use std::fmt;
use std::ops;

#[derive(Debug)]
pub enum Expr {
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Var(String),
    True,
    False,
}

impl Clone for Expr {
    fn clone(&self) -> Self {
        match self {
            Expr::Var(n) => Expr::Var(n.clone()),
            Expr::Not(a) => Expr::Not(a.clone()),
            Expr::And(a, b) => Expr::And(a.clone(), b.clone()),
            Expr::Or(a, b) => Expr::Or(a.clone(), b.clone()),
            Expr::True => Expr::True,
            Expr::False => Expr::False,
        }
    }
}

pub fn var(n: &str) -> Box<Expr> {
    Box::new(Expr::Var(n.to_string()))
}

pub fn etrue() -> Box<Expr> {
    Box::new(Expr::True)
}

pub fn efalse() -> Box<Expr> {
    Box::new(Expr::False)
}

impl ops::Add<Box<Expr>> for Box<Expr> {
    type Output = Box<Expr>;

    fn add(self, _rhs: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Or(self, _rhs))
    }
}

impl ops::Mul<Box<Expr>> for Box<Expr> {
    type Output = Box<Expr>;

    fn mul(self, _rhs: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::And(self, _rhs))
    }
}

impl ops::Add<Box<Expr>> for &Box<Expr> {
    type Output = Box<Expr>;

    fn add(self, _rhs: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Or(self.clone(), _rhs))
    }
}

impl ops::Mul<Box<Expr>> for &Box<Expr> {
    type Output = Box<Expr>;

    fn mul(self, _rhs: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::And(self.clone(), _rhs))
    }
}
impl ops::Add<&Box<Expr>> for &Box<Expr> {
    type Output = Box<Expr>;

    fn add(self, _rhs: &Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Or(self.clone(), _rhs.clone()))
    }
}

impl ops::Mul<&Box<Expr>> for &Box<Expr> {
    type Output = Box<Expr>;

    fn mul(self, _rhs: &Box<Expr>) -> Box<Expr> {
        Box::new(Expr::And(self.clone(), _rhs.clone()))
    }
}

impl ops::Not for &Box<Expr> {
    type Output = Box<Expr>;

    fn not(self) -> Box<Expr> {
        Box::new(Expr::Not(self.clone()))
    }
}

impl ops::Not for Box<Expr> {
    type Output = Box<Expr>;

    fn not(self) -> Box<Expr> {
        Box::new(Expr::Not(self))
    }
}

pub fn distribute(e: Box<Expr>) -> Box<Expr> {
    match e {
        box Expr::Var(_) => e,
        box Expr::True => etrue(),
        box Expr::False => efalse(),
        box Expr::Not(v) => match v {
            box Expr::Var(n) => !Box::new(Expr::Var(n)),
            // Double Negation Law
            box Expr::Not(a) => distribute(a),
            // de Morgan's Theorem
            box Expr::And(a, b) => distribute(!a) + distribute(!b),
            // de Morgan's Theorem
            box Expr::Or(a, b) => distribute(!a) * distribute(!b),
            box Expr::True => efalse(),
            box Expr::False => etrue(),
        },
        // A(B + C) = AB + AC
        box Expr::And(s, t) => {
            match (s, t) {
                (box Expr::Or(b, c), a) | (a, box Expr::Or(b, c)) => {
                    let a1 = a;
                    let a2 = a1.clone();
                    distribute(a1 * b) + distribute(a2 * c)
                }
                // Annulment Law
                (box Expr::False, _) => efalse(),
                // Identity Law
                (box Expr::True, a) | (a, box Expr::True) => a,
                (s, t) => distribute(s) * distribute(t),
            }
        }
        box Expr::Or(s, t) => match (s, t) {
            // This doesn't work Arithmetically, but does in Boolean Algebra.
            // A + BC <=> (A+B)(A+C)
            //        <=> AA + AC + BA + BC
            //        <=> A + AC + BA + BC
            //        <=> A(1 + C + B) + BC
            //        <=> A(1) + BC
            //        <=> A + BC
            (box Expr::Or(b, c), a) | (a, box Expr::Or(b, c)) => {
                let a1 = a;
                let a2 = a1.clone();
                distribute(a1 + b) * distribute(a2 + c)
            }
            // Annulment Law
            (box Expr::True, _) => etrue(),
            // Identity Law
            (box Expr::False, a) | (a, box Expr::False) => a,
            (s, t) => distribute(s) + distribute(t),
        },
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Expr::Var(n) => n.clone(),
                Expr::True => "T".to_string(),
                Expr::False => "F".to_string(),
                Expr::Not(a) => format!(
                    "!{}",
                    match a {
                        box Expr::Var(n) => format!("{}", n),
                        box Expr::Not(a) => format!("!{}", a),
                        box Expr::True => format!("{}", etrue()),
                        box Expr::False => format!("{}", efalse()),
                        box Expr::And(a, b) => format!("({})", a * b),
                        box Expr::Or(a, b) => format!("({})", a + b),
                    }
                ),
                Expr::And(a, b) => match (a, b) {
                    (a, box Expr::Or(s, t)) => format!("({})({})", a, s + t),
                    (box Expr::Or(s, t), b) => format!("({})({})", s + t, b),
                    (a, b) => format!("{}{}", a, b),
                },
                Expr::Or(a, b) => format!("{} + {}", a, b),
            }
        )
    }
}

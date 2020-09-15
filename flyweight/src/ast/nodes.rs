#[derive(Debug, PartialEq, Eq)]
pub enum Expr<'a> {
    Term(&'a str),
    And(Box<Expr<'a>>, Box<Expr<'a>>),
    Or(Box<Expr<'a>>, Box<Expr<'a>>),
    Implication(Box<Expr<'a>>, Box<Expr<'a>>),
    Biconditional(Box<Expr<'a>>, Box<Expr<'a>>),
    Xor(Box<Expr<'a>>, Box<Expr<'a>>),
    Not(Box<Expr<'a>>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Constraints<'a>(pub Vec<&'a str>, pub Vec<Expr<'a>>);

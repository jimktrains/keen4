#[derive(Debug, PartialEq, Eq)]
pub struct Term<'a>(pub &'a str);

#[derive(Debug, PartialEq, Eq)]
pub struct Fact<'a> {
    pub name: &'a str,
    pub terms: Vec<Term<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Var<'a>(pub &'a str);

#[derive(Debug, PartialEq, Eq)]
pub struct PFact<'a> {
    pub name: &'a str,
    pub vars: Vec<Var<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Predicate<'a> {
    pub name: &'a str,
    pub vars: Vec<Var<'a>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExprSide<'a> {
    PFact(PFact<'a>),
    Predicate(Predicate<'a>),
    Expr(Expr<'a>),
    None,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    And,
    Or,
    Implication,
    None,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Expr<'a> {
    pub lhs: Box<ExprSide<'a>>,
    pub rhs: Box<ExprSide<'a>>,
    pub op: Operator,
}

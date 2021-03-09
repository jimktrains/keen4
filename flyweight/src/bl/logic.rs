use crate::ast::nodes::Expr as astExpr;

type TermName<'a> = &'a str;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Expr<'a> {
    Term(TermName<'a>),
    And(Vec<Box<Expr<'a>>>),
    Or(Vec<Box<Expr<'a>>>),
    Xor(Vec<Box<Expr<'a>>>),
    Implication(Box<Expr<'a>>, Box<Expr<'a>>),
    Biconditional(Box<Expr<'a>>, Box<Expr<'a>>),
    Not(Box<Expr<'a>>),
    True,
    False,
}

#[macro_export]
macro_rules! matchtovec {
    ( $x:ident, $a:ident, $b:ident ) => {
        match ($a, $b) {
            (astExpr::$x(box c, box d), astExpr::$x(box e, box f)) => Box::new(Expr::$x(
                (vec![astToLogic(c), astToLogic(d), astToLogic(e), astToLogic(f)]).flatten(),
            )),
            (astExpr::$x(box c, box d), e) | (e, astExpr::$x(box c, box d)) => Box::new(Expr::$x(
                vec![astToLogic(c), astToLogic(d), astToLogic(e)].flatten(),
            )),
            (c, d) => Box::new(Expr::$x([astToLogic(c), astToLogic(d)].flatten())),
        }
    };
}

pub fn astToLogic<'a>(ae: &astExpr<'a>) -> Box<Expr<'a>> {
    match ae {
        astExpr::Term(a) => Box::new(Expr::Term(a)),
        astExpr::And(a, b) => matchtovec!(And, a, b),
        _ => Box::new(Expr::False),
    }
}

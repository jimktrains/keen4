%start ProgramRoot
%left 'PIPE' 'AMPERSAND' 'PLUS'
%nonassoc 'EXCLAMATION'
%%
ProgramRoot -> Result<Program, ()>:
    GlobalsSection ReactiveSection { 
      let globals = $1?;
      let reactives = $2?;
      Ok(Program {globals, reactives})
    }
    ;

VDeclaration -> Result<Variable, ()>:
    Identifier Identifier { 
      let vartype = $1?;
      let name = $2?;
      Ok(Variable {name, vartype})
    }
    ;

GlobalsSection -> Result<Globals, ()>:
     'GLOBALS' 'LBRACK' ListOfVDecs 'RBRACK' { $3 }
 ;

ListOfVDecs -> Result<Globals, ()>:
      VDeclaration { Ok(vec![$1?]) }
    | ListOfVDecs VDeclaration { flatten($1, $2) }
    ;

ReactiveSection -> Result<Reactives, ()>:
      'REACTIVE' 'LBRACK' ListOfRDecs 'RBRACK' { $3 }
    ; 

ListOfRDecs -> Result<Reactives, ()>:
      RDeclaration { Ok(vec![$1?]) }
    | ListOfRDecs RDeclaration { flatten($1, $2) }
    ;

RDeclaration -> Result<Reactive, ()>:
    LogicalExpression 'LBRACK' ListOfAssignments 'RBRACK' {
      let expr = $1?;
      let assignments = $3?;
      Ok(Reactive { expr, assignments })
    }
    ;

ListOfAssignments -> Result<Assignments, ()>:
      Assignment { Ok(vec![$1?]) }
    | ListOfAssignments Assignment { flatten($1, $2) }
    ;

Assignment -> Result<Assignment, ()>:
  Identifier 'ASSIGN' Identifier {
    let variable = $1?;
    let value = $3?;
    Ok(Assignment { variable, value })
  }
  ;

Identifier -> Result<Identifier, ()>:
  'IDENT' {
    let name = $lexer.lexeme_str(&$1.map_err(|_| ())?).to_string();
    Ok(Identifier { name })
  }
  ;

LogicalExpression -> Result<Box<LogicalExpression>, ()>:
    Identifier { Ok(Box::new(LogicalExpression::Identifier($1?))) }
    | 'EXCLAMATION' LogicalExpression { Ok(Box::new(LogicalExpression::LogicalUnaryExpression(LogicalUnaryExpression::Not($2?)))) }
    | Identifier 'LPAREN' Identifier 'RPAREN' { Ok(Box::new(LogicalExpression::LogicalUnaryExpression(LogicalUnaryExpression::Predicate($1?, $3?)))) }
    | LogicalExpression 'PIPE' LogicalExpression { Ok(Box::new(LogicalExpression::LogicalBinaryExpression(LogicalBinaryExpression::Or($1?, $3?)))) }
    | LogicalExpression 'AMPERSAND' LogicalExpression { Ok(Box::new(LogicalExpression::LogicalBinaryExpression(LogicalBinaryExpression::And($1?, $3?)))) }
    | LogicalExpression 'PLUS' LogicalExpression { Ok(Box::new(LogicalExpression::LogicalBinaryExpression(LogicalBinaryExpression::Xor($1?, $3?)))) }
    | 'LPAREN' LogicalExpression 'RPAREN' { Ok(Box::new(LogicalExpression::LogicalExpression($2?))) }
    ;

%%

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Variable {
  name: Identifier,
  vartype: Identifier,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Identifier {
  // TODO: make this &str so that we're not copying the memory.
  // I do not know how to deal with the lifetimes :(
  name: String,
}

#[derive(Debug)]
pub enum LogicalUnaryExpression {
  Not(Box<LogicalExpression>),
  Predicate(Identifier, Identifier),
}

#[derive(Debug)]
pub enum LogicalBinaryExpression {
  And(Box<LogicalExpression>, Box<LogicalExpression>),
  Or(Box<LogicalExpression>, Box<LogicalExpression>),
  Xor(Box<LogicalExpression>, Box<LogicalExpression>),
}

#[derive(Debug)]
pub enum LogicalExpression {
  Identifier(Identifier),
  LogicalExpression(Box<LogicalExpression>),
  LogicalUnaryExpression(LogicalUnaryExpression),
  LogicalBinaryExpression(LogicalBinaryExpression),
}

#[derive(Debug)]
pub struct Assignment {
  variable: Identifier,
  value: Identifier,
}

type Assignments = Vec<Assignment>;

#[derive(Debug)]
pub struct Reactive {
  expr: Box<LogicalExpression>,
  assignments: Assignments,
}

type Globals = Vec<Variable>;
type Reactives = Vec<Reactive>;

#[derive(Debug)]
pub struct Program {
  globals: Globals,
  reactives: Reactives,
}

// Taken from https://softdevteam.github.io/grmtools/master/book/parsing_idioms.html
fn flatten<T>(lhs: Result<Vec<T>, ()>, rhs: Result<T, ()>)
           -> Result<Vec<T>, ()>
{
    let mut flt = lhs?;
    flt.push(rhs?);
    Ok(flt)
}

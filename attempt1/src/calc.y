%start ProgramRoot
%%
ProgramRoot -> Result<Program, ()>:
    GlobalsSection ReactiveSection { 
      let globals = Box::new($1?);
      let reactives = Box::new($2?);
      Ok(Program {globals, reactives})
    }
    ;

VDeclaration -> Variable:
    Identifier Identifier { 
      let vartype = $1;
      let name = $2;
      Variable {name, vartype}
    }
    ;

GlobalsSection -> Result<Globals, ()>:
     'GLOBALS' 'LBRACK' ListOfVDecs 'RBRACK' { $3 }
 ;

ListOfVDecs -> Result<Globals, ()>:
      VDeclaration { Ok(vec![Box::new($1)]) }
    | ListOfVDecs VDeclaration { flatten($1, Ok(Box::new($2))) }
    ;

ReactiveSection -> Result<Reactives, ()>:
      'REACTIVE' 'LBRACK' ListOfRDecs 'RBRACK' { $3 }
    ; 

ListOfRDecs -> Result<Reactives, ()>:
      RDeclaration { Ok(vec![Box::new($1)]) }
    | ListOfRDecs RDeclaration { flatten($1, Ok(Box::new($2))) }
    ;

RDeclaration -> Reactive:
    LogicalExpression 'LBRACK' ListOfAssignments 'RBRACK' {
      let expr = $1;
      let assignments = $3.unwrap();
      Reactive { expr, assignments }
    }
    ;

ListOfAssignments -> Result<Assignments, ()>:
      Assignment { Ok(vec![Box::new($1)]) }
    | ListOfAssignments Assignment { flatten($1, Ok(Box::new($2))) }
    ;

Assignment -> Assignment:
  Identifier 'ASSIGN' Identifier {
    let variable = $1;
    let value = $3;
    Assignment { variable, value }
  }
  ;

Identifier -> Box<Identifier>:
  'IDENT' {
    let name = $1.unwrap();
    Box::new(Identifier { name })
  }
  ;

LogicalExpression -> LogicalExpression:
      "EXCLAMATION" LogicalExpression { 
      LogicalExpression::LogicalUnaryExpression(Box::new(LogicalUnaryExpression::Not(Box::new($2)))) 
    }
    | Identifier 'LPAREN' Identifier 'RPAREN' { 
      LogicalExpression::LogicalUnaryExpression(Box::new(LogicalUnaryExpression::Predicate($1, $3)))
    }
    ;
%%

#[derive(Debug)]
pub struct Variable {
  name: Box<Identifier>,
  vartype: Box<Identifier>,
}

type StorageT = u32;

#[derive(Debug)]
pub struct Identifier {
  name: Lexeme<StorageT>,
}

#[derive(Debug)]
pub enum LogicalUnaryExpression {
  Not(Box<LogicalExpression>),
  Predicate(Box<Identifier>, Box<Identifier>),
}

#[derive(Debug)]
pub enum LogicalBinaryExpression {
  And(Box<LogicalExpression>, Box<LogicalExpression>),
  Or(Box<LogicalExpression>, Box<LogicalExpression>),
  Xor(Box<LogicalExpression>, Box<LogicalExpression>),
}

#[derive(Debug)]
pub enum LogicalExpression {
  Identifier(Box<Identifier>),
  Variable(Box<Variable>),
  LogicalExpression(Box<LogicalExpression>),
  LogicalUnaryExpression(Box<LogicalUnaryExpression>),
  LogicalBinaryExpression(Box<LogicalBinaryExpression>),
}

#[derive(Debug)]
pub struct Assignment {
  variable: Box<Identifier>,
  value: Box<Identifier>,
}

type Assignments = Vec<Box<Assignment>>;

#[derive(Debug)]
pub struct Reactive {
  expr: LogicalExpression,
  assignments: Assignments,
}

type Globals = Vec<Box<Variable>>;
type Reactives = Vec<Box<Reactive>>;

#[derive(Debug)]
pub struct Program {
  globals: Box<Globals>,
  reactives: Box<Reactives>,
}

// Taken from https://softdevteam.github.io/grmtools/master/book/parsing_idioms.html
fn flatten<T>(lhs: Result<Vec<T>, ()>, rhs: Result<T, ()>)
           -> Result<Vec<T>, ()>
{
    let mut flt = lhs?;
    flt.push(rhs?);
    Ok(flt)
}

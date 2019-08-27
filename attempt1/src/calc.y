%start ProgramRoot
%%
ProgramRoot -> Result<Program, ()>:
    GlobalsSection ReactiveSection { 
      let globals = $1?;
      let reactives = $2?;
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
      VDeclaration { Ok(vec![$1]) }
    | ListOfVDecs VDeclaration { flatten($1, Ok($2)) }
    ;

ReactiveSection -> Result<Reactives, ()>:
      'REACTIVE' 'LBRACK' ListOfRDecs 'RBRACK' { $3 }
    ; 

ListOfRDecs -> Result<Reactives, ()>:
      RDeclaration { Ok(vec![$1]) }
    | ListOfRDecs RDeclaration { flatten($1, Ok($2)) }
    ;

RDeclaration -> Reactive:
    LogicalExpression 'LBRACK' ListOfAssignments 'RBRACK' {
      let expr = $1;
      let assignments = $3.unwrap();
      Reactive { expr, assignments }
    }
    ;

ListOfAssignments -> Result<Assignments, ()>:
      Assignment { Ok(vec![$1]) }
    | ListOfAssignments Assignment { flatten($1, Ok($2)) }
    ;

Assignment -> Assignment:
  Identifier 'ASSIGN' Identifier {
    let variable = $1;
    let value = $3;
    Assignment { variable, value }
  }
  ;

Identifier -> Identifier:
  'IDENT' {
    let name = $1.unwrap();
    Identifier { name }
  }
  ;

LogicalExpression -> LogicalExpression:
      'EXCLAMATION' LogicalExpression { 
      LogicalExpression::LogicalUnaryExpression(LogicalUnaryExpression::Not(Box::new($2))) 
    }
    | Identifier 'LPAREN' Identifier 'RPAREN' { 
      LogicalExpression::LogicalUnaryExpression(LogicalUnaryExpression::Predicate($1, $3))
    }
    | ParenLogicalExpression 'PIPE' ParenLogicalExpression { 
      LogicalExpression::LogicalBinaryExpression(LogicalBinaryExpression::Or($1, $3))
    }
    | ParenLogicalExpression 'AMPERSAND' ParenLogicalExpression { 
      LogicalExpression::LogicalBinaryExpression(LogicalBinaryExpression::And($1, $3))
    }
    | ParenLogicalExpression 'PLUS' ParenLogicalExpression { 
      LogicalExpression::LogicalBinaryExpression(LogicalBinaryExpression::Xor($1, $3))
    }
    | ParenLogicalExpression { LogicalExpression::LogicalExpression($1) }
    ;

ParenLogicalExpression -> Box<LogicalExpression>:
    'LPAREN' LogicalExpression 'RPAREN' { Box::new($2) }
    ;
%%


#[derive(Debug)]
pub struct Variable {
  name: Identifier,
  vartype: Identifier,
}

type StorageT = u32;

#[derive(Debug)]
pub struct Identifier {
  name: Lexeme<StorageT>,
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
  expr: LogicalExpression,
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

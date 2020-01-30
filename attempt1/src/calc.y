%start ProgramRoot
%left 'PIPE' 'AMPERSAND' 'PLUS'
%nonassoc 'EXCLAMATION'
%%
ProgramRoot -> Result<Program, ()>:
    EnumSection GlobalsSection ReactiveSection { 
      let enums = $1?;
      let globals = $2?;
      let reactives = $3?;
      Ok(Program {enums, globals, reactives})
    }
    ;

GlobalsSection -> Result<Globals, ()>:
     'GLOBALS' 'LBRACE' ListOfAssignments 'RBRACE' { $3 }
 ;

ReactiveSection -> Result<Reactives, ()>:
      'REACTIVE' 'LBRACE' ListOfRDecs 'RBRACE' { $3 }
    ; 

ListOfRDecs -> Result<Reactives, ()>:
      RDeclaration { Ok(vec![$1?]) }
    | ListOfRDecs RDeclaration { flatten($1, $2) }
    ;

RDeclaration -> Result<Reactive, ()>:
    LogicalExpression 'LBRACE' ListOfAssignments 'RBRACE' {
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
    Ok(name)
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

EnumSection -> Result<Enums, ()>:
      'ENUMS' 'LBRACE' ListOfEDecs 'RBRACE' { $3 }
    ; 

ListOfEDecs -> Result<Enums, ()>:
      EnumDeclaration { Ok(vec![$1?]) }
    | ListOfEDecs EnumDeclaration { flatten($1, $2) }
    ;

EnumDeclaration -> Result<Enum, ()>:
    Identifier 'LBRACE' ListOfIdentifiers 'RBRACE' {
      let name = $1?;
      let values = $3?;
      Ok(Enum { name, values })
    }
    ;

ListOfIdentifiers -> Result<Identifiers, ()>:
      Identifier { Ok(vec![$1?]) }
    | ListOfIdentifiers Identifier { flatten($1, $2) }
    ;
%%

type Identifier = String;
type Identifiers = Vec<Identifier>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Enum {
  pub name: Identifier,
  pub values: Identifiers,
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

type Globals = Assignments;
type Reactives = Vec<Reactive>;
type Enums = Vec<Enum>;

#[derive(Debug)]
pub struct Program {
  pub enums: Enums,
  pub globals: Globals,
  pub reactives: Reactives,
}

// Taken from https://softdevteam.github.io/grmtools/master/book/parsing_idioms.html
fn flatten<T>(lhs: Result<Vec<T>, ()>, rhs: Result<T, ()>)
           -> Result<Vec<T>, ()>
{
    let mut flt = lhs?;
    flt.push(rhs?);
    Ok(flt)
}

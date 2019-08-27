#[derive(Debug)]
struct Variable {
  name: Box<Identifier>,
  vartype: Box<Identifier>,
}

type StorageT = u32;

#[derive(Debug)]
struct Identifier {
  name: Lexeme<StorageT>,
}

#[derive(Debug)]
enum LogicalUnaryExpression {
  Not(Box<LogicalExpression>),
}

#[derive(Debug)]
enum LogicalBinaryExpression {
  And(Box<LogicalExpression>, Box<LogicalExpression>),
  Or(Box<LogicalExpression>, Box<LogicalExpression>),
  Xor(Box<LogicalExpression>, Box<LogicalExpression>),
}

#[derive(Debug)]
enum LogicalExpression {
  Identifier(Box<Identifier>),
  Variable(Box<Variable>),
  LogicalExpression(Box<LogicalExpression>),
  LogicalUnaryExpression,
  LogicalBinaryExpression,
}

#[derive(Debug)]
struct Assignment {
  variable: Box<Variable>,
  value: Box<Identifier>,
}

type Assignments = Vec<Box<Assignment>>;

#[derive(Debug)]
struct Reactive {
  expr: LogicalExpression,
  assignments: Assignments,
}

type Globals = Vec<Box<Variable>>;
type Reactives = Vec<Box<Reactive>>;

#[derive(Debug)]
struct Program {
  globals: Box<Globals>,
  //reactive: Reactives,
}


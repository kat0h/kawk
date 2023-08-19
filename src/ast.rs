pub type Program = Vec<Item>;

#[derive(Debug, PartialEq)]
pub struct Item {
    pub pattern: Pattern,
    pub action: Action,
}

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Always,
    Begin,
    End,
    Expression(Expression),
}

pub type Action = Vec<Statement>;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Value(Value),
    BinaryOp {
        op: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    GetField(Box<Expression>),
    Name(String),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Print(Vec<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Num(f64),
    Str(String),
    None,
}

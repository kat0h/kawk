pub type Program = Vec<Item>;

#[derive(Debug, PartialEq)]
pub enum Item {
    PatternAction(PatternAction),
    Function(Function)
}

#[derive(Debug, PartialEq)]
pub struct PatternAction {
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
pub struct Function {
    // 引数リスト
    pub args: Vec<String>,
    pub action: Action
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Value(Value),
    BinaryOp {
        op: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    GetField(Box<Expression>),
    LValue(LValue),
    Assign {
        lval: LValue,
        expr: Box<Expression>,
    },
    CallIFunc {
        name: String,
        args: Vec<Expression>
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LValue {
    Name(String),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,              // +
    Sub,              // -
    Mul,              // *
    Div,              // /
    Pow,              // ^
    Mod,              // %
    Cat,              // string concat
    And,              // &&
    Or,               // ||
    LessThan,         // <
    LessEqualThan,    // <=
    NotEqual,         // !=
    Equal,            // ==
    GreaterThan,      // >
    GreaterEqualThan, // >=
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Print(Vec<Expression>),
    Expression(Expression),
    While {
        exp: Expression,
        stat: Action
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Num(f64),
    Str(String),
    None,
}

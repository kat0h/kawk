pub type Program = Vec<Item>;

#[derive(Debug, PartialEq)]
pub enum Item {
    PatternAction(PatternAction),
    Function(Function)
}

#[derive(Debug, PartialEq)]
pub struct PatternAction {
    pub pattern: Pattern,
    pub action: Statement,
}

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Always,
    Begin,
    End,
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    // 引数リスト
    pub args: Vec<String>,
    pub action: Statement
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
    },
    CallUserFunc {
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
    Action(Vec<Statement>),
    Print(Vec<Expression>),
    Expression(Expression),
    While {
        exp: Expression,
        stat: Box<Statement>
    },
    If {
        cond: Expression,
        stat: Box<Statement>,
    },
    IfElse {
        cond: Expression,
        stat: Box<Statement>,
        els: Box<Statement>
    },
    Return(Expression)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Num(f64),
    Str(String),
    None,
}

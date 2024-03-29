pub type Program = Vec<Item>;

#[derive(Debug, PartialEq)]
pub enum Item {
    PatternAction(PatternAction),
    Function(Function),
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
    pub action: Statement,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Value(Value),
    BinaryOp {
        op: BOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    IncDec {
        op: IncDecType,
        lval: Box<LValue>,
    },
    GetField(Box<Expression>),
    LValue(LValue),
    Assign {
        lval: LValue,
        expr: Box<Expression>,
    },
    CallIFunc {
        name: String,
        args: Vec<Expression>,
    },
    CallUserFunc {
        name: String,
        args: Vec<Expression>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum LValue {
    Name(String),
    Array {
        name: String,
        expr_list: Vec<Expression>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum BOperator {
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

#[derive(Debug, PartialEq, Clone)]
pub enum IncDecType {
    PostInc, // lvalue++
    PostDec, // lvalue--
    PreInc,  // ++lvalue
    PreDec,  // --lvalue
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Action(Vec<Statement>),
    Print(Vec<Expression>),
    Printf {
        fmt: Box<Expression>,
        args: Vec<Expression>,
    },
    Expression(Expression),
    While {
        exp: Expression,
        stat: Box<Statement>,
    },
    For {
        init: Box<Statement>,
        test: Expression,
        updt: Box<Statement>,
        stat: Box<Statement>,
    },
    If {
        cond: Expression,
        stat: Box<Statement>,
    },
    IfElse {
        cond: Expression,
        stat: Box<Statement>,
        els: Box<Statement>,
    },
    Return(Expression),
    Break,
    Continue,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Num(f64),
    Str(String),
    None,
}

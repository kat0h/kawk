use crate::ast::Value;
use crate::parser::awk::number;

impl Value {
    // Value -> f64 / String
    pub fn to_str(&self) -> String {
        match self {
            Value::Num(n) => n.to_string(),
            Value::Str(s) => s.clone(),
            Value::None => "".to_string(),
        }
    }
    pub fn to_float(&self) -> f64 {
        match self {
            Value::Num(n) => *n,
            Value::Str(s) => number(s).unwrap_or(0.0),
            Value::None => 0.0,
        }
    }
    pub fn is_true(&self) -> bool {
        match self {
            Value::Num(n) => *n == 1.0,
            Value::Str(s) => !s.is_empty(),
            Value::None => false,
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
enum Operator {
    LT,  // <
    LET, // <=
    NE,  // !=
    EQ,  // ==
    GT,  // >
    GET, // >=
}

impl Value {
    // TODO: 多倍長整数演算
    pub fn add(&self, val: &Value) -> Value {
        Value::Num(self.to_float() + val.to_float())
    }
    pub fn sub(&self, val: &Value) -> Value {
        Value::Num(self.to_float() - val.to_float())
    }
    pub fn mul(&self, val: &Value) -> Value {
        Value::Num(self.to_float() * val.to_float())
    }
    pub fn div(&self, val: &Value) -> Value {
        // エラーをチェック
        Value::Num(self.to_float() / val.to_float())
    }
    pub fn module(&self, val: &Value) -> Value {
        Value::Num(self.to_float() % val.to_float())
    }
    pub fn pow(&self, val: &Value) -> Value {
        Value::Num(self.to_float().powf(val.to_float()))
    }
    pub fn not(&self) -> Value {
        Value::Num(if self.is_true() { 0.0 } else { 1.0 })
    }
    pub fn plus(&self) -> Value {
        Value::Num(self.to_float())
    }
    pub fn minus(&self) -> Value {
        Value::Num(self.to_float() * -1.0)
    }
    pub fn concat(&self, val: &Value) -> Value {
        Value::Str(self.to_str() + &val.to_str())
    }
    pub fn and(&self, val: &Value) -> Value {
        // 短絡評価
        if !self.is_true() {
            return Value::Num(0.0);
        };
        if !val.is_true() {
            return Value::Num(0.0);
        };
        Value::Num(1.0)
    }
    pub fn or(&self, val: &Value) -> Value {
        // 短絡評価
        if self.is_true() {
            return Value::Num(1.0);
        };
        if val.is_true() {
            return Value::Num(1.0);
        };
        Value::Num(0.0)
    }
    // 比較のルール
    // 両方が数字 -> 数値として比較する
    // それ以外 -> 文字列に変換して比較する
    // POSIXの記述は誤りです
    //
    fn compbase(&self, val: &Value, op: Operator) -> Value {
        let (left, right) = (self, val);
        Value::Num(
            if match (left, right) {
                (Value::Num(left), Value::Num(right)) => match op {
                    Operator::LT => left < right,
                    Operator::LET => left <= right,
                    Operator::NE => left != right,
                    Operator::EQ => left == right,
                    Operator::GT => left > right,
                    Operator::GET => left <= right,
                },
                (_, _) => {
                    let (left, right) = (left.to_str(), right.to_str());
                    match op {
                        Operator::LT => left < right,
                        Operator::LET => left <= right,
                        Operator::NE => left != right,
                        Operator::EQ => left == right,
                        Operator::GT => left > right,
                        Operator::GET => left <= right,
                    }
                }
            } {
                1.0
            } else {
                0.0
            },
        )
    }
    // <
    pub fn lessthan(&self, val: &Value) -> Value {
        self.compbase(val, Operator::LT)
    }
    pub fn lessequalthan(&self, val: &Value) -> Value {
        self.compbase(val, Operator::LET)
    }
    pub fn notequal(&self, val: &Value) -> Value {
        self.compbase(val, Operator::NE)
    }
    pub fn equal(&self, val: &Value) -> Value {
        self.compbase(val, Operator::EQ)
    }
    pub fn greaterthan(&self, val: &Value) -> Value {
        self.compbase(val, Operator::GT)
    }
    pub fn greaterequalthan(&self, val: &Value) -> Value {
        self.compbase(val, Operator::GET)
    }
}

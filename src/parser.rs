use crate::ast;
use crate::ifunc::get_index_from_name;

pub fn parse(prog: &str) -> Result<Vec<ast::Item>, peg::error::ParseError<peg::str::LineCol>> {
    awk::prog(prog)
}

peg::parser! {
    pub grammar awk() for str {
        // BEGIN { print(123) } のような一連のプログラム
        // 改行文字は含まないので予め消してから
        pub rule prog() -> ast::Program
            = __ i:(
                item() ** (_ [';' | '\n']* _)
            ) __ { i }

        // itemはpattern BEGIN とaction {} の複合
        rule item() -> ast::Item
            = pattern:pattern() _ action:action() { ast::Item::PatternAction(ast::PatternAction { pattern, action }) }

        // BEGIN / END / 条件式など
        rule pattern() -> ast::Pattern
            = precedence! {
                "BEGIN" { ast::Pattern::Begin }
                "END" { ast::Pattern::End }
                e:expression() { ast::Pattern::Expression(e) }
                "" { ast::Pattern::Always }
            }

        // action は {} で囲われていて，それぞれの文は ; で区切られている
        rule action() -> ast::Action
            = "{" __ a:(statement() ** (_ [';' | '\n']* _)) __ "}" { a }

        // print文 POSIXでは括弧の前に空白を置くことが許可される
        rule statement() -> ast::Statement
            = precedence! {
                // 式
                e:expression() { ast::Statement::Expression(e) }
                // 括弧ありprint文
                "print" _ "(" _ a:(expression() ** (_ "," _)) _ ")" {
                    ast::Statement::Print(a)
                }
                // 括弧なしprint文
                "print" _ a:(expression() ** (_ "," _)) {
                    ast::Statement::Print(a)
                }
                // while文
                "while" _ "(" e:expression() _ ")" _ s:action() {
                    ast::Statement::While {
                        exp: e,
                        stat: s
                    }
                }
            }

        // 式
        rule expression() -> ast::Expression
            = precedence! {
                l:lvalue() _ "=" _ e:@ { ast::Expression::Assign { lval: l, expr: Box::new(e)} }
                l:lvalue() _ "+=" _ e:@ {
                    ast::Expression::Assign {
                        lval: l.clone(), expr: Box::new(ast::Expression::BinaryOp {
                            op: ast::Operator::Add,
                            left: Box::new(ast::Expression::LValue(l)),
                            right: Box::new(e),
                        })
                    }
                }
                l:lvalue() _ "-=" _ e:@ {
                    ast::Expression::Assign {
                        lval: l.clone(), expr: Box::new(ast::Expression::BinaryOp {
                            op: ast::Operator::Sub,
                            left: Box::new(ast::Expression::LValue(l)),
                            right: Box::new(e),
                        })
                    }
                }
                l:lvalue() _ "*=" _ e:@ {
                    ast::Expression::Assign {
                        lval: l.clone(), expr: Box::new(ast::Expression::BinaryOp {
                            op: ast::Operator::Mul,
                            left: Box::new(ast::Expression::LValue(l)),
                            right: Box::new(e),
                        })
                    }
                }
                l:lvalue() _ "/=" _ e:@ {
                    ast::Expression::Assign {
                        lval: l.clone(), expr: Box::new(ast::Expression::BinaryOp {
                            op: ast::Operator::Div,
                            left: Box::new(ast::Expression::LValue(l)),
                            right: Box::new(e),
                        })
                    }
                }
                l:lvalue() _ "%=" _ e:@ {
                    ast::Expression::Assign {
                        lval: l.clone(), expr: Box::new(ast::Expression::BinaryOp {
                            op: ast::Operator::Mod,
                            left: Box::new(ast::Expression::LValue(l)),
                            right: Box::new(e),
                        })
                    }
                }
                l:lvalue() _ "^=" _ e:@ {
                    ast::Expression::Assign {
                        lval: l.clone(), expr: Box::new(ast::Expression::BinaryOp {
                            op: ast::Operator::Pow,
                            left: Box::new(ast::Expression::LValue(l)),
                            right: Box::new(e),
                        })
                    }
                }
                --
                l:(@) _ "||" _ r:@ { ast::Expression::BinaryOp { op: ast::Operator::Or, left: Box::new(l), right: Box::new(r), } }
                --
                l:(@) _ "&&" _ r:@ { ast::Expression::BinaryOp { op: ast::Operator::And, left: Box::new(l), right: Box::new(r), } }
                --
                l:(@) _ "<" _ r:@ { ast::Expression::BinaryOp { op: ast::Operator::LessThan, left: Box::new(l), right: Box::new(r), } }
                l:(@) _ "<=" _ r:@ { ast::Expression::BinaryOp { op: ast::Operator::LessEqualThan, left: Box::new(l), right: Box::new(r), } }
                l:(@) _ "!=" _ r:@ { ast::Expression::BinaryOp { op: ast::Operator::NotEqual, left: Box::new(l), right: Box::new(r), } }
                l:(@) _ "==" _ r:@ { ast::Expression::BinaryOp { op: ast::Operator::Equal, left: Box::new(l), right: Box::new(r), } }
                l:(@) _ ">" _ r:@ { ast::Expression::BinaryOp { op: ast::Operator::GreaterThan, left: Box::new(l), right: Box::new(r), } }
                l:(@) _ ">=" _ r:@ { ast::Expression::BinaryOp { op: ast::Operator::GreaterEqualThan, left: Box::new(l), right: Box::new(r), } }
                --
                l:(@) _ r:@ { ast::Expression::BinaryOp { op: ast::Operator::Cat, left: Box::new(l), right: Box::new(r), } }
                --
                l:(@) _ "+" _ r:@ { ast::Expression::BinaryOp { op: ast::Operator::Add, left: Box::new(l), right: Box::new(r), } }
                l:(@) _ "-" _ r:@ { ast::Expression::BinaryOp { op: ast::Operator::Sub, left: Box::new(l), right: Box::new(r), } }
                --
                l:(@) _ "*" _ r:@ { ast::Expression::BinaryOp { op: ast::Operator::Mul, left: Box::new(l), right: Box::new(r), } }
                l:(@) _ "/" _ r:@ { ast::Expression::BinaryOp { op: ast::Operator::Div, left: Box::new(l), right: Box::new(r), } }
                l:(@) _ "%" _ r:@ { ast::Expression::BinaryOp { op: ast::Operator::Mod, left: Box::new(l), right: Box::new(r), } }
                --
                l:@ _ "^" _ r:(@) { ast::Expression::BinaryOp { op: ast::Operator::Pow, left: Box::new(l), right: Box::new(r), } }
                --
                "$" _ e:@ { ast::Expression::GetField(Box::new(e)) }
                --
                n:number() { ast::Expression::Value(ast::Value::Num(n)) }
                n:string() { ast::Expression::Value(ast::Value::Str(n)) }
                e:internal_func_call() { e }
                n:lvalue() { ast::Expression::LValue(n) }
                "(" _ e:expression() _ ")" { e }
            }

        rule internal_func_call() -> ast::Expression
            = n:name() "(" a:(expression() ** (_ "," _)) ")" {?
                if get_index_from_name(&n).is_some() {
                    Ok(ast::Expression::CallIFunc {
                        name: n,
                        args: a
                    })
                } else {
                    Err("Not a internal command")
                }
            }

        rule name() -> String
            = n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '_' | '0'..='9']*) {?
                if is_awk_reserved_name(n) {
                    Err("Reserved name")
                } else {
                    Ok(n.to_string())
                }
            }

        rule lvalue() -> ast::LValue
            = l:name() { ast::LValue::Name(l) }

        // 数字 (もっと詳しくパースできるように)
        pub rule number() -> f64
            = n:$(['0'..='9']+) {? n.parse::<f64>().or(Err("i64")) }

        // 文字列
        rule string() -> String
            = "\"" s:$([^'"']*) "\"" { s.to_string() }

        // 空白文字を処理
        rule _() = [' ' | '\t']*
        rule __() = [' ' | '\t' | '\n']*
    }
}

/// 名前がAWKの予約語に含まれているかを判定
pub fn is_awk_reserved_name(name: &str) -> bool {
    let list = [
        "BEGIN", "delete", "END", "function", "in", "printf", "break", "do", "exit", "getline",
        "next", "return", "continue", "else", "for", "if", "print", "while",
    ];
    list.iter().any(|n| n == &name)
}

#[test]
fn test_parser() {
    let prg = " BEGIN { print( 123 + 333 , 456 ) } ";
    let expect = vec![ast::Item::PatternAction(ast::PatternAction {
        pattern: ast::Pattern::Begin,
        action: vec![ast::Statement::Print(vec![
            ast::Expression::BinaryOp {
                op: ast::Operator::Add,
                left: Box::new(ast::Expression::Value(ast::Value::Num(123.0))),
                right: Box::new(ast::Expression::Value(ast::Value::Num(333.0))),
            },
            ast::Expression::Value(ast::Value::Num(456.0)),
        ])],
    })];
    let actual = awk::prog(prg).unwrap();

    assert_eq!(expect, actual);
}

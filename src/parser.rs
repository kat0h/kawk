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
                (patternaction() / function()) ** comment_cr()
            ) __ { i }

        rule comment_cr()
            = (_ (";" / ("#" [^ '\n']* ) "\n" / "\n")* _)

        // patternactionはpattern BEGIN とaction {} の複合
        rule patternaction() -> ast::Item
            = pattern:pattern() _ action:action() { ast::Item::PatternAction(ast::PatternAction { pattern, action }) }

        rule function() -> ast::Item
            // NOTE:: 内蔵関数の書き換えはどうする？
            // = "function" _ name:name() "(" _ args:(name() ** (_ "," _))  _ ")" __ action:action() {
            //    ast::Item::Function(ast::Function { name, args, action })
            // }
            = "function" _ name:name() "(" _ args:name() ** (_ "," _) ")" __ action:action() {
               ast::Item::Function(ast::Function { name, args, action })
            }

        // BEGIN / END / 条件式など
        rule pattern() -> ast::Pattern
            = precedence! {
                "BEGIN" { ast::Pattern::Begin }
                "END" { ast::Pattern::End }
                e:expression() { ast::Pattern::Expression(e) }
                "" { ast::Pattern::Always }
            }

        // action は {} で囲われていて，それぞれの文は ; で区切られている
        rule action() -> ast::Statement
            = "{" __ a:(statement() ** comment_cr()) __ "}" { ast::Statement::Action(a) }

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
                        stat: Box::new(s)
                    }
                }
                // if else
                "if" _ "(" e:expression() _ ")" __ s:statement() __ "else" __ es:statement() {
                    ast::Statement::IfElse {
                        cond: e,
                        stat: Box::new(s),
                        els: Box::new(es)
                    }
                }
                // if
                "if" _ "(" e:expression() _ ")" __ s:statement() {
                    ast::Statement::If {
                        cond: e,
                        stat: Box::new(s),
                    }
                }
                // return文
                "return" _ e:expression() {
                    ast::Statement::Return(e)
                }
                "return" {
                    ast::Statement::Return(ast::Expression::Value(ast::Value::None))
                }
                a:action() { a }
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
                e:func_call() { e }
                n:lvalue() { ast::Expression::LValue(n) }
                "(" _ e:expression() _ ")" { e }
            }

        rule func_call() -> ast::Expression
            = name:name() "(" args:(expression() ** (_ "," _)) ")" {
                if get_index_from_name(&name).is_some() {
                    // 内蔵関数とユーザー関数は区別される
                    ast::Expression::CallIFunc { name, args }
                } else {
                    // ast::Expression::CallUserFunc { name, args }
                    ast::Expression::CallUserFunc { name, args }
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
            = l:name() "[" e:expression() "]" { ast::LValue::Array { name: l, expr_list: vec![e]} }
            / l:name() { ast::LValue::Name(l) }

        // 数字 (もっと詳しくパースできるように)
        pub rule number() -> f64
            = n:$(['0'..='9']+) {? n.parse::<f64>().or(Err("i64")) }

        // 文字列
        rule string() -> String
            = "\"" s:$([^'"']*) "\"" { s.to_string() }

        // 空白文字を処理
        rule _() = [' ' | '\t']*
        rule __() = (" " / "\t" / ("#" [^ '\n']* ) / "\n")*
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
        action: ast::Statement::Action(vec![ast::Statement::Print(vec![
            ast::Expression::BinaryOp {
                op: ast::Operator::Add,
                left: Box::new(ast::Expression::Value(ast::Value::Num(123.0))),
                right: Box::new(ast::Expression::Value(ast::Value::Num(333.0))),
            },
            ast::Expression::Value(ast::Value::Num(456.0)),
        ])]),
    })];
    let actual = awk::prog(prg).unwrap();

    assert_eq!(expect, actual);
}

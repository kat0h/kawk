use crate::ast;

pub fn parse(prog: &str) {}

peg::parser! {
    pub grammar awk() for str {
        pub rule prog() -> ast::Program
            = i:(item() ** ";") { i }

        rule item() -> ast::Item
            = pattern:pattern() action:action() { ast::Item { pattern, action } }

        rule pattern() -> ast::Pattern
            = precedence! {
                "BEGIN" { ast::Pattern::Begin }
                "END" { ast::Pattern::End }
            }

        rule action() -> ast::Action
            = "{" a:(statement() ** ";") "}" { a }

        rule statement() -> ast::Statement
            = precedence! {
                "print(" a:(expression() ** ",") ")" {
                    ast::Statement::Print(a)
                }
            }

        rule expression() -> ast::Expression
            = precedence! {
                n:number() { ast::Expression::Value(ast::Value::Num(n)) }
            }

        pub rule number() -> f64
            = n:$(['0'..='9']+) {? n.parse::<f64>().or(Err("i64")) }

    }
}

#[test]
fn test_parser() {
    let prg = "BEGIN{print(123)}";
    let expect = vec![ast::Item {
        pattern: ast::Pattern::Begin,
        action: vec![ast::Statement::Print(vec![ast::Expression::Value(ast::Value::Num(123.0))])],
    }];
    let actual = awk::prog(prg).unwrap();

    assert_eq!(expect, actual);
}

use crate::ast;

pub fn parse(prog: &str) -> Result<Vec<ast::Item>, peg::error::ParseError<peg::str::LineCol>> {
    awk::prog(prog)
}

peg::parser! {
    pub grammar awk() for str {
        // BEGIN { print(123) } のような一連のプログラム
        // 改行文字は含まないので予め消してから
        pub rule prog() -> ast::Program
            = _ i:(item() ** (_ ";" _)) _ { i }

        // itemはpattern BEGIN とaction {} の複合
        rule item() -> ast::Item
            = pattern:pattern() _ action:action() { ast::Item { pattern, action } }

        // BEGIN / END / 条件式など
        rule pattern() -> ast::Pattern
            = precedence! {
                "BEGIN" { ast::Pattern::Begin }
                "END" { ast::Pattern::End }
            }

        // action は {} で囲われていて，それぞれの文は ; で区切られている
        rule action() -> ast::Action
            = "{" _ a:(statement() ** (_ ";" _)) _ "}" { a }

        // print文 POSIXでは括弧の前に空白を置くことが許可される
        rule statement() -> ast::Statement
            = precedence! {
                "print(" _ a:(expression() ** (_ "," _)) _ ")" {
                    ast::Statement::Print(a)
                }
            }

        // 式
        rule expression() -> ast::Expression
            = precedence! {
                n:number() { ast::Expression::Value(ast::Value::Num(n)) }
            }

        // 数字 (もっと詳しくパースできるように)
        pub rule number() -> f64
            = n:$(['0'..='9']+) {? n.parse::<f64>().or(Err("i64")) }

        // 空白文字を処理
        rule _() = [' ' | '\t']*
    }
}

#[test]
fn test_parser() {
    let prg = " BEGIN { print( 123 , 456 ) } ";
    let expect = vec![ast::Item {
        pattern: ast::Pattern::Begin,
        action: vec![ast::Statement::Print(vec![
            ast::Expression::Value(ast::Value::Num(123.0)),
            ast::Expression::Value(ast::Value::Num(456.0)),
        ])],
    }];
    let actual = awk::prog(prg).unwrap();

    assert_eq!(expect, actual);
}

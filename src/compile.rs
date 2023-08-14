use crate::ast;
use crate::vm::Opcode;

pub type VMProgram = Vec<Opcode>;

/*
 * メモ：
 * AWKのプログラムは次のような形をしている
 *   BEGIN{ print(123) };
 *
 *   BEGIN -> Pattern
 *   {} -> Action
 *   print(123) -> Statement
 *   123 -> Expression
 */

pub fn compile(ast: &ast::Program) -> VMProgram {
    // そのうちはコンパイルエラーをResultで返すようにしたい
    // (エラーは呼び出し側で処理すべきなので)
    let mut vmprogram: VMProgram = vec![];

    // BEGINパターンを探しコンパイル
    compile_all_begin_pattern(ast, &mut vmprogram);
    
    // ENDパターンを探してコンパイル
    compile_all_end_pattern(ast, &mut vmprogram);

    // 最後にENDを追加 (そうしないとVMが終了しない)
    vmprogram.push(Opcode::End);

    vmprogram
}

// 全てのBEGINパターンを探してコンパイルする
fn compile_all_begin_pattern(ast: &ast::Program, vmprogram: &mut VMProgram) {
    // fin BEGIN pattern
    let items = ast
        .iter()
        .filter(|i| matches!(i.pattern, ast::Pattern::Begin))
        .collect::<Vec<_>>();

    for item in items.into_iter() {
        // actionの列をコンパイル
        compile_action(&item.action, vmprogram);
    }
}

fn compile_all_end_pattern(ast: &ast::Program, vmprogram: &mut VMProgram) {
    // fin BEGIN pattern
    let items = ast
        .iter()
        .filter(|i| matches!(i.pattern, ast::Pattern::End))
        .collect::<Vec<_>>();

    for item in items.into_iter() {
        // actionの列をコンパイル
        compile_action(&item.action, vmprogram);
    }
}

// action ::: {}で囲われた一連のコード
fn compile_action(action: &ast::Action, vmprogram: &mut VMProgram) {
    for statement in action.iter() {
        match statement {
            ast::Statement::Print(expressions) => {
                // 表示する式を逆順に取り出し，pushしてから最後にOpcode::Print(len)
                for e in expressions.iter().rev() {
                    compile_expression(e, vmprogram);
                }
                vmprogram.push(Opcode::Print(expressions.len()));
            }
        }
    }
}

fn compile_expression(expression: &ast::Expression, vmprogram: &mut VMProgram) {
    // 式をコンパイル
    // compile_expressionはeval関数のように再帰しながら式をコンパイルする
    match expression {
        ast::Expression::Value(v) => {
            vmprogram.push(Opcode::Push(v.clone()));
        }
        ast::Expression::BinaryOp { op, left, right } => {
            compile_expression(left, vmprogram);
            compile_expression(right, vmprogram);
            compile_operator(op, vmprogram);
        }
    }
}

fn compile_operator(op: &ast::Operator, vmprogram: &mut VMProgram) {
    vmprogram.push(match op {
        ast::Operator::Add => Opcode::Add,
        ast::Operator::Sub => Opcode::Sub,
        ast::Operator::Mul => Opcode::Mul,
        ast::Operator::Div => Opcode::Div,
    })
}

#[test]
fn test_compile() {
    let ast = vec![ast::Item {
        pattern: ast::Pattern::Begin,
        action: vec![ast::Statement::Print(vec![
            ast::Expression::Value(ast::Value::Num(1.0)),
            ast::Expression::Value(ast::Value::Num(2.0)),
        ])],
    }];
    let expect = vec![
        Opcode::Push(ast::Value::Num(2.0)),
        Opcode::Push(ast::Value::Num(1.0)),
        Opcode::Print(2),
        Opcode::End,
    ];
    let actual = compile(&ast);

    assert_eq!(&expect, &actual);
}

#[test]
fn test_compile2() {
    let ast = vec![ast::Item {
        pattern: ast::Pattern::Begin,
        action: vec![
            ast::Statement::Print(vec![
                ast::Expression::BinaryOp {
                    op: ast::Operator::Div,
                    left: Box::new(ast::Expression::Value(ast::Value::Num(6.0))),
                    right: Box::new(ast::Expression::Value(ast::Value::Num(2.0))),
                }
            ])
        ],
    }];
    let expect = vec![
        Opcode::Push(ast::Value::Num(6.0)),
        Opcode::Push(ast::Value::Num(2.0)),
        Opcode::Div,
        Opcode::Print(1),
        Opcode::End,
    ];
    let actual = compile(&ast);

    assert_eq!(&expect, &actual);
}

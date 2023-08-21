use std::collections::HashMap;

use crate::ast;
use crate::ast::Value;
use crate::vm::Opcode;

pub type VMProgram = Vec<Opcode>;
type Asm = Vec<OpcodeL>;

#[derive(Debug, PartialEq, Clone)]
enum OpcodeL {
    End,
    Nop,
    Push(Value),
    Pop,
    Jump(String),
    If(String),
    NIf(String),
    // Expression
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    Cat,
    And,
    Or,
    LessThan,
    LessEqualThan,
    NotEqual,
    Equal,
    GreaterThan,
    GreaterEqualThan,
    // AWK
    Readline,
    Print(usize),
    GetField,
    // Variable
    InitEnv(usize),
    LoadVar(String),
    SetVar(String),
    // ジャンプ先を示す
    Label(String),
}

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
    let mut asm: Asm = vec![];

    // BEGINパターンを探しコンパイル
    compile_all_begin_pattern(ast, &mut asm);

    // Always，Expressionパターンを探してコンパイル
    compile_normal_pattern(ast, &mut asm);

    // ENDパターンを探してコンパイル
    compile_all_end_pattern(ast, &mut asm);

    // 最後にENDを追加 (そうしないとVMが終了しない)
    asm.push(OpcodeL::End);

    asm_to_vmprogram(&asm)
}

// 全てのBEGINパターンを探してコンパイルする
fn compile_all_begin_pattern(ast: &ast::Program, asm: &mut Asm) {
    // fin BEGIN pattern
    let items = ast
        .iter()
        .filter(|i| matches!(i.pattern, ast::Pattern::Begin))
        .collect::<Vec<_>>();

    for item in items.into_iter() {
        // actionの列をコンパイル
        compile_action(&item.action, asm);
    }
}

fn compile_normal_pattern(ast: &ast::Program, asm: &mut Asm) {
    // BEGIN/END以外のパターンが存在するか確認
    let items = ast
        .iter()
        .filter(|i| !matches!(i.pattern, ast::Pattern::Begin))
        .filter(|i| !matches!(i.pattern, ast::Pattern::End))
        .collect::<Vec<_>>();

    if items.is_empty() {
        return;
    }

    asm.push(OpcodeL::Label("loop".to_string()));
    // 行を読み込む
    asm.push(OpcodeL::Readline);
    // EOF (スタックのトップが1.0)なら終了
    asm.push(OpcodeL::If("theend".to_string()));

    let mut expression_index = 0;
    for item in items.into_iter() {
        if let ast::Pattern::Expression(e) = &item.pattern {
            let label = format!("exp{}", expression_index);
            compile_expression(e, asm);
            asm.push(OpcodeL::NIf(label.to_string()));
            compile_action(&item.action, asm);
            asm.push(OpcodeL::Label(label));
            expression_index += 1;
        } else {
            compile_action(&item.action, asm);
        }

    }

    asm.push(OpcodeL::Jump("loop".to_string()));
    asm.push(OpcodeL::Label("theend".to_string()));
}

fn compile_all_end_pattern(ast: &ast::Program, asm: &mut Asm) {
    // fin BEGIN pattern
    let items = ast
        .iter()
        .filter(|i| matches!(i.pattern, ast::Pattern::End))
        .collect::<Vec<_>>();

    for item in items.into_iter() {
        // actionの列をコンパイル
        compile_action(&item.action, asm);
    }
}

// action ::: {}で囲われた一連のコード
fn compile_action(action: &ast::Action, asm: &mut Asm) {
    for statement in action.iter() {
        match statement {
            ast::Statement::Print(expressions) => {
                // 表示する式を逆順に取り出し，pushしてから最後にOpcodeL::Print(len)
                for e in expressions.iter().rev() {
                    compile_expression(e, asm);
                }
                asm.push(OpcodeL::Print(expressions.len()));
            }
            ast::Statement::Expression(expression) => {
                // 式単体の場合，最後にpopする
                compile_expression(expression, asm);
                asm.push(OpcodeL::Pop);
            }
        }
    }
}

fn compile_expression(expression: &ast::Expression, asm: &mut Asm) {
    // 式をコンパイル
    // compile_expressionはeval関数のように再帰しながら式をコンパイルする
    match expression {
        ast::Expression::Value(v) => {
            asm.push(OpcodeL::Push(v.clone()));
        }
        ast::Expression::BinaryOp { op, left, right } => {
            compile_expression(left, asm);
            compile_expression(right, asm);
            compile_operator(op, asm);
        }
        ast::Expression::GetField(e) => {
            compile_expression(e, asm);
            asm.push(OpcodeL::GetField);
        }
        ast::Expression::Name(name) => {
            asm.push(OpcodeL::LoadVar(name.to_string()));
        }
        ast::Expression::Assign { lval, expr } => {
            compile_expression(expr, asm);
            match lval {
                ast::LValue::Name(name) => asm.push(OpcodeL::SetVar(name.to_string())),
            }
        }
    }
}

fn compile_operator(op: &ast::Operator, asm: &mut Asm) {
    asm.push(match op {
        ast::Operator::Add => OpcodeL::Add,
        ast::Operator::Sub => OpcodeL::Sub,
        ast::Operator::Mul => OpcodeL::Mul,
        ast::Operator::Div => OpcodeL::Div,
        ast::Operator::Pow => OpcodeL::Pow,
        ast::Operator::Mod => OpcodeL::Mod,
        ast::Operator::Cat => OpcodeL::Cat,
        ast::Operator::And => OpcodeL::And,
        ast::Operator::Or => OpcodeL::Or,
        ast::Operator::LessThan => OpcodeL::LessThan,
        ast::Operator::LessEqualThan => OpcodeL::LessEqualThan,
        ast::Operator::NotEqual => OpcodeL::NotEqual,
        ast::Operator::Equal => OpcodeL::Equal,
        ast::Operator::GreaterThan => OpcodeL::GreaterThan,
        ast::Operator::GreaterEqualThan => OpcodeL::GreaterEqualThan,
    })
}

fn asm_to_vmprogram(asm: &Asm) -> VMProgram {
    let mut a = asm.to_vec();

    // 変数名の解決
    let mut names: HashMap<String, usize> = HashMap::new();
    // 全ての変数名を探索
    for i in a.iter() {
        if let OpcodeL::SetVar(name) = i {
            if names.get(name).is_none() {
                names.insert(name.to_string(), names.len());
            }
        }
        if let OpcodeL::LoadVar(name) = i {
            if names.get(name).is_none() {
                names.insert(name.to_string(), names.len());
            }
        }
    }

    // 変数分の領域を確保
    if !names.is_empty() {
        a.insert(0, OpcodeL::InitEnv(names.len()));
    }

    // ラベル名の解決
    // 初めに全てのラベル位置を特定してジャンプ先の要素番号を特定する
    // これ以降アセンブリに追加，削除してはいけない
    let mut labels: HashMap<String, usize> = HashMap::new();
    // 計算量が大きいので見直す
    while !a
        .iter()
        .filter(|i| matches!(i, OpcodeL::Label(_)))
        .collect::<Vec<_>>()
        .is_empty()
    {
        for (i, op) in a.iter().enumerate() {
            if let OpcodeL::Label(labelname) = op {
                labels.insert(labelname.to_string(), i);
                a.remove(i);
                break;
            }
        }
    }

    let mut bytecode: VMProgram = vec![];

    for op in a.iter() {
        bytecode.push(match op {
            OpcodeL::End => Opcode::End,
            OpcodeL::Nop => Opcode::Nop,
            OpcodeL::Push(value) => Opcode::Push(value.clone()),
            OpcodeL::Pop => Opcode::Pop,
            // TODO
            OpcodeL::Jump(label) => Opcode::Jump(*labels.get(label).unwrap()),
            OpcodeL::If(label) => Opcode::If(*labels.get(label).unwrap()),
            OpcodeL::NIf(label) => Opcode::NIf(*labels.get(label).unwrap()),
            // Expression
            OpcodeL::Add => Opcode::Add,
            OpcodeL::Sub => Opcode::Sub,
            OpcodeL::Mul => Opcode::Mul,
            OpcodeL::Div => Opcode::Div,
            OpcodeL::Pow => Opcode::Pow,
            OpcodeL::Mod => Opcode::Mod,
            OpcodeL::Cat => Opcode::Cat,
            OpcodeL::And => Opcode::And,
            OpcodeL::Or => Opcode::Or,
            OpcodeL::LessThan => Opcode::LessThan,
            OpcodeL::LessEqualThan => Opcode::LessEqualThan,
            OpcodeL::NotEqual => Opcode::NotEqual,
            OpcodeL::Equal => Opcode::Equal,
            OpcodeL::GreaterThan => Opcode::GreaterThan,
            OpcodeL::GreaterEqualThan => Opcode::GreaterEqualThan,
            // AWK
            OpcodeL::Readline => Opcode::Readline,
            OpcodeL::Print(len) => Opcode::Print(*len),
            OpcodeL::GetField => Opcode::GetField,
            // Variable
            OpcodeL::InitEnv(n) => Opcode::InitEnv(*n),
            OpcodeL::LoadVar(n) => Opcode::LoadVar(*names.get(n).unwrap()),
            OpcodeL::SetVar(n) => Opcode::SetVar(*names.get(n).unwrap()),
            // ジャンプ先を示す
            OpcodeL::Label(_label) => unreachable!(),
        })
    }
    bytecode
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
        action: vec![ast::Statement::Print(vec![ast::Expression::BinaryOp {
            op: ast::Operator::Div,
            left: Box::new(ast::Expression::Value(ast::Value::Num(6.0))),
            right: Box::new(ast::Expression::Value(ast::Value::Num(2.0))),
        }])],
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

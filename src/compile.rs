use indexmap::IndexSet;
use std::collections::HashMap;

use crate::ast;
use crate::ast::Value;
use crate::ifunc;
use crate::vm::Opcode;

pub type VMProgram = Vec<Opcode>;
type Asm = Vec<OpcodeL>;

struct CompileEnv {
    // while文が使ったラベルのカウント
    while_label_count: usize,
    // for文が使ったラベルのカウント
    for_label_count: usize,
    // if文が使ったラベルのカウント
    if_label_count: usize,
    // 登場する変数の一覧
    variables: IndexSet<String>,
    // 登場する関数の一覧
    functions: HashMap<String, usize>,
    // 関数の引数
    func_args: Vec<String>,
    // break, continueのジャンプ先
    // >0でwhile<0でfor
    break_continue: Vec<BCLabel>,
}

enum BCLabel {
    For(usize),
    While(usize),
}

#[derive(Debug, PartialEq, Clone)]
enum OpcodeL {
    End,
    Push(Value),
    Pop,
    Jump(String),
    If(String),
    NIf(String),
    Call(usize),
    CallUserFunc(String),
    Return,
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
    Printf(usize),
    GetField,
    // Variable
    InitEnv(usize),
    InitEnvArray(usize),
    LoadVar(String),
    SetVar(String),
    LoadArray(String),
    SetArray(String),
    LoadSFVar(usize),
    SetSFVar(usize),
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

pub fn compile(ast: &ast::Program) -> Result<VMProgram, &str> {
    // そのうちはコンパイルエラーをResultで返すようにしたい
    // (エラーは呼び出し側で処理すべきなので)
    let mut asm: Asm = vec![];

    // コンパイル時環境
    let mut env = CompileEnv {
        while_label_count: 0,
        for_label_count: 0,
        if_label_count: 0,
        variables: IndexSet::new(),
        functions: HashMap::new(),
        func_args: vec![],
        break_continue: vec![],
    };

    find_user_definition_function(ast, &mut env);

    // BEGINパターンを探しコンパイル
    compile_all_begin_pattern(ast, &mut asm, &mut env)?;

    // Always，Expressionパターンを探してコンパイル
    compile_normal_pattern(ast, &mut asm, &mut env)?;

    // ENDパターンを探してコンパイル
    compile_all_end_pattern(ast, &mut asm, &mut env)?;

    // 最後にENDを追加 (そうしないとVMが終了しない)
    asm.push(OpcodeL::End);

    compile_user_definition_function(ast, &mut asm, &mut env)?;

    Ok(asm_to_vmprogram(&asm, &mut env))
}

fn find_user_definition_function(ast: &ast::Program, env: &mut CompileEnv) {
    ast.iter().for_each(|i| {
        if let ast::Item::Function(func) = i {
            env.functions.insert(func.name.clone(), func.args.len());
        }
    })
}

/*
* ユーザー定義関数の仕様についてのメモ:
* ・現状
*      定義時の引数の数: n
*      呼出時の引数の数: m
*      n > m → エラーを吐く(関数のスタックフレームが引数の数固定のため)
*      n < m → エラーなく動く(awkでは警告が出る)
* ・正しい動作
*      n > m → 問題なく動作する(少ない分の引数はローカル変数扱い)
*      n < m → 警告
*
* ・正しくするには
*      関数のスタックフレームを作成する命令を追加する
*      コンパイル時に引数の数をチェックする
*
*/
fn compile_user_definition_function(
    ast: &ast::Program,
    asm: &mut Asm,
    env: &mut CompileEnv,
) -> Result<(), &'static str> {
    // ユーザー定義関数を探す
    let functions = ast
        .iter()
        .filter_map(|i| {
            if let ast::Item::Function(func) = i {
                Some(func)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for func in functions.into_iter() {
        // 引数
        env.func_args = func.args.clone();
        asm.push(OpcodeL::Label(format!("userfn_{}", &func.name)));
        // 関数内のactionであることを伝える方法を用意する
        compile_statement(&func.action, asm, env)?;
        // 空returnはnoneを返す
        asm.push(OpcodeL::Push(ast::Value::None));
        asm.push(OpcodeL::Return);
        env.func_args = vec![];
    }

    Ok(())
}

// 全てのBEGINパターンを探してコンパイルする
fn compile_all_begin_pattern(
    ast: &ast::Program,
    asm: &mut Asm,
    env: &mut CompileEnv,
) -> Result<(), &'static str> {
    // find BEGIN pattern
    let items = ast
        .iter()
        .filter_map(|i| match i {
            ast::Item::PatternAction(i) => {
                if matches!(i.pattern, ast::Pattern::Begin) {
                    return Some(i);
                }
                None
            }
            ast::Item::Function(_) => None,
        })
        .collect::<Vec<_>>();

    for item in items.into_iter() {
        // actionの列をコンパイル
        compile_statement(&item.action, asm, env)?;
    }

    Ok(())
}

// 全ての通常パターンをコンパイルする
fn compile_normal_pattern(
    ast: &ast::Program,
    asm: &mut Asm,
    env: &mut CompileEnv,
) -> Result<(), &'static str> {
    // BEGIN/END以外のパターンが存在するか確認
    let items = ast
        .iter()
        .filter_map(|i| match i {
            ast::Item::PatternAction(i) => {
                if !(matches!(i.pattern, ast::Pattern::Begin)
                    || matches!(i.pattern, ast::Pattern::End))
                {
                    return Some(i);
                }
                None
            }
            ast::Item::Function(_) => None,
        })
        .collect::<Vec<_>>();

    if items.is_empty() {
        return Ok(());
    }

    asm.push(OpcodeL::Label("loop".to_string()));
    // 行を読み込む
    asm.push(OpcodeL::Readline);
    // EOF (スタックのトップが1.0)なら終了
    asm.push(OpcodeL::If("theend".to_string()));

    let mut expression_index = 0;
    for item in items.into_iter() {
        // 式パターン
        if let ast::Pattern::Expression(e) = &item.pattern {
            let label = format!("exp{}", expression_index);
            compile_expression(e, asm, env)?;
            asm.push(OpcodeL::NIf(label.to_string()));
            compile_statement(&item.action, asm, env)?;
            asm.push(OpcodeL::Label(label));
            expression_index += 1;

        // Alwaysパターン
        } else {
            compile_statement(&item.action, asm, env)?;
        }
    }

    asm.push(OpcodeL::Jump("loop".to_string()));
    asm.push(OpcodeL::Label("theend".to_string()));

    Ok(())
}

fn compile_all_end_pattern(
    ast: &ast::Program,
    asm: &mut Asm,
    env: &mut CompileEnv,
) -> Result<(), &'static str> {
    // fin BEGIN pattern
    let items = ast
        .iter()
        .filter_map(|i| match i {
            ast::Item::PatternAction(i) => {
                if matches!(i.pattern, ast::Pattern::End) {
                    return Some(i);
                }
                None
            }
            ast::Item::Function(_) => None,
        })
        .collect::<Vec<_>>();

    for item in items.into_iter() {
        // actionの列をコンパイル
        compile_statement(&item.action, asm, env)?;
    }

    Ok(())
}

// action ::: {}で囲われた一連のコード
fn compile_statement(
    statement: &ast::Statement,
    asm: &mut Asm,
    env: &mut CompileEnv,
) -> Result<(), &'static str> {
    match statement {
        // {}で囲われたAction
        ast::Statement::Action(action) => {
            for s in action {
                compile_statement(s, asm, env)?;
            }
        }

        // print文
        ast::Statement::Print(expressions) => {
            for e in expressions.iter() {
                compile_expression(e, asm, env)?;
            }
            asm.push(OpcodeL::Print(expressions.len()));
        }

        // printf文
        ast::Statement::Printf { fmt, args } => {
            compile_expression(fmt, asm, env)?;
            for e in args.iter() {
                compile_expression(e, asm, env)?;
            }
            asm.push(OpcodeL::Printf(args.len()))
        }

        // 式
        ast::Statement::Expression(expression) => {
            // 式単体の場合，最後にpopする
            compile_expression(expression, asm, env)?;
            asm.push(OpcodeL::Pop);
        }

        // while文
        ast::Statement::While { exp, stat } => {
            // while文用のラベルを使う
            let label = env.while_label_count;
            env.while_label_count += 1;
            env.break_continue.push(BCLabel::While(label));

            // continueの飛び先
            asm.push(OpcodeL::Label(format!("while_s_{label}")));
            compile_expression(exp, asm, env)?;
            asm.push(OpcodeL::NIf(format!("while_e_{label}")));
            compile_statement(stat, asm, env)?;
            asm.push(OpcodeL::Jump(format!("while_s_{label}")));
            // breakの飛び先
            asm.push(OpcodeL::Label(format!("while_e_{label}")));

            env.break_continue.pop().unwrap();
        }

        // For
        //    initialize
        // ┌─►conditon?──┐
        // │  statement  │NO
        // └──update     │
        //               ▼
        ast::Statement::For {
            init,
            test,
            updt,
            stat,
        } => {
            let label = env.for_label_count;
            env.for_label_count += 1;
            env.break_continue.push(BCLabel::For(label));

            // init
            compile_statement(init, asm, env)?;
            asm.push(OpcodeL::Label(format!("for_s_{label}")));
            // condition?
            compile_expression(test, asm, env)?;
            asm.push(OpcodeL::NIf(format!("for_e_{label}")));
            compile_statement(stat, asm, env)?;
            // update
            asm.push(OpcodeL::Label(format!("for_c_{label}"))); // continueの飛び先
            compile_statement(updt, asm, env)?;
            asm.push(OpcodeL::Jump(format!("for_s_{label}")));
            asm.push(OpcodeL::Label(format!("for_e_{label}"))); // breakの飛び先

            env.break_continue.pop().unwrap();
        }

        // If文
        ast::Statement::If { cond, stat } => {
            let label = env.if_label_count;
            env.if_label_count += 1;
            compile_expression(cond, asm, env)?;
            asm.push(OpcodeL::NIf(format!("if_{label}")));
            compile_statement(stat, asm, env)?;
            asm.push(OpcodeL::Label(format!("if_{label}")));
        }

        // If文
        ast::Statement::IfElse { cond, stat, els } => {
            let label = env.if_label_count;
            env.if_label_count += 1;

            compile_expression(cond, asm, env)?;
            asm.push(OpcodeL::NIf(format!("if_{label}")));
            compile_statement(stat, asm, env)?;
            asm.push(OpcodeL::Jump(format!("if_elskip_{label}")));
            asm.push(OpcodeL::Label(format!("if_{label}")));
            compile_statement(els, asm, env)?;
            asm.push(OpcodeL::Label(format!("if_elskip_{label}")));
        }

        // Return文
        // TODO: 関数の外でのreturn文をエラーにする
        ast::Statement::Return(e) => {
            compile_expression(e, asm, env)?;
            asm.push(OpcodeL::Return);
        }

        ast::Statement::Break => {
            if let Some(label) = env.break_continue.last() {
                match label {
                    BCLabel::For(l) => {
                        asm.push(OpcodeL::Jump(format!("for_e_{l}")));
                    }
                    BCLabel::While(l) => {
                        asm.push(OpcodeL::Jump(format!("while_e_{l}")));
                    }
                }
            } else {
                return Err("`break' is not allowed outside a loop");
            }
        }

        ast::Statement::Continue => {
            if let Some(label) = env.break_continue.last() {
                match label {
                    BCLabel::For(l) => {
                        asm.push(OpcodeL::Jump(format!("for_c_{l}")));
                    }
                    BCLabel::While(l) => {
                        asm.push(OpcodeL::Jump(format!("while_s_{l}")));
                    }
                }
            } else {
                return Err("`continue' is not allowed outside a loop");
            }
        }
    }

    Ok(())
}

fn compile_expression(
    expression: &ast::Expression,
    asm: &mut Asm,
    env: &mut CompileEnv,
) -> Result<(), &'static str> {
    // 式をコンパイル
    // compile_expressionはeval関数のように再帰しながら式をコンパイルする
    match expression {
        ast::Expression::Value(v) => {
            asm.push(OpcodeL::Push(v.clone()));
        }
        ast::Expression::BinaryOp { op, left, right } => {
            compile_expression(left, asm, env)?;
            compile_expression(right, asm, env)?;
            compile_operator(op, asm);
        }
        ast::Expression::IncDec { op, lval } => {
            // 下のLvalueと共通化
            let loadlval = |lvalue: &ast::LValue,
                            asm: &mut Asm,
                            env: &mut CompileEnv|
             -> Result<(), &'static str> {
                match lvalue {
                    ast::LValue::Name(name) => {
                        if let Some(sfi) = env.func_args.iter().position(|n| n == name) {
                            asm.push(OpcodeL::LoadSFVar(sfi));
                        } else {
                            // 関数の引数にない場合
                            env.variables.insert(name.to_string());
                            asm.push(OpcodeL::LoadVar(name.to_string()));
                        }
                    }
                    ast::LValue::Array { name, expr_list } => {
                        // 順番に注意
                        for expr in expr_list.iter() {
                            compile_expression(expr, asm, env)?;
                        }
                        asm.push(OpcodeL::LoadArray(name.to_string()));
                    }
                };
                Ok(())
            };
            let assign = |lvalue: &ast::LValue,
                          asm: &mut Asm,
                          env: &mut CompileEnv|
             -> Result<(), &'static str> {
                match lvalue {
                    ast::LValue::Name(name) => {
                        if let Some(sfi) = env.func_args.iter().position(|n| n == name) {
                            asm.push(OpcodeL::SetSFVar(sfi));
                        } else {
                            env.variables.insert(name.to_string());
                            asm.push(OpcodeL::SetVar(name.to_string()))
                        }
                    }
                    ast::LValue::Array { name, expr_list } => {
                        for expr in expr_list.iter() {
                            compile_expression(expr, asm, env)?;
                        }
                        asm.push(OpcodeL::SetArray(name.to_string()));
                    }
                }
                Ok(())
            };
            match op {
                ast::IncDecType::PreInc => {
                    // increment lvalue
                    loadlval(lval, asm, env)?;
                    asm.push(OpcodeL::Push(ast::Value::Num(1.0)));
                    asm.push(OpcodeL::Add);
                    assign(lval, asm, env)?;
                    // load lvalue
                    loadlval(lval, asm, env)?;
                }
                ast::IncDecType::PreDec => {
                    loadlval(lval, asm, env)?;
                    asm.push(OpcodeL::Push(ast::Value::Num(1.0)));
                    asm.push(OpcodeL::Sub);
                    assign(lval, asm, env)?;
                    loadlval(lval, asm, env)?;
                }
                ast::IncDecType::PostInc => {
                    // TODO: 未初期化のときi++は0．無理矢理実装している
                    // increment lvalue
                    loadlval(lval, asm, env)?;
                    asm.push(OpcodeL::Push(ast::Value::Num(1.0)));
                    asm.push(OpcodeL::Add);
                    assign(lval, asm, env)?;
                    // load lvalue
                    loadlval(lval, asm, env)?;
                    asm.push(OpcodeL::Push(ast::Value::Num(1.0)));
                    asm.push(OpcodeL::Sub);
                }
                ast::IncDecType::PostDec => {
                    loadlval(lval, asm, env)?;
                    asm.push(OpcodeL::Push(ast::Value::Num(1.0)));
                    asm.push(OpcodeL::Sub);
                    assign(lval, asm, env)?;
                    loadlval(lval, asm, env)?;
                    asm.push(OpcodeL::Push(ast::Value::Num(1.0)));
                    asm.push(OpcodeL::Add);
                }
            }
        }
        ast::Expression::GetField(e) => {
            compile_expression(e, asm, env)?;
            asm.push(OpcodeL::GetField);
        }
        ast::Expression::LValue(lvalue) => match lvalue {
            ast::LValue::Name(name) => {
                if let Some(sfi) = env.func_args.iter().position(|n| n == name) {
                    asm.push(OpcodeL::LoadSFVar(sfi));
                } else {
                    // 関数の引数にない場合
                    env.variables.insert(name.to_string());
                    asm.push(OpcodeL::LoadVar(name.to_string()));
                }
            }
            ast::LValue::Array { name, expr_list } => {
                // 順番に注意
                for expr in expr_list.iter() {
                    compile_expression(expr, asm, env)?;
                }
                asm.push(OpcodeL::LoadArray(name.to_string()));
            }
        },
        ast::Expression::Assign { lval, expr } => {
            // TODO: 引数の書き換え
            compile_expression(expr, asm, env)?;
            match lval {
                ast::LValue::Name(name) => {
                    if let Some(sfi) = env.func_args.iter().position(|n| n == name) {
                        asm.push(OpcodeL::SetSFVar(sfi));
                    } else {
                        env.variables.insert(name.to_string());
                        asm.push(OpcodeL::SetVar(name.to_string()))
                    }
                }
                ast::LValue::Array { name, expr_list } => {
                    for expr in expr_list.iter() {
                        compile_expression(expr, asm, env)?;
                    }
                    asm.push(OpcodeL::SetArray(name.to_string()));
                }
            }
            asm.push(OpcodeL::Push(Value::None));
        }
        ast::Expression::CallIFunc { name, args } => {
            for e in args.iter().rev() {
                compile_expression(e, asm, env)?;
            }
            let index = ifunc::get_index_from_name(name).unwrap();
            if args.len() != ifunc::get_len_of_args(index) {
                return Err("Invalid arg len");
            }
            // TODO
            // ここで引数の個数はチェックしたい
            asm.push(OpcodeL::Call(index));
        }
        ast::Expression::CallUserFunc { name, args } => {
            if *env.functions.get(name).unwrap() < args.len() {
                eprintln!(
                    "warning: function `{}' called with more arguments than declared",
                    name
                );
            }
            // 引数をpushする(前から)
            for a in args.iter() {
                compile_expression(a, asm, env)?;
            }
            // 引数の数をpushする
            asm.push(OpcodeL::Push(ast::Value::Num(args.len() as f64)));
            asm.push(OpcodeL::CallUserFunc(format!("userfn_{}", name)));
        }
    }

    Ok(())
}

fn compile_operator(op: &ast::BOperator, asm: &mut Asm) {
    asm.push(match op {
        ast::BOperator::Add => OpcodeL::Add,
        ast::BOperator::Sub => OpcodeL::Sub,
        ast::BOperator::Mul => OpcodeL::Mul,
        ast::BOperator::Div => OpcodeL::Div,
        ast::BOperator::Pow => OpcodeL::Pow,
        ast::BOperator::Mod => OpcodeL::Mod,
        ast::BOperator::Cat => OpcodeL::Cat,
        ast::BOperator::And => OpcodeL::And,
        ast::BOperator::Or => OpcodeL::Or,
        ast::BOperator::LessThan => OpcodeL::LessThan,
        ast::BOperator::LessEqualThan => OpcodeL::LessEqualThan,
        ast::BOperator::NotEqual => OpcodeL::NotEqual,
        ast::BOperator::Equal => OpcodeL::Equal,
        ast::BOperator::GreaterThan => OpcodeL::GreaterThan,
        ast::BOperator::GreaterEqualThan => OpcodeL::GreaterEqualThan,
    })
}

fn asm_to_vmprogram(asm: &Asm, _env: &mut CompileEnv) -> VMProgram {
    let mut a = asm.to_vec();

    // 変数名の解決
    let mut names: HashMap<String, usize> = HashMap::new();
    let mut arraynames: HashMap<String, usize> = HashMap::new();
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
        if let OpcodeL::LoadArray(name) = i {
            if arraynames.get(name).is_none() {
                arraynames.insert(name.to_string(), arraynames.len());
            }
        }
        if let OpcodeL::SetArray(name) = i {
            if arraynames.get(name).is_none() {
                arraynames.insert(name.to_string(), arraynames.len());
            }
        }
    }

    // 変数分の領域を確保
    if !arraynames.is_empty() {
        a.insert(0, OpcodeL::InitEnvArray(arraynames.len()));
    }
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
            OpcodeL::Push(value) => Opcode::Push(value.clone()),
            OpcodeL::Pop => Opcode::Pop,
            // TODO
            OpcodeL::Jump(label) => Opcode::Jump(*labels.get(label).unwrap()),
            OpcodeL::If(label) => Opcode::If(*labels.get(label).unwrap()),
            OpcodeL::NIf(label) => Opcode::NIf(*labels.get(label).unwrap()),
            OpcodeL::Call(i) => Opcode::Call(*i),
            OpcodeL::CallUserFunc(label) => Opcode::CallUserFunc(*labels.get(label).unwrap()),
            OpcodeL::Return => Opcode::Return,
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
            OpcodeL::Printf(len) => Opcode::Printf(*len),
            OpcodeL::GetField => Opcode::GetField,
            // Variable
            OpcodeL::InitEnv(n) => Opcode::InitEnv(*n),
            OpcodeL::InitEnvArray(n) => Opcode::InitEnvArray(*n),
            OpcodeL::LoadVar(n) => Opcode::LoadVar(*names.get(n).unwrap()),
            OpcodeL::SetArray(n) => Opcode::SetArray(*arraynames.get(n).unwrap()),
            OpcodeL::SetVar(n) => Opcode::SetVar(*names.get(n).unwrap()),
            OpcodeL::LoadArray(n) => Opcode::LoadArray(*arraynames.get(n).unwrap()),
            OpcodeL::LoadSFVar(n) => Opcode::LoadSFVar(*n),
            OpcodeL::SetSFVar(n) => Opcode::SetSFVar(*n),
            // ジャンプ先を示す
            OpcodeL::Label(_label) => unreachable!(),
        })
    }
    bytecode
}

#[test]
fn test_compile() {
    let ast = vec![ast::Item::PatternAction(ast::PatternAction {
        pattern: ast::Pattern::Begin,
        action: ast::Statement::Action(vec![ast::Statement::Print(vec![
            ast::Expression::Value(ast::Value::Num(1.0)),
            ast::Expression::Value(ast::Value::Num(2.0)),
        ])]),
    })];
    let expect = vec![
        Opcode::Push(ast::Value::Num(1.0)),
        Opcode::Push(ast::Value::Num(2.0)),
        Opcode::Print(2),
        Opcode::End,
    ];
    let actual = compile(&ast).unwrap();

    assert_eq!(&expect, &actual);
}

#[test]
fn test_compile2() {
    let ast = vec![ast::Item::PatternAction(ast::PatternAction {
        pattern: ast::Pattern::Begin,
        action: ast::Statement::Action(vec![ast::Statement::Print(vec![
            ast::Expression::BinaryOp {
                op: ast::BOperator::Div,
                left: Box::new(ast::Expression::Value(ast::Value::Num(6.0))),
                right: Box::new(ast::Expression::Value(ast::Value::Num(2.0))),
            },
        ])]),
    })];
    let expect = vec![
        Opcode::Push(ast::Value::Num(6.0)),
        Opcode::Push(ast::Value::Num(2.0)),
        Opcode::Div,
        Opcode::Print(1),
        Opcode::End,
    ];
    let actual = compile(&ast).unwrap();

    assert_eq!(&expect, &actual);
}

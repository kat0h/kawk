use crate::ast;
use crate::vm::Opcode;

pub fn compile(ast: &ast::Program) {
    let vmprogram: Vec<Opcode> = vec![];

    // BEGINパターンを探しコンパイル
    compile_all_begin_pattern(ast, &vmprogram);
}

fn compile_all_begin_pattern(ast: &ast::Program, vmprogram: &Vec<Opcode>) {
    let items = ast
        .iter()
        .filter(|i| matches!(i.pattern, ast::Pattern::Begin))
        .collect::<Vec<_>>();

    for item in items.into_iter() {
        compile_action(&item.action, vmprogram);
    }
}

fn compile_action(action: &ast::Action, vmprogram: &Vec<Opcode>) {
    for statement in action.into_iter() {
        match statement {
            ast::Statement::Print(expressions) => {

            }
        }
    }
}

fn compile_expression(expression: &ast::Expression, vmprogram: &Vec<Opcode>) {

}

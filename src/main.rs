use std::io;

mod ast;
mod compile;
mod parser;
mod vm;

fn main() {
    // 引数を取得
    let args: Vec<String> = std::env::args().collect();

    // コマンド単体で呼ばれた際はヘルプメッセージを表示
    if args.len() == 1 {
        let binary_name = &args[0];
        println!("usage: {} 'prog'", binary_name);
        return;
    }

    let program = &args[1];

    // Parse program
    // ここを綺麗に書き直したい
    // Goのエラー処理みたいに書くべきなのか，そうではないのか
    let ast = parser::parse(program);
    if ast.is_err() {
        println!("Syntax Error!");
        dbg!(&ast);
        return;
    }
    let ast = ast.unwrap();

    // Compile program
    let vmprg = compile::compile(&ast);

    // Run Program
    let mut r = std::io::stdin().lock();
    let mut w = std::io::stdout().lock();
    let mut vm = vm::VM::new(&vmprg);
    vm.run(&mut r, &mut w);
}

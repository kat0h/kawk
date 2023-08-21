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
        // usage: awk 'prog'
        println!("usage: {} 'prog'", binary_name);
        return;
    }

    // Debug Level
    // 1 -> AST
    // 2 -> Byte Code
    // 3 -> print stack and env
    let debuglevel = if args.len() == 3 {
        if &args[2] == "1" {
            1
        } else if &args[2] == "2" {
            2
        } else {
            3
        }
    } else {
        0
    };

    let program = &args[1];

    // Parse program
    // ここを綺麗にフラットに書き直したい
    // Goのエラー処理みたいに書くべきなのか，そうではないのか
    let ast = parser::parse(program);
    if let Err(err) = &ast {
        let line = err.location.line;
        let col = err.location.column;
        eprintln!("Syntax Error!");
        // Syntaxエラーの時はもっと詳細にエラーを出したいよね
        eprintln!("{}", program.split('\n').collect::<Vec<&str>>()[line - 1]);
        eprintln!("{}^", " ".to_string().repeat(col - 1));
        dbg!(&ast);
        return;
    }
    let ast = ast.unwrap();
    if debuglevel == 1 {
        dbg!(ast);
        return;
    }

    // Compile program
    let vmprg = compile::compile(&ast);
    if debuglevel == 2 {
        show_vmprog(&vmprg);
        return;
    }

    // Run Program
    let mut r = std::io::stdin().lock();
    let mut w = std::io::stdout().lock();
    let mut vm = vm::VM::new(&vmprg);
    vm.run(&mut r, &mut w);

    if debuglevel == 3 {
        vm.show_stack_and_env();
    }
}

fn show_vmprog(vmprog: &compile::VMProgram) {
    for (i, opcode) in vmprog.iter().enumerate() {
        let opc = match opcode {
            vm::Opcode::End => "end",
            vm::Opcode::Push(_) => "push",
            vm::Opcode::Pop => "pop",
            vm::Opcode::Jump(_) => "jump",
            vm::Opcode::If(_) => "if",
            vm::Opcode::NIf(_) => "nif",
            // Expression
            vm::Opcode::Add => "add",
            vm::Opcode::Sub => "sub",
            vm::Opcode::Mul => "mul",
            vm::Opcode::Div => "div",
            vm::Opcode::Pow => "Pow",
            vm::Opcode::Mod => "Mod",
            vm::Opcode::Cat => "Cat",
            vm::Opcode::And => "And",
            vm::Opcode::Or => "Or",
            vm::Opcode::LessThan => "LessThan",
            vm::Opcode::LessEqualThan => "LessEqualThan",
            vm::Opcode::NotEqual => "NotEqual",
            vm::Opcode::Equal => "Equal",
            vm::Opcode::GreaterThan => "GreaterThan",
            vm::Opcode::GreaterEqualThan => "GreaterEqualThan",
            // AWK
            vm::Opcode::Readline => "readline",
            vm::Opcode::Print(_) => "print",
            vm::Opcode::GetField => "getfield",
            // Variable
            vm::Opcode::InitEnv(_) => "initenv",
            vm::Opcode::LoadVar(_) => "loadval",
            vm::Opcode::SetVar(_) => "setval",
        };

        let arg = match opcode {
            vm::Opcode::Push(val) => val.to_str(),
            vm::Opcode::Jump(i) => i.to_string(),
            vm::Opcode::If(i) => i.to_string(),
            vm::Opcode::NIf(i) => i.to_string(),
            vm::Opcode::Print(l) => l.to_string(),
            vm::Opcode::InitEnv(n) => n.to_string(),
            vm::Opcode::LoadVar(n) => n.to_string(),
            vm::Opcode::SetVar(n) => n.to_string(),
            _ => "".to_string(),
        };

        println!("{}\t{}\t{}", i, opc, &arg);
    }
}

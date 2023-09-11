use getopts::Options;

mod ast;
mod compile;
mod ifunc;
mod parser;
mod vm;

/*
 * AWK オプション
 *   -h            : ヘルプを表示して終了
 *   -f progfile   : progfileを実行
 *   -d 1|2|3      : デバッグレベル
 *   'program'     : programを実行
 */

struct Opts {
    debuglevel: DebugLevel,
}

#[derive(PartialEq, Eq)]
enum DebugLevel {
    None,
    Ast,
    ByteCode,
    Env
}

fn main() {
    // 引数を取得
    let args: Vec<String> = std::env::args().collect();
    let mut option = Opts {
        debuglevel: DebugLevel::None
    };

    // コマンド単体で呼ばれた際はヘルプメッセージを表示
    if args.len() == 1 {
        print_usage(&args[0]);
        return;
    }

    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help menu");
    opts.optopt("d", "debug", "Set debug level", "DEBUGLEVEL");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error: {}", e);
            print_usage(&args[0]);
            return;
        }
    };

    if matches.opt_present("h") {
        // TODO: ヘルプを実装
        println!("HELP");
        return;
    };

    if let Some(debuglevel) = matches.opt_str("d") {
        println!("DEBUGLEVEL: {}", debuglevel);
        option.debuglevel = if debuglevel == "1" {
            DebugLevel::Ast
        } else if debuglevel == "2" {
            DebugLevel::ByteCode
        } else if debuglevel == "3" {
            DebugLevel::Env
        } else {
            eprintln!("Invalid debuglevel: {}", debuglevel);
            return;
        }
    }

    let program = &args[1];

    // Parse program
    // ここを綺麗にフラットに書き直したい
    // Goのエラー処理みたいに書くべきなのか，そうではないのか
    let ast = match parser::parse(program) {
        Ok(ast) => ast,
        Err(err) => {
            let line = err.location.line;
            let col = err.location.column;
            eprintln!("Syntax Error!");
            // Syntaxエラーの時はもっと詳細にエラーを出したいよね
            eprintln!("{}", program.split('\n').collect::<Vec<&str>>()[line - 1]);
            eprintln!("{}^", " ".to_string().repeat(col - 1));
            dbg!(&err);
            return;
        }
    };
    if option.debuglevel == DebugLevel::Ast {
        dbg!(ast);
        return;
    }

    // Compile program
    let vmprg = match compile::compile(&ast) {
        Ok(vmprg) => vmprg,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    if option.debuglevel == DebugLevel::ByteCode {
        show_vmprog(&vmprg);
        return;
    }

    // Run Program
    let mut r = std::io::stdin().lock();
    let mut w = std::io::stdout().lock();
    let mut vm = vm::VM::new(&vmprg);
    vm.run(&mut r, &mut w);

    if option.debuglevel == DebugLevel::Env {
        vm.show_stack_and_env();
    }
}

fn print_usage(binary_name: &str) {
    println!("usage: {} 'prog'", binary_name);
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
            vm::Opcode::Call(_) => "call",
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
            // 内蔵関数と対応させたい
            vm::Opcode::Call(i) => i.to_string(),
            vm::Opcode::Print(l) => l.to_string(),
            vm::Opcode::InitEnv(n) => n.to_string(),
            vm::Opcode::LoadVar(n) => n.to_string(),
            vm::Opcode::SetVar(n) => n.to_string(),
            _ => "".to_string(),
        };

        println!("{}\t{}\t{}", i, opc, &arg);
    }
}

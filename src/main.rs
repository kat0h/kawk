use getopts::Options;
use indoc::indoc;
use std::fs::File;
use std::io::prelude::*;

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

// 速度計測

struct Opts {
    program_name: String,
    debuglevel: DebugLevel,
}

#[derive(PartialEq, Eq)]
enum DebugLevel {
    None,
    Ast,
    ByteCode,
    Env,
}

fn main() {
    // 引数を取得
    let args: Vec<String> = std::env::args().collect();
    let mut option = Opts {
        program_name: args[0].clone(),
        debuglevel: DebugLevel::None,
    };

    let mut opts = Options::new();
    opts.optflag("h", "help", "Print this help menu");
    opts.optopt("d", "", "Set debug level", "DEBUGLEVEL");
    opts.optopt("f", "", "filename to run", "progfile");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error: {}", e);
            print_usage(&option.program_name);
            return;
        }
    };

    if matches.opt_present("h") {
        // TODO: ヘルプを実装
        print_help(&option.program_name);
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

    let program = if let Some(filename) = matches.opt_str("f") {
        let mut f = match File::open(&filename) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("{}: fatal: cannot open source file `{}' for reading: No such file or directory", &args[0], filename);
                return;
            }
        };

        let mut contents = String::new();
        match f.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(err) => {
                eprintln!(
                    "{}: fatal: cannot read source file `{}': {}",
                    option.program_name, filename, err
                );
                return;
            }
        };
        contents
    } else {
        match matches.free.first() {
            Some(program) => program.clone(),
            None => {
                // 実行するプログラムがなければメッセージを表示
                print_usage(&args[0]);
                return;
            }
        }
    };

    // Parse
    let ast = match parser::parse(&program) {
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

    // Compile
    let vmprg = match compile::compile(&ast) {
        Ok(vmprg) => vmprg,
        Err(err) => {
            eprintln!("Compile Error: {}", err);
            return;
        }
    };
    if option.debuglevel == DebugLevel::ByteCode {
        show_vmprog(&vmprg);
        return;
    }

    // Run
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

fn print_help(binary_name: &str) {
    println!(
        indoc! {"
            Usage: {} [options] 
            options:
                    -f progfile
                        file to run
                    -d 1|2|3
                        specify debug level
        "},
        binary_name
    )
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
            vm::Opcode::CallUserFunc(_) => "calluserfunc",
            vm::Opcode::Return => "return",
            // Expression
            vm::Opcode::Add => "add",
            vm::Opcode::Sub => "sub",
            vm::Opcode::Mul => "mul",
            vm::Opcode::Div => "div",
            vm::Opcode::Pow => "pow",
            vm::Opcode::Mod => "mod",
            vm::Opcode::Cat => "cat",
            vm::Opcode::And => "and",
            vm::Opcode::Or => "or",
            vm::Opcode::LessThan => "lessthan",
            vm::Opcode::LessEqualThan => "lessequalthan",
            vm::Opcode::NotEqual => "notequal",
            vm::Opcode::Equal => "equal",
            vm::Opcode::GreaterThan => "greaterthan",
            vm::Opcode::GreaterEqualThan => "greaterequalthan",
            // AWK
            vm::Opcode::Readline => "readline",
            vm::Opcode::Print(_) => "print",
            vm::Opcode::Printf(_) => "printf",
            vm::Opcode::GetField => "getfield",
            // Variable
            vm::Opcode::InitEnv(_) => "initenv",
            vm::Opcode::InitEnvArray(_) => "initenvarray",
            vm::Opcode::LoadVar(_) => "loadval",
            vm::Opcode::SetVar(_) => "setval",
            vm::Opcode::LoadArray(_) => "loadarray",
            vm::Opcode::SetArray(_) => "setarray",
            vm::Opcode::LoadSFVar(_) => "loadsfvar",
            vm::Opcode::SetSFVar(_) => "setsfvar",
        };

        let arg = match opcode {
            vm::Opcode::Push(val) => val.to_dbgstr(),
            vm::Opcode::Jump(i) => i.to_string(),
            vm::Opcode::If(i) => i.to_string(),
            vm::Opcode::NIf(i) => i.to_string(),
            // 内蔵関数と対応させたい
            vm::Opcode::Call(i) => i.to_string(),
            vm::Opcode::CallUserFunc(i) => i.to_string(),
            vm::Opcode::Print(l) => l.to_string(),
            vm::Opcode::Printf(l) => l.to_string(),
            vm::Opcode::InitEnv(n) => n.to_string(),
            vm::Opcode::InitEnvArray(n) => n.to_string(),
            vm::Opcode::LoadVar(n) => n.to_string(),
            vm::Opcode::SetVar(n) => n.to_string(),
            vm::Opcode::LoadArray(n) => n.to_string(),
            vm::Opcode::SetArray(n) => n.to_string(),
            vm::Opcode::LoadSFVar(n) => n.to_string(),
            vm::Opcode::SetSFVar(n) => n.to_string(),
            _ => "".to_string(),
        };

        println!("{}\t{}\t{}", i, opc, &arg);
    }
}

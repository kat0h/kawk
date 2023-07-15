mod ast;
mod compile;
mod parser;
mod vm;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("usage: {} 'prog'", &args[0]);
    }

    parser::parse(&args[1]);
    compile::compile();
    // vm::vm_run();
}

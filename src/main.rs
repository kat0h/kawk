mod ast;
mod compile;
mod parser;
mod vm;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        let binary_name = &args[0];
        println!("usage: {} 'prog'", binary_name);
        return;
    }

    println!("{}", &args[1]);
}

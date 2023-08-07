mod value;
use crate::ast::Value;

use std::io::{self, BufRead, Write};

pub struct VM<'a> {
    program: &'a [Opcode],
    stack: Vec<Value>,
    pc: usize,

    fields: Vec<String>,
    nf: Value,
}

impl VM<'_> {
    pub fn new(program: &[Opcode]) -> VM {
        VM {
            program,
            stack: vec![],
            pc: 0,

            fields: vec![],
            nf: Value::Num(0.0),
        }
    }

    pub fn run<R: BufRead, W: Write>(&mut self, reader: &mut R, writer: &mut W) {
        loop {
            match &self.program[self.pc] {
                Opcode::End => {
                    break;
                }
                Opcode::Push(a) => {
                    self.stack.push(a.clone());
                }
                Opcode::Pop => {
                    self.stack.pop();
                }
                Opcode::Jump(pc) => {
                    self.pc = *pc;
                    continue;
                }
                /*
                 * If
                 * スタックの一番上がtruetyなとき指定されたポインタにジャンプ
                 */
                Opcode::If(pc) => {
                    if self.stack.pop().unwrap().is_true() {
                        self.pc = *pc;
                        continue;
                    }
                }
                /*
                 * Readline
                 * 標準入力から行を一行読み込み，fieldsに設定する．
                 * 行の読み込みに成功したらスタックに0をpushし，失敗(EOF)したら1をpushする．
                 */
                Opcode::Readline => op_readline(self, reader),
                Opcode::Print(n) => op_print(self, writer, *n),
            }
            self.pc += 1;
        }
    }

    pub fn print_state(&mut self) {
        println!("Program:");
        println!("{:?}\n", self.program);
        println!("Stack:");
        println!("{:?}", self.stack);
    }
}

fn op_readline<R: BufRead>(vm: &mut VM, reader: &mut R) {
    let mut line = String::new();
    if reader.read_line(&mut line).expect("Failed to read line.") != 0 {
        vm.fields = line.split_whitespace().map(|f| f.to_string()).collect();
        vm.nf = Value::Num(vm.fields.len() as f64);
        vm.stack.push(Value::Num(0.0));
    } else {
        // 読む行がなくなったとき
        vm.stack.push(Value::Num(1.0));
    }
}

fn op_print<W: Write>(vm: &mut VM, writer: &mut W, n: usize) {
    let mut s = false;
    for _ in 0..n {
        write!(
            writer,
            "{}{}",
            if s { " " } else { "" },
            // スタックが空の時はpanicする
            vm.stack.pop().unwrap().to_str()
        ).unwrap();
        s = true;
    }
    writeln!(writer).unwrap();
}

#[derive(Debug)]
pub enum Opcode {
    End,
    Push(Value),
    Pop,
    Jump(usize),
    If(usize),
    // AWK
    Readline,
    Print(usize),
}

#[test]
fn test_vm() {
    use std::str;

    let prg = [
        Opcode::Push(Value::Num(1.0)),
        Opcode::Print(1),
        Opcode::End,
    ];

    let mut vm = VM::new(&prg);

    let mut reader = "".as_bytes();
    let mut writer = Vec::<u8>::new();

    vm.run(&mut reader, &mut writer);

    assert_eq!("1\n", str::from_utf8(&writer).unwrap());
}

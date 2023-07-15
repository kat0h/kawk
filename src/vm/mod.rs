mod value;
use crate::ast::Value;

use std::io;

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

    pub fn run(&mut self) {
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
                /*
                 * Readline
                 * 標準入力から行を一行読み込み，fieldsに設定する．
                 * 行の読み込みに成功したらスタックに0をpushし，失敗(EOF)したら1をpushする．
                 */
                Opcode::Readline => op_readline(self),
            }
            self.pc += 1;
        }
    }
}

fn op_readline(vm: &mut VM) {
    let mut line = String::new();
    if io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line.")
        != 0
    {
        vm.fields = line.split_whitespace().map(|f| f.to_string()).collect();
        vm.nf = Value::Num(vm.fields.len() as f64);
        vm.stack.push(Value::Num(0.0));
    } else {
        // 読む行がなくなったとき
        vm.stack.push(Value::Num(1.0));
    }
}

pub enum Opcode {
    End,
    Push(Value),
    Pop,
    // AWK
    Readline,
}

#[test]
fn test_vm() {}

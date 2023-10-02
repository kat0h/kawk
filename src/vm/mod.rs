mod value;
pub mod ifunc;
use crate::ast::Value;
use crate::ifunc::call_internal_func_from_index;

use std::io::{BufRead, Write};

// Opcodeに項目を追加するときはcompile.rsのOpcodeLも変更
#[derive(Debug, PartialEq)]
pub enum Opcode {
    End,
    Push(Value),
    Pop,
    Jump(usize),
    If(usize),
    NIf(usize),
    Call(usize),
    CallUserFunc(usize),
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
    GetField,
    // Variable
    InitEnv(usize),
    LoadVar(usize),
    SetVar(usize),
}

pub struct VM<'a> {
    program: &'a [Opcode],
    stack: Vec<Value>,
    pc: usize,

    fields: Vec<String>,
    nf: Value,
    env: Vec<Value>,
    retpc: Vec<usize>,
}

// TODO
// unwrap() などでエラーをハンドリングしているところをきちんと伝搬させるようにする

impl VM<'_> {
    pub fn new(program: &[Opcode]) -> VM {
        VM {
            program,
            stack: vec![],
            pc: 0,

            fields: vec![],
            nf: Value::Num(0.0),
            env: vec![],
            retpc: vec![],
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
                //
                // If
                // スタックの一番上がtruetyなとき指定されたポインタにジャンプ
                //
                Opcode::If(pc) => {
                    if self.stack.pop().unwrap().is_true() {
                        self.pc = *pc;
                        continue;
                    }
                }
                //
                // NIf
                // スタックの一番上がfaltyなとき指定されたポインタにジャンプ
                //
                Opcode::NIf(pc) => {
                    if !self.stack.pop().unwrap().is_true() {
                        self.pc = *pc;
                        continue;
                    }
                }
                //
                // Call
                // 内蔵関数を呼び出します．内蔵関数のindexを指定します．
                //
                Opcode::Call(i) => {
                    call_internal_func_from_index(*i, self);
                }

                //
                // CallUserFunc
                // ユーザー定義関数を呼び出します
                // 呼び出すとき現在のプログラムカウンタを環境にpushし，returnで元の位置に戻れるようにします
                // 引数の個数をスタックにpushする
                Opcode::CallUserFunc(i) => {
                    // プログラムカウンタを保存
                    self.retpc.push(self.pc);
                    self.pc = *i;
                    // pcに1を足されると困るのでcontinue(jumpと同じ挙動)
                    continue;
                }

                // 関数呼び出しから復帰します
                // 戻り先pcスタックから一つ取り出し，プログラムカウンタをセットします
                Opcode::Return => {
                    let pc = self.retpc.pop().unwrap();
                    self.pc = pc;
                }

                // 四則演算
                // スタックのトップからR→Lの順に値を取り出し，計算する
                // トップに置かれた数字が右側なのはコンパイルしやすくするため

                // +
                Opcode::Add => {
                    // AWKは一つのオブジェクトに複数の名前が束縛されることはないため，
                    // とくにGCの仕組みを考えたり，メモリリークの心配をしなくていい(はず)
                    // だが，本当に大丈夫なのか
                    let r = self.stack.pop().unwrap();
                    let l = self.stack.pop().unwrap();
                    self.stack.push(l.add(&r));
                }
                // -
                Opcode::Sub => {
                    let r = self.stack.pop().unwrap();
                    let l = self.stack.pop().unwrap();
                    self.stack.push(l.sub(&r));
                }
                // *
                Opcode::Mul => {
                    let r = self.stack.pop().unwrap();
                    let l = self.stack.pop().unwrap();
                    self.stack.push(l.mul(&r));
                }
                // /
                Opcode::Div => {
                    let r = self.stack.pop().unwrap();
                    let l = self.stack.pop().unwrap();
                    self.stack.push(l.div(&r));
                }
                // ^
                Opcode::Pow => {
                    let r = self.stack.pop().unwrap();
                    let l = self.stack.pop().unwrap();
                    self.stack.push(l.pow(&r));
                }
                Opcode::Mod => {
                    let r = self.stack.pop().unwrap();
                    let l = self.stack.pop().unwrap();
                    self.stack.push(l.module(&r));
                }
                Opcode::Cat => {
                    let r = self.stack.pop().unwrap();
                    let l = self.stack.pop().unwrap();
                    self.stack.push(l.concat(&r));
                }
                Opcode::And => {
                    let r = self.stack.pop().unwrap();
                    let l = self.stack.pop().unwrap();
                    self.stack.push(l.and(&r));
                }
                Opcode::Or => {
                    let r = self.stack.pop().unwrap();
                    let l = self.stack.pop().unwrap();
                    self.stack.push(l.or(&r));
                }
                Opcode::LessThan => {
                    let r = self.stack.pop().unwrap();
                    let l = self.stack.pop().unwrap();
                    self.stack.push(l.lessthan(&r));
                }
                Opcode::LessEqualThan => {
                    let r = self.stack.pop().unwrap();
                    let l = self.stack.pop().unwrap();
                    self.stack.push(l.lessequalthan(&r));
                }
                Opcode::NotEqual => {
                    let r = self.stack.pop().unwrap();
                    let l = self.stack.pop().unwrap();
                    self.stack.push(l.notequal(&r));
                }
                Opcode::Equal => {
                    let r = self.stack.pop().unwrap();
                    let l = self.stack.pop().unwrap();
                    self.stack.push(l.equal(&r));
                }
                Opcode::GreaterThan => {
                    let r = self.stack.pop().unwrap();
                    let l = self.stack.pop().unwrap();
                    self.stack.push(l.greaterthan(&r));
                }
                Opcode::GreaterEqualThan => {
                    let r = self.stack.pop().unwrap();
                    let l = self.stack.pop().unwrap();
                    self.stack.push(l.greaterequalthan(&r));
                }

                //
                //  Readline
                //  標準入力から行を一行読み込み，fieldsに設定する．
                //  行の読み込みに成功したらスタックに0をpushし，失敗(EOF)したら1をpushする．
                //
                Opcode::Readline => op_readline(self, reader),
                Opcode::Print(n) => op_print(self, writer, *n),
                Opcode::GetField => op_getfield_n(self),

                //
                // 変数
                //   InitEnv(n): n個分の変数の領域を確保する
                //   LoadVal(n): n番目の変数の値をスタックのトップに積む
                //   SetVal(n): スタックトップの値をn番目の変数に設定する
                Opcode::InitEnv(n) => {
                    self.env = vec![Value::None; *n];
                }
                Opcode::LoadVar(n) => {
                    self.stack.push(self.env[*n].clone());
                }
                Opcode::SetVar(n) => {
                    let val = self.stack.pop().unwrap();
                    self.env[*n] = val;
                }
            }
            self.pc += 1;
        }
    }

    pub fn show_stack_and_env(self) {
        println!("STACK");
        dbg!(&self.stack);
        println!("ENVIRONMENT");
        dbg!(&self.env);
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
    let mut i = vec![];
    for _ in 0..n {
        i.push(vm.stack.pop().unwrap().to_str());
    }
    for o in i.iter().rev() {
        write!(
            writer,
            "{}{}",
            if s { " " } else { "" },
            // スタックが空の時はpanicする
            o
        )
        .unwrap();
        s = true;
    }
    writeln!(writer).unwrap();
}

// スタックトップの値をnとし，$nの値を取得し，スタックのトップに配置する
fn op_getfield_n(vm: &mut VM) {
    let n = vm.stack.pop().unwrap().to_float() as usize;
    if n == 0 {
        vm.stack.push(Value::Str(vm.fields.join(" ")));
    } else if n <= vm.fields.len() {
        vm.stack.push(Value::Str(vm.fields[n - 1].clone()));
    } else {
        vm.stack.push(Value::Str("".to_string()));
    }
}

#[test]
fn test_vm() {
    use std::str;

    let prg = [
        Opcode::Push(Value::Num(1.0)),
        Opcode::Push(Value::Num(2.0)),
        Opcode::Add,
        Opcode::Push(Value::Num(4.0)),
        Opcode::Push(Value::Num(2.0)),
        Opcode::Div,
        Opcode::Print(2),
        Opcode::End,
    ];

    let mut vm = VM::new(&prg);

    let mut reader = "".as_bytes();
    let mut writer = Vec::<u8>::new();

    vm.run(&mut reader, &mut writer);

    assert_eq!("3 2\n", str::from_utf8(&writer).unwrap());
}

#[test]
fn test_vm2() {
    use std::str;

    let prg = [
        Opcode::Readline,
        Opcode::Pop,
        Opcode::Push(Value::Num(2.0)),
        Opcode::GetField,
        Opcode::Print(1),
        Opcode::End,
    ];

    let mut vm = VM::new(&prg);

    let mut reader = "1 2".as_bytes();
    let mut writer = Vec::<u8>::new();

    vm.run(&mut reader, &mut writer);

    assert_eq!("2\n", str::from_utf8(&writer).unwrap());
}

#[test]
fn test_vm3() {
    use std::str;

    let prg = [
        Opcode::InitEnv(1),
        Opcode::Push(Value::Num(44.0)),
        Opcode::SetVar(0),
        Opcode::LoadVar(0),
        Opcode::Print(1),
        Opcode::End,
    ];

    let mut vm = VM::new(&prg);

    let mut reader = "".as_bytes();
    let mut writer = Vec::<u8>::new();

    vm.run(&mut reader, &mut writer);

    assert_eq!("44\n", str::from_utf8(&writer).unwrap());
}

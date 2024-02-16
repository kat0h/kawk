use crate::ast::Value;
use crate::vm::VM;
use rand::prelude::*;
use std::process::Command;
use std::io::{stdout, Write};

pub fn ifunc_sin(vm: &mut VM) {
    let arg = vm.stack.pop().unwrap();
    let ret = Value::Num(arg.to_float().sin());
    vm.stack.push(ret);
}

pub fn ifunc_cos(vm: &mut VM) {
    let arg = vm.stack.pop().unwrap();
    let ret = Value::Num(arg.to_float().cos());
    vm.stack.push(ret);
}

pub fn ifunc_exp(vm: &mut VM) {
    let arg = vm.stack.pop().unwrap();
    let ret = Value::Num(arg.to_float().exp());
    vm.stack.push(ret);
}

pub fn ifunc_tolower(vm: &mut VM) {
    let arg = vm.stack.pop().unwrap();
    let ret = Value::Str(arg.to_str().to_lowercase());
    vm.stack.push(ret);
}

pub fn ifunc_toupper(vm: &mut VM) {
    let arg = vm.stack.pop().unwrap();
    let ret = Value::Str(arg.to_str().to_uppercase());
    vm.stack.push(ret);
}

pub fn ifunc_rand(vm: &mut VM) {
    vm.stack.push(Value::Num(vm.rng.gen()));
}

pub fn ifunc_srand(vm: &mut VM) {
    let arg = vm.stack.pop().unwrap().to_float() as u64;
    vm.rng = rand::SeedableRng::seed_from_u64(arg);
}

pub fn ifunc_sqrt(vm: &mut VM) {
    let arg = vm.stack.pop().unwrap().to_float();
    let ret = arg.sqrt();
    vm.stack.push(Value::Num(ret));
}

pub fn ifunc_log(vm: &mut VM) {
    let arg = vm.stack.pop().unwrap().to_float();
    let ret = arg.ln();
    vm.stack.push(Value::Num(ret));
}

pub fn ifunc_int(vm: &mut VM) {
    let arg = vm.stack.pop().unwrap().to_float();
    let ret = (arg as i64) as f64;
    vm.stack.push(Value::Num(ret));
}

pub fn ifunc_atan2(vm: &mut VM) {
    let x = vm.stack.pop().unwrap().to_float();
    let y = vm.stack.pop().unwrap().to_float();
    let ret = x.atan2(y);
    vm.stack.push(Value::Num(ret));
}

pub fn ifunc_length(vm: &mut VM) {
    let arg = vm.stack.pop().unwrap().to_str();
    let ret = arg.chars().count();
    vm.stack.push(Value::Num(ret as f64));
}

pub fn ifunc_index(vm: &mut VM) {
    let s = vm.stack.pop().unwrap().to_str();
    let t = vm.stack.pop().unwrap().to_str();
    let ret = if let Some(idx) = s.find(&t) {
        s[..idx].chars().count() + 1
    } else {
        0
    };
    vm.stack.push(Value::Num(ret as f64));
}

pub fn ifunc_system(vm: &mut VM) {
    // 現状stdin/outは書き換えられない
    let arg = vm.stack.pop().unwrap().to_str();
    let mut cmd = Command::new("sh")
        .arg("-c")
        .arg(arg)
        .spawn()
        .expect("Internal Error system()");
    let _ = cmd.wait();
}

pub fn ifunc_flush(_vm: &mut VM) {
    stdout().flush().unwrap();
}

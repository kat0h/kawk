use crate::ast::Value;
use crate::vm::VM;
use rand::prelude::*;

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
    let mut rng = rand::thread_rng();
    vm.stack.push(Value::Num(rng.gen()));
}

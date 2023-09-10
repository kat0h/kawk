use crate::ast::Value;
use crate::vm::VM;

pub fn ifunc_sin(vm: &mut VM) {
    let arg = vm.stack.pop().unwrap();
    let ret = Value::Num(arg.to_float().sin());
    vm.stack.push(ret);
}

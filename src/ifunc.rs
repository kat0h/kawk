use crate::vm::VM;
use crate::vm::ifunc;
// 内蔵関数の定義


type Func = fn(vm: &mut VM);
struct IFunc {
    name: &'static str,
    func: Func,
    arglen: usize
}

const INTERNAL_FUNC: &[IFunc] = &[
    IFunc { name: "sin", func: ifunc::ifunc_sin, arglen: 1 },
    IFunc { name: "cos", func: ifunc::ifunc_cos, arglen: 1 },
    IFunc { name: "exp", func: ifunc::ifunc_exp, arglen: 1 },
];

pub fn get_index_from_name(name: &str) -> Option<usize> {
   INTERNAL_FUNC.iter().position(|i| i.name == name)
}

pub fn call_internal_func_from_index(index: usize, vm: &mut VM) {
    (INTERNAL_FUNC[index].func)(vm);
}

pub fn get_len_of_args(index: usize) -> usize {
    INTERNAL_FUNC[index].arglen
}

#[test]
fn test_index_from_name() {
    assert_eq!(0, get_index_from_name("sin").unwrap());
}

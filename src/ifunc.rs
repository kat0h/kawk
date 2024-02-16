use crate::vm::ifunc;
use crate::vm::VM;
// 内蔵関数の定義

// gsub(ere, repl[, in])
// match(s, ere)
// split(s, a[, fs  ])
// sprintf(fmt, expr, expr, ...)
// sub(ere, repl[, in  ])
// substr(s, m[, n  ])
// tolower(s)
// toupper(s)
// close(expression)
// expression |  getline [var]
// getline
// getline  var
// getline [var]  < expression
// system(expression)

type Func = fn(vm: &mut VM);
struct IFunc {
    name: &'static str,
    func: Func,
    arglen: usize,
}

const INTERNAL_FUNC: &[IFunc] = &[
    IFunc {
        name: "sin",
        func: ifunc::ifunc_sin,
        arglen: 1,
    },
    IFunc {
        name: "cos",
        func: ifunc::ifunc_cos,
        arglen: 1,
    },
    IFunc {
        name: "exp",
        func: ifunc::ifunc_exp,
        arglen: 1,
    },
    IFunc {
        name: "tolower",
        func: ifunc::ifunc_tolower,
        arglen: 1,
    },
    IFunc {
        name: "toupper",
        func: ifunc::ifunc_toupper,
        arglen: 1,
    },
    IFunc {
        name: "rand",
        func: ifunc::ifunc_rand,
        arglen: 0,
    },
    IFunc {
        name: "sqrt",
        func: ifunc::ifunc_sqrt,
        arglen: 1,
    },
    IFunc {
        name: "log",
        func: ifunc::ifunc_log,
        arglen: 1,
    },
    IFunc {
        name: "int",
        func: ifunc::ifunc_int,
        arglen: 1,
    },
    // 引数をオプショナルに
    IFunc {
        name: "srand",
        func: ifunc::ifunc_srand,
        arglen: 1,
    },
    IFunc {
        name: "atan2",
        func: ifunc::ifunc_atan2,
        arglen: 2,
    },
    // 引数をオプショナルに
    IFunc {
        name: "length",
        func: ifunc::ifunc_length,
        arglen: 1,
    },
    IFunc {
        name: "index",
        func: ifunc::ifunc_index,
        arglen: 2,
    },
    IFunc {
        name: "system",
        func: ifunc::ifunc_system,
        arglen: 1,
    },
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

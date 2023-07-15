mod value;

pub struct VM<'a> {
    program: &'a [Opcode],
    stack: Vec<i64>,
    pc: usize,
}

impl VM<'_> {
    pub fn new(program: &[Opcode]) -> VM {
        VM {
            program,
            stack: vec![],
            pc: 0,
        }
    }
}

pub fn vm_run(vm: &mut VM) {
    loop {
        match vm.program[vm.pc] {
            Opcode::End => {
                break;
            }
            Opcode::Push(a) => {
                vm.stack.push(a);
            }
            Opcode::Pop => {
                vm.stack.pop();
            }
            Opcode::Readline => {

            }
        }
        vm.pc += 1;
    }
}

pub enum Opcode {
    End,
    Push(i64),
    Pop,
    //
    Readline,
}

#[test]
fn test_vm() {
}

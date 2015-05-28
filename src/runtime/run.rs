use runtime::bytecode::*;
use runtime::function::*;


pub fn run<'a>(function: &Function<'a>, mut locals: Vec<u64>) {
    let insts = &function.instructions;
    let inst_count = function.instructions.len();
    let mut inst_index = 0;

    // Exclusive.
    let mut stack_top = 0;

    // 8 byte aligned stack.
    let mut stack: [u64; 16] = [0; 16];

    while inst_index < inst_count {
        let inst = &insts[inst_index];
        match *inst {
            Instruction::Nop => println!("nop!"),

            Instruction::Pop => stack_top -= 1,
            Instruction::Dup => {
                stack[stack_top] = stack[stack_top - 1];
                stack_top += 1;
            },

            Instruction::Cst(ref index) => {
                let constant = &function.constant_table.table[*index as usize];
                match constant {
                    &Constant::U64(num) => {
                        stack[stack_top] = num;
                        stack_top += 1;
                    },
                    _ => panic!(format!("Constant at index {} must be a number!", index)),
                }
            },

            Instruction::Load(ref var) => {
                stack[stack_top] = locals[*var as usize];
                stack_top += 1;
            },

            Instruction::Store(ref var) => {
                stack_top -= 1;
                locals[*var as usize] = stack[stack_top];
            },

            Instruction::Add(ref t) => {
                let left = stack_top - 2;
                let right = stack_top - 1;

                match *t {
                    Type::U64 => stack[left] = stack[left] + stack[right],
                    _ => panic!("Unsupported type!"),
                }

                stack_top = right;
            },

            Instruction::Sub(ref t) => {
                let left = stack_top - 2;
                let right = stack_top - 1;
                stack[left] = stack[left] - stack[right];
                stack_top = right;
            },

            Instruction::Mul(ref t) => {
                let left = stack_top - 2;
                let right = stack_top - 1;
                stack[left] = stack[left] * stack[right];
                stack_top = right;
            },

            Instruction::Div(ref t) => {
                let left = stack_top - 2;
                let right = stack_top - 1;
                stack[left] = stack[left] / stack[right];
                stack_top = right;
            },

            Instruction::Print(ref t) => {
                println!("{}", stack[stack_top - 1]);
                stack_top -= 1;
            },

            // _ => panic!(format!("Instruction at index {} not implemented!", inst_index))
        }
        inst_index += 1;
    }
}

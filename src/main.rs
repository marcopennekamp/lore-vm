#![feature(test)]
#![feature(alloc)]

mod runtime;
mod tests;

use runtime::bytecode::*;
use runtime::context::*;
use runtime::function::*;


fn main() {
    let inc_and_print_instructions = vec![
        Instruction::Load(0),
        Instruction::Cst(0),
        Instruction::Add(Type::I64),
        Instruction::Store(0),
        Instruction::Load(0),
        Instruction::Cst(1),
        Instruction::Mul(Type::I64),
        Instruction::Print(Type::I64),
    ];

    let inc_and_print_constants = ConstantTable::new(vec![
        Constant::I64(-25),
        Constant::I64(20),
    ]);

    let inc_and_print = Function::new(
        inc_and_print_constants,
        inc_and_print_instructions,
    );

    for instruction in &inc_and_print.instructions {
        println!("{:?}", instruction);
    }

    let context = Context::new(1024);
    context.set_local(0, 5);

    context.run(&inc_and_print, 0)
}

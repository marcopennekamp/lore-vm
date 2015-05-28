mod runtime;

use runtime::bytecode::*;
use runtime::function::*;
use runtime::run;


fn main() {
    let inc_and_print_instructions = vec![
        Instruction::Load(0),
        Instruction::Cst(0),
        Instruction::Add(Type::U64),
        Instruction::Store(0),
        Instruction::Load(0),
        Instruction::Dup,
        Instruction::Mul(Type::U64),
        Instruction::Print(Type::U64),
    ];

    let inc_and_print_constants = ConstantTable::new(vec![
        Constant::U64(10),
    ]);

    let inc_and_print = Function::new(
        inc_and_print_constants,
        inc_and_print_instructions,
    );

    for instruction in &inc_and_print.instructions {
        println!("{:?}", instruction);
    }

    run::run(&inc_and_print, vec![5])
}

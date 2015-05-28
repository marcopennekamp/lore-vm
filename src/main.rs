mod runtime;

use runtime::bytecode::*;
use runtime::function::*;
use runtime::run;


fn main() {
    let inc_and_print_instructions = vec![
        Instruction::Load(FormatVariable::new(0)),
        Instruction::Cst(0),
        Instruction::Add(FormatTypedOp::new(Type::U64)),
        Instruction::Store(FormatVariable::new(0)),
        Instruction::Load(FormatVariable::new(0)),
        Instruction::Dup,
        Instruction::Mul(FormatTypedOp::new(Type::U64)),
        Instruction::Print(FormatTypedOp::new(Type::U64)),
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

    run::run(&inc_and_print, vec![3])
}

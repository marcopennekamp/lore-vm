#![feature(test)]

extern crate test;
extern crate lore;

use lore::runtime::bytecode::*;
use lore::runtime::context::*;
use lore::runtime::function::*;
use lore::runtime::environment::*;


#[test]
fn inc_and_print() {
    let inc_and_print_instructions = vec![
        Instruction::Load(0),
        Instruction::Cst(0),
        Instruction::Add(Type::I64),
        Instruction::Cst(1),
        Instruction::Mul(Type::I64),
        Instruction::Dup,
        Instruction::Print(Type::I64),
        Instruction::Ret(1),
    ];


    for instruction in &inc_and_print_instructions {
        println!("{:?}", instruction);
    }

    let inc_and_print_constants = ConstantTable::new(vec![
        Constant::I64(-25),
        Constant::I64(20),
    ]);

    let inc_and_print = Function::new(
        "inc_and_print".to_string(),
        Sizes::new(1, 1, 2, 2),
        inc_and_print_constants,
        Instructions::Bytecode(inc_and_print_instructions),
    );

    let mut environment = Environment::new();
    let inc_and_print_ref = environment.register_function(inc_and_print);

    let context = Context::new(1024);
    let arguments = vec![5];
    let results = context.run(inc_and_print_ref, &arguments);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0] as i64, -400);
}

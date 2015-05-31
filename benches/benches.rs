#![feature(test)]

extern crate test;
extern crate lore;

use self::test::Bencher;

use lore::runtime::bytecode::*;
use lore::runtime::context::*;
use lore::runtime::function::*;


#[bench]
fn add_cst_repeat(bencher: &mut Bencher) {
    let add_cst_repeat_instructions = vec![
        Instruction::Load(0),
        Instruction::Cst(0),
        Instruction::Add(Type::U64),
        Instruction::Cst(0),
        Instruction::Add(Type::U64),
        Instruction::Cst(0),
        Instruction::Add(Type::U64),
        Instruction::Cst(0),
        Instruction::Add(Type::U64),
        Instruction::Cst(0),
        Instruction::Add(Type::U64),
        Instruction::Cst(0),
        Instruction::Add(Type::U64),
        Instruction::Cst(0),
        Instruction::Add(Type::U64),
        Instruction::Cst(0),
        Instruction::Add(Type::U64),
        Instruction::Cst(0),
        Instruction::Add(Type::U64),
        Instruction::Cst(0),
        Instruction::Add(Type::U64),
    ];

    let add_cst_repeat_constants = ConstantTable::new(vec![
        Constant::U64(20),
    ]);

    let add_cst_repeat = Function::new(
        add_cst_repeat_constants,
        add_cst_repeat_instructions,
        1,
    );

    let context = Context::new(1024);
    let arguments = vec![5];
    bencher.iter(|| {
        context.run(&add_cst_repeat, &arguments);
    });
}

#[bench]
fn dup_repeat(bencher: &mut Bencher) {
    let dup_repeat_instructions = vec![
        Instruction::Load(0),
        Instruction::Dup,
        Instruction::Dup,
        Instruction::Dup,
        Instruction::Dup,
        Instruction::Dup,
        Instruction::Dup,
        Instruction::Dup,
        Instruction::Dup,
        Instruction::Dup,
        Instruction::Dup,
        Instruction::Dup,
        Instruction::Dup,
        Instruction::Dup,
        Instruction::Dup,
        Instruction::Dup,
        Instruction::Dup,
        Instruction::Dup,
        Instruction::Dup,
        Instruction::Dup,
        Instruction::Dup,
    ];

    let dup_repeat_constants = ConstantTable::new(vec![
        Constant::U64(20),
    ]);

    let dup_repeat = Function::new(
        dup_repeat_constants,
        dup_repeat_instructions,
        1,
    );

    let context = Context::new(1024);
    let arguments = vec![5];
    bencher.iter(|| {
        context.run(&dup_repeat, &arguments);
    });
}

#[bench]
fn nop_repeat(bencher: &mut Bencher) {
    let nop_repeat_instructions = vec![
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Nop,
        Instruction::Nop,
    ];

    let nop_repeat = Function::new(
        ConstantTable::new(vec![]),
        nop_repeat_instructions,
        0,
    );

    let context = Context::new(1024);
    let arguments = vec![];
    bencher.iter(|| {
        context.run(&nop_repeat, &arguments);
    });
}

#[bench]
fn overhead(bencher: &mut Bencher) {
    let do_nothing = Function::new(
        ConstantTable::new(vec![]),
        vec![],
        0,
    );

    let context = Context::new(1024);
    let arguments = vec![];
    bencher.iter(|| {
        context.run(&do_nothing, &arguments);
    });
}

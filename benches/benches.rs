#![feature(test)]

extern crate test;
extern crate lore;

use self::test::Bencher;

use lore::runtime::bytecode::*;
use lore::runtime::context::*;
use lore::runtime::function::*;
use lore::runtime::environment::*;


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
        "add_cst_repeat".to_string(),
        add_cst_repeat_constants,
        add_cst_repeat_instructions,
        1,
    );

    let mut environment = Environment::new();
    let add_cst_repeat_ref = environment.register_function(add_cst_repeat);

    let context = Context::new(1024);
    let arguments = vec![5];
    bencher.iter(|| {
        context.run(add_cst_repeat_ref, &arguments);
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
        "dup_repeat".to_string(),
        dup_repeat_constants,
        dup_repeat_instructions,
        1,
    );

    let mut environment = Environment::new();
    let dup_repeat_ref = environment.register_function(dup_repeat);

    let context = Context::new(1024);
    let arguments = vec![5];
    bencher.iter(|| {
        context.run(dup_repeat_ref, &arguments);
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
        "nop_repeat".to_string(),
        ConstantTable::new(vec![]),
        nop_repeat_instructions,
        0,
    );

    let mut environment = Environment::new();
    let nop_repeat_ref = environment.register_function(nop_repeat);

    let context = Context::new(1024);
    let arguments = vec![];
    bencher.iter(|| {
        context.run(nop_repeat_ref, &arguments);
    });
}

#[bench]
fn overhead(bencher: &mut Bencher) {
    let do_nothing = Function::new(
        "do_nothing".to_string(),
        ConstantTable::new(vec![]),
        vec![],
        0,
    );

    let mut environment = Environment::new();
    let do_nothing_ref = environment.register_function(do_nothing);

    let context = Context::new(1024);
    let arguments = vec![];
    bencher.iter(|| {
        context.run(do_nothing_ref, &arguments);
    });
}

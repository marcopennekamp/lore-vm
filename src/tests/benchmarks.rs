extern crate test;

use self::test::Bencher;

use runtime::bytecode::*;
use runtime::context::*;
use runtime::function::*;


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
    );

    let context = Context::new(1024);
    bencher.iter(|| {
        context.set_local(0, 5);
        context.run(&add_cst_repeat);
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
    );

    let context = Context::new(1024);
    bencher.iter(|| {
        context.set_local(0, 5);
        context.run(&dup_repeat);
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
    );

    let context = Context::new(1024);
    bencher.iter(|| {
        context.run(&nop_repeat);
    });
}

#[bench]
fn overhead(bencher: &mut Bencher) {
    let do_nothing = Function::new(
        ConstantTable::new(vec![]),
        vec![],
    );

    let context = Context::new(1024);
    bencher.iter(|| {
        context.run(&do_nothing);
    });
}

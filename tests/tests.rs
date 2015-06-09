#![feature(test)]

extern crate test;
extern crate lore;

use std::fs::File;
use std::path::Path;

use lore::bytecode::*;
use lore::context::*;
use lore::function::*;
use lore::environment::*;
use lore::scribe::*;


#[test]
fn inc_and_print() {
    let mut environment = Environment::new();

    let inc_and_print = Function::from_file(&mut environment, Path::new("produced")).unwrap();
    let id = environment.register_function(inc_and_print);
    let inc_and_print_ref = environment.fetch_function_by_id(id);

    let context = Context::new(1024);
    let arguments = vec![5];
    let results = context.run(inc_and_print_ref, &arguments);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0] as i64, -400);
}

#[test]
fn write_inc_and_print() {
    let mut file = File::create("produced.code").unwrap();
    let mut writer = InstructionWriter::new(&mut file);

    writer.write_load(0);
    writer.write_cst(0);
    writer.write_typed(Opcode::Add, Type::I64);
    writer.write_cst(1);
    writer.write_typed(Opcode::Mul, Type::I64);
    writer.write_operation(Opcode::Dup);
    writer.write_typed(Opcode::Print, Type::I64);
    writer.write_ret(1);

    writer.finish();

    let mut sizes = writer.sizes;
    sizes.argument_count = 1;

    let constant_table = ConstantTable::new(vec![
        Constant::I64(-25),
        Constant::I64(20),
    ]);

    let mut cst_file = File::create("table_0.cst").unwrap();
    let mut cst_writer = ConstantTableWriter::new(&mut cst_file);
    cst_writer.write_constant_table(&constant_table);

    let mut function_file = File::create("produced.info").unwrap();
    let mut function_writer = FunctionWriter::new(&mut function_file);
    function_writer.write_function("inc_and_print", &sizes, "table_0");
}

use std::fmt;

use runtime::bytecode;


pub enum Constant<'a> {
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Str(&'a String),
}

pub struct ConstantTable<'a> {
    pub table: Vec<Constant<'a>>,
}

pub struct Function<'a> {
    pub constant_table: ConstantTable<'a>,
    pub instructions: Vec<bytecode::Instruction>,

    /// The amount of stack elements that are returned from the function.
    pub return_count: u8,

    /// The expected amount of arguments.
    pub argument_count: u8,

    /// The size that the operand stack needs to be.
    pub stack_size: usize,

    /// The amount of slots for variables.
    pub locals_size: usize,
}


impl<'a> ConstantTable<'a> {
    pub fn new(table: Vec<Constant<'a>>) -> ConstantTable<'a> {
        ConstantTable { table: table }
    }
}

impl<'a> Function<'a> {
    pub fn new(constant_table: ConstantTable<'a>,
           instructions: Vec<bytecode::Instruction>,
           argument_count: u8) -> Function<'a> {
        let (stack_size, locals_size, return_count) = bytecode::calculate_sizes(&instructions);
        if stack_size < 0 {
            panic!("Calculated stack size is less than 0!");
        }

        Function {
            constant_table: constant_table,
            instructions: instructions,
            return_count: return_count,
            argument_count: argument_count,
            stack_size: stack_size as usize,
            locals_size: locals_size,
        }
    }
}


impl<'a> fmt::Debug for Constant<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Constant::I32(ref num) => write!(f, "i32: {}", num),
            Constant::I64(ref num) => write!(f, "i64: {}", num),
            Constant::U32(ref num) => write!(f, "u32: {}", num),
            Constant::U64(ref num) => write!(f, "u64: {}", num),
            Constant::F32(ref num) => write!(f, "f32: {}", num),
            Constant::F64(ref num) => write!(f, "f64: {}", num),
            Constant::Str(ref val) => write!(f, "str: '{}'", val),
        }
    }
}

use std::fs::File;
use std::io::{Read, BufReader};

use byteorder::{BigEndian, ReadBytesExt};

use io;
use bytecode::*;


pub const INVALID_FUNCTION_ID: u32 = 0xFFFFFFFF;

pub struct ConstantTable {
    pub table: Vec<Constant>,
}

pub enum Instructions {
    FilePath(String),
    Bytecode(Vec<Instruction>),
}

pub struct Sizes {
    /// The amount of stack elements that are returned from the function.
    pub return_count: u8,

    /// The expected amount of arguments.
    pub argument_count: u8,

    /// The amount of locals that the function needs.
    /// Includes the argument count.
    pub locals_count: u16,

    /// The maximum size that the operand stack needs to be.
    pub max_operands: u16,
}

pub struct Function {
    /// The ID of the function in the current environment.
    pub id: u32,

    /// The unique name of the function.
    pub name: String,

    pub sizes: Sizes,

    pub constant_table: ConstantTable,

    pub instructions: Instructions,
}


impl ConstantTable {
    pub fn new(table: Vec<Constant>) -> ConstantTable {
        ConstantTable { table: table }
    }

    pub fn from_read(read: &mut Read) -> ConstantTable {
        let table_size = read.read_u16::<BigEndian>().unwrap();
        let mut table: Vec<Constant> = Vec::with_capacity(table_size as usize);

        for _ in 0..table_size {
            let constant = Constant::from_read(read);
            table.push(constant);
        }

        ConstantTable { table: table }
    }
}

impl Instructions {
    pub fn from_file(path: &str) -> Instructions {
        let file = File::open(path).unwrap();
        let mut read = BufReader::new(file);

        // Read instruction count.
        let count = read.read_u32::<BigEndian>().unwrap() as usize;
        let mut instructions = Vec::with_capacity(count);

        for _ in 0..count {
            let instruction = Instruction::from_read(&mut read);
            instructions.push(instruction);
        }

        Instructions::Bytecode(instructions)
    }
}

impl Sizes {
    pub fn new(return_count: u8, argument_count: u8,
            locals_count: u16, max_operands: u16) -> Sizes {
        Sizes {
            return_count: return_count,
            argument_count: argument_count,
            locals_count: locals_count,
            max_operands: max_operands,
        }
    }

    pub fn from_read(read: &mut Read) -> Sizes {
        let return_count = read.read_u8().unwrap();
        let argument_count = read.read_u8().unwrap();
        let locals_count = read.read_u16::<BigEndian>().unwrap();
        let max_operands = read.read_u16::<BigEndian>().unwrap();
        Sizes {
            return_count: return_count,
            argument_count: argument_count,
            locals_count: locals_count,
            max_operands: max_operands,
        }
    }
}

impl Function {
    pub fn from_file(path: &str) -> Function {
        let file = File::open(path.to_string() + ".info").unwrap();
        let mut read = BufReader::new(file);

        // Read name.
        let name = io::read_string(&mut read);

        // Read sizes.
        let sizes = Sizes::from_read(&mut read);

        // Read constant table.
        let constant_table = ConstantTable::from_read(&mut read);

        Function {
            id: INVALID_FUNCTION_ID,
            name: name,
            sizes: sizes,
            constant_table: constant_table,
            instructions: Instructions::FilePath(path.to_string() + ".code"),
        }
    }

    pub fn new(name: String, sizes: Sizes,
           constant_table: ConstantTable,
           instructions: Instructions) -> Function {
        Function {
            id: INVALID_FUNCTION_ID,
            name: name,
            sizes: sizes,
            constant_table: constant_table,
            instructions: instructions,
        }
    }
}

use std::fs::File;
use std::io::{Read, BufReader, Result};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use byteorder::{BigEndian, ReadBytesExt};

use bytecode::*;
use environment::Environment;
use io;


pub const INVALID_FUNCTION_ID: u32 = 0xFFFFFFFF;

pub struct ConstantTable {
    pub table: Vec<Constant>,
}

pub enum Instructions {
    FilePath(PathBuf),
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

    pub constant_table: Arc<ConstantTable>,

    pub instructions: Instructions,
}


impl ConstantTable {
    pub fn new(table: Vec<Constant>) -> ConstantTable {
        ConstantTable { table: table }
    }

    pub fn from_file(path: &Path) -> Result<ConstantTable> {
        let mut with_extension = PathBuf::from(path);
        with_extension.set_extension("cst");

        let file = try!(File::open(with_extension.as_path()));
        let mut read = BufReader::new(file);

        ConstantTable::from_read(&mut read)
    }

    pub fn from_read(read: &mut Read) -> Result<ConstantTable> {
        let table_size = try!(read.read_u16::<BigEndian>());
        let mut table: Vec<Constant> = Vec::with_capacity(table_size as usize);

        for _ in 0..table_size {
            let constant = try!(Constant::from_read(read));
            table.push(constant);
        }

        Ok(ConstantTable { table: table })
    }
}

impl Instructions {
    pub fn from_file(path: &Path) -> Result<Instructions> {
        let mut with_extension = PathBuf::from(path);
        with_extension.set_extension("code");

        let file = try!(File::open(with_extension.as_path()));
        let mut read = BufReader::new(file);

        // Read instruction count.
        let count = try!(read.read_u32::<BigEndian>()) as usize;
        let mut instructions = Vec::with_capacity(count);

        for _ in 0..count {
            let instruction = try!(Instruction::from_read(&mut read));
            instructions.push(instruction);
        }

        Ok(Instructions::Bytecode(instructions))
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

    pub fn from_read(read: &mut Read) -> Result<Sizes> {
        let return_count = try!(read.read_u8());
        let argument_count = try!(read.read_u8());
        let locals_count = try!(read.read_u16::<BigEndian>());
        let max_operands = try!(read.read_u16::<BigEndian>());
        Ok(
            Sizes {
                return_count: return_count,
                argument_count: argument_count,
                locals_count: locals_count,
                max_operands: max_operands,
            }
        )
    }
}

impl Function {
    pub fn from_file(environment: &mut Environment, path: &Path) -> Result<Function> {
        let mut with_extension = PathBuf::from(path);
        with_extension.set_extension("info");

        let file = try!(File::open(with_extension.as_path()));
        let mut read = BufReader::new(file);

        // Read name.
        let name = try!(io::read_string(&mut read));

        // Read sizes.
        let sizes = try!(Sizes::from_read(&mut read));

        // Read constant table name.
        let constant_table_name = try!(io::read_string(&mut read));

        // Fetch constant table.
        let mut cst_path = PathBuf::from(path.parent().unwrap_or(Path::new("")));
        cst_path.push(constant_table_name);
        let constant_table = environment.fetch_constant_table(cst_path.as_path());

        Ok(
            Function {
                id: INVALID_FUNCTION_ID,
                name: name,
                sizes: sizes,
                constant_table: constant_table,
                instructions: Instructions::FilePath(PathBuf::from(path)),
            }
        )
    }

    pub fn new(name: String, sizes: Sizes,
           constant_table: Arc<ConstantTable>,
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

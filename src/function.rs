use std::fs::File;
use std::io::{Read, BufReader, Result};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::mem;

use byteorder::{BigEndian, ReadBytesExt};

use bytecode::*;
use cst::ConstantTable;
use environment::Environment;
use io;


pub const INVALID_FUNCTION_ID: u32 = 0xFFFFFFFF;

pub struct Function {
    /// The ID of the function in the current environment.
    pub id: u32,

    /// The unique name of the function.
    pub name: String,

    pub sizes: Sizes,

    pub constant_table: Arc<ConstantTable>,

    pub instructions: Instructions,
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

pub enum Instructions {
    File {
        path: PathBuf,
        offset: u64,
    },
    Bytecode(Vec<Instruction>),
}


impl Function {
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

    // TODO Split into from_file and from_read.
    pub fn from_file(environment: &mut Environment, path: &Path) -> Result<Function> {
        let mut read = try!(Function::open_reader(path));

        // Read name.
        let name = try!(io::read_string(&mut read));

        // Read sizes.
        let sizes = try!(Sizes::from_read(&mut read));

        // Read constant table name.
        let constant_table_name = try!(io::read_string(&mut read));

        // Calculate offset in file.
        let file_offset = Function::calculate_instructions_offset(&name[..], &constant_table_name[..]);

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
                instructions: Instructions::File {
                    path: PathBuf::from(path),
                    offset: file_offset,
                },
            }
        )
    }

    pub fn open_reader(path: &Path) -> Result<BufReader<File>> {
        let mut with_extension = PathBuf::from(path);
        with_extension.set_extension("func");
        let file = try!(File::open(with_extension.as_path()));
        Ok(BufReader::new(file))
    }

    pub fn calculate_instructions_offset(name: &str, constant_table_name: &str) -> u64 {
        let mut offset = io::string_disk_size(name);
        offset += Sizes::disk_size();
        offset += io::string_disk_size(constant_table_name);
        offset as u64
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

    pub fn disk_size() -> usize {
        return mem::size_of::<u8>() * 2 + mem::size_of::<u16>() * 2;
    }
}

impl Instructions {
    pub fn from_read(read: &mut Read) -> Result<Instructions> {
        // Read instruction count.
        let count = try!(read.read_u32::<BigEndian>()) as usize;
        let mut instructions = Vec::with_capacity(count);

        for _ in 0..count {
            let instruction = try!(Instruction::from_read(read));
            instructions.push(instruction);
        }

        Ok(Instructions::Bytecode(instructions))
    }
}

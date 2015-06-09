use std::io::{Read, BufReader, Result};
use std::path::{Path, PathBuf};
use std::fs::File;

use byteorder::{BigEndian, ReadBytesExt};

use bytecode::Constant;


pub struct ConstantTable {
    pub table: Vec<Constant>,
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

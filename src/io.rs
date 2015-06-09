use std::io::{Read, Write, Result};
use std::mem;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};


pub fn read_string(read: &mut Read) -> Result<String> {
    let string_length = try!(read.read_u16::<BigEndian>()) as usize;
    let mut string_bytes = Vec::with_capacity(string_length);
    unsafe {
        string_bytes.set_len(string_length);
    }

    let result = try!(read.read(&mut string_bytes[..]));
    if result != string_length {
        panic!("Read length {} does not equal the string length {}.",
                result, string_length);
    }

    Ok(String::from_utf8(string_bytes).unwrap())
}

pub fn write_string(write: &mut Write, string: &str) -> Result<()> {
    try!(write.write_u16::<BigEndian>(string.len() as u16));
    try!(write.write_all(string.as_bytes()));
    Ok(())
}

pub fn string_disk_size(string: &str) -> usize {
    return mem::size_of::<u16>() + string.len();
}

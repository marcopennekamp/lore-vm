use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};


pub fn read_string(read: &mut Read) -> String {
    let string_length = read.read_u16::<BigEndian>().unwrap() as usize;
    let mut string_bytes = Vec::with_capacity(string_length);

    let result = read.read(&mut string_bytes[..]);
    if !result.is_ok() {
        panic!("Could not read string!");
    }

    if result.unwrap() != string_length {
        panic!("Read length does not equal the string length!")
    }

    String::from_utf8(string_bytes).unwrap()
}

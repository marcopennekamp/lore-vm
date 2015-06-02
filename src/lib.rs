#![feature(alloc)]

extern crate byteorder;

#[macro_use]
extern crate enum_primitive;

extern crate num;

pub mod bytecode;
pub mod context;
pub mod environment;
pub mod function;
pub mod io;

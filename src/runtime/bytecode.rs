use std::fmt;
use std::cmp;


pub enum Type {
    I8      = 0x0,
    I16     = 0x1,
    I32     = 0x2,
    I64     = 0x3,
    U8      = 0x4,
    U16     = 0x5,
    U32     = 0x6,
    U64     = 0x7,
    F32     = 0x8,
    F64     = 0x9,
    Ptr     = 0xA,
    Void    = 0xB,
}

pub type VariableIndex = u16;
pub type ConstantTableIndex = u16;

pub enum Instruction {
    Nop,
    Pop,
    Dup,
    Cst(ConstantTableIndex),
    Load(VariableIndex),
    Store(VariableIndex),
    Add(Type),
    Sub(Type),
    Mul(Type),
    Div(Type),
    Ret(u8), // u8: The number of elements on the stack that are returned.
    Print(Type),
}

pub enum Constant<'a> {
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    Str(&'a String),
}


/// Return 1: The maximum amount of values at the same time that are on the operand stack.
/// Return 2: The minimum size of the locals array. Depending on the bytecode, possibly not optimal.
/// Return 3: The maximum amount of values that are returned by the Ret(u8) instruction.
pub fn calculate_sizes(instructions: &Vec<Instruction>) -> (i32, u16, u8) {
    let mut size: i32 = 0;
    let mut highest_var: i32 = -1;
    let mut return_count: u8 = 0;
    for inst in instructions.iter() {
        match *inst {
            Instruction::Nop => { },
            Instruction::Pop => size -= 1,
            Instruction::Dup => size += 1,
            Instruction::Cst(..) => size += 1,
            Instruction::Load(var) => {
                size += 1;
                highest_var = cmp::max(highest_var, var as i32);
            },
            Instruction::Store(var) => {
                size -= 1;
                highest_var = cmp::max(highest_var, var as i32);
            },
            Instruction::Add(..) => size -= 1,
            Instruction::Sub(..) => size -= 1,
            Instruction::Mul(..) => size -= 1,
            Instruction::Div(..) => size -= 1,
            Instruction::Print(..) => size -= 1,
            Instruction::Ret(ref count) => {
                size -= *count as i32;
                if *count > return_count {
                    return_count = *count;
                }
            },
        }
    }
    (size, (highest_var + 1) as u16, return_count)
}


impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::I8   => write!(f, "i8"),
            Type::I16  => write!(f, "i16"),
            Type::I32  => write!(f, "i32"),
            Type::I64  => write!(f, "i64"),
            Type::U8   => write!(f, "u8"),
            Type::U16  => write!(f, "u16"),
            Type::U32  => write!(f, "u32"),
            Type::U64  => write!(f, "u64"),
            Type::F32  => write!(f, "f32"),
            Type::F64  => write!(f, "f64"),
            Type::Ptr  => write!(f, "ptr"),
            Type::Void => write!(f, "void"),
        }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::Nop => write!(f, "nop"),
            Instruction::Pop => write!(f, "pop"),
            Instruction::Dup => write!(f, "dup"),
            Instruction::Cst(ref index) => write!(f, "cst #{:?}", index),
            Instruction::Load(ref var) => write!(f, "load ${:?}", var),
            Instruction::Store(ref var) => write!(f, "store ${:?}", var),
            Instruction::Add(ref t) => write!(f, "add[{:?}]", t),
            Instruction::Sub(ref t) => write!(f, "sub[{:?}]", t),
            Instruction::Mul(ref t) => write!(f, "mul[{:?}]", t),
            Instruction::Div(ref t) => write!(f, "div[{:?}]", t),
            Instruction::Ret(ref count) => write!(f, "ret({:?})", count),
            Instruction::Print(ref t) => write!(f, "print[{:?}]", t),
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

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
    Print(Type),
}


/// Calculates the minimum (and optimal) size of the operand stack.
/// Also calculates the minimum (possibly not optimal) size of the locals array.
pub fn calculate_sizes(instructions: &Vec<Instruction>) -> (i32, usize) {
    let mut size = 0;
    let mut highest_var: isize = -1;
    for inst in instructions.iter() {
        match *inst {
            Instruction::Pop => size -= 1,
            Instruction::Dup => size += 1,
            Instruction::Cst(..) => size += 1,
            Instruction::Load(var) => {
                size += 1;
                highest_var = cmp::max(highest_var, var as isize);
            },
            Instruction::Store(var) => {
                size -= 1;
                highest_var = cmp::max(highest_var, var as isize);
            },
            Instruction::Add(..) => size -= 1,
            Instruction::Sub(..) => size -= 1,
            Instruction::Mul(..) => size -= 1,
            Instruction::Div(..) => size -= 1,
            Instruction::Print(..) => size -= 1,
            _ => {
                // No change in size.
            }
        }
    }
    (size, (highest_var + 1) as usize)
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
            Instruction::Print(ref t) => write!(f, "print[{:?}]", t),
        }
    }
}

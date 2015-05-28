use std::fmt;


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

pub struct FormatTypedOp {
    pub t: Type,
}

pub struct FormatVariable {
    pub var: VariableIndex,
}


pub enum Instruction {
    Nop,
    Pop,
    Dup,
    Cst(ConstantTableIndex),
    Load(FormatVariable),
    Store(FormatVariable),
    Add(FormatTypedOp),
    Sub(FormatTypedOp),
    Mul(FormatTypedOp),
    Div(FormatTypedOp),
    Print(FormatTypedOp),
}


impl FormatTypedOp {
    pub fn new(t: Type) -> FormatTypedOp {
        FormatTypedOp { t: t }
    }
}

impl FormatVariable {
    pub fn new(var: VariableIndex) -> FormatVariable {
        FormatVariable { var: var }
    }
}


impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Type::I8   => write!(f, "i8"),
            &Type::I16  => write!(f, "i16"),
            &Type::I32  => write!(f, "i32"),
            &Type::I64  => write!(f, "i64"),
            &Type::U8   => write!(f, "u8"),
            &Type::U16  => write!(f, "u16"),
            &Type::U32  => write!(f, "u32"),
            &Type::U64  => write!(f, "u64"),
            &Type::F32  => write!(f, "f32"),
            &Type::F64  => write!(f, "f64"),
            &Type::Ptr  => write!(f, "ptr"),
            &Type::Void => write!(f, "void"),
        }
    }
}

impl fmt::Debug for FormatTypedOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}]", self.t)
    }
}

impl fmt::Debug for FormatVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " ${:?}", self.var)
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Instruction::Nop => write!(f, "nop"),
            &Instruction::Pop => write!(f, "pop"),
            &Instruction::Dup => write!(f, "dup"),
            &Instruction::Cst(ref index) => write!(f, "cst #{:?}", index),
            &Instruction::Load(ref format) => write!(f, "load{:?}", format),
            &Instruction::Store(ref format) => write!(f, "store{:?}", format),
            &Instruction::Add(ref format) => write!(f, "add{:?}", format),
            &Instruction::Sub(ref format) => write!(f, "sub{:?}", format),
            &Instruction::Mul(ref format) => write!(f, "mul{:?}", format),
            &Instruction::Div(ref format) => write!(f, "div{:?}", format),
            &Instruction::Print(ref format) => write!(f, "print{:?}", format),
        }
    }
}

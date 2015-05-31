use runtime::bytecode;


pub struct ConstantTable<'a> {
    pub table: Vec<bytecode::Constant<'a>>,
}

pub struct Function<'a> {
    pub constant_table: ConstantTable<'a>,
    pub instructions: Vec<bytecode::Instruction>,

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


impl<'a> ConstantTable<'a> {
    pub fn new(table: Vec<bytecode::Constant<'a>>) -> ConstantTable<'a> {
        ConstantTable { table: table }
    }
}

impl<'a> Function<'a> {
    pub fn new(constant_table: ConstantTable<'a>,
           instructions: Vec<bytecode::Instruction>,
           argument_count: u8) -> Function<'a> {
        let (max_operands, locals_count, return_count) = bytecode::calculate_sizes(&instructions);
        if max_operands < 0 {
            panic!("Calculated maximum operand count is less than 0!");
        }

        Function {
            constant_table: constant_table,
            instructions: instructions,
            return_count: return_count,
            argument_count: argument_count,
            locals_count: locals_count,
            max_operands: max_operands as u16,
        }
    }
}

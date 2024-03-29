use std::cmp;
use std::io::{Write, Seek, SeekFrom};

use byteorder::{BigEndian, WriteBytesExt};

use bytecode::*;
use cst::ConstantTable;
use io;
use function::{Function, Sizes};


pub struct FunctionWriter<'a, W: 'a> where W: Write + Seek {
    write: &'a mut W,
    sizes_offset: u64,
    instructions_offset: u64,

    instruction_count: u32,

    pub sizes: Sizes,
    current_op_size: u16,
}

pub struct ConstantTableWriter<'a, W: 'a> where W: Write + Seek {
    write: &'a mut W,
}


impl<'a, W: Write + Seek> FunctionWriter<'a, W> {

    pub fn new(write: &'a mut W, name: &str, constant_table_name: &str, argument_count: u8) -> FunctionWriter<'a, W>
            where W: Write + Seek {
        let mut writer = FunctionWriter {
            write: write,
            sizes_offset: io::string_disk_size(name) as u64,
            instructions_offset: Function::calculate_instructions_offset(name, constant_table_name),
            instruction_count: 0,
            sizes: Sizes::new(0, argument_count, 0, 0),
            current_op_size: 0,
        };

        // Write name.
        io::write_string(writer.write, name).unwrap();

        // Reserve space for the sizes.
        writer.write.write_u8(0).unwrap();
        writer.write.write_u8(0).unwrap();
        writer.write.write_u16::<BigEndian>(0).unwrap();
        writer.write.write_u16::<BigEndian>(0).unwrap();

        // Write constant table name.
        io::write_string(writer.write, constant_table_name).unwrap();

        // Reserve 4 bytes for the instruction count.
        writer.write.write_u32::<BigEndian>(0).unwrap();

        writer
    }

    /// Writes the correct sizes and instruction count.
    pub fn finish(&mut self) {
        // Write all sizes.
        self.write.seek(SeekFrom::Start(self.sizes_offset)).unwrap();
        self.write.write_u8(self.sizes.return_count).unwrap();
        self.write.write_u8(self.sizes.argument_count).unwrap();
        self.write.write_u16::<BigEndian>(self.sizes.locals_count).unwrap();
        self.write.write_u16::<BigEndian>(self.sizes.max_operands).unwrap();

        // Write instruction count.
        self.write.seek(SeekFrom::Start(self.instructions_offset)).unwrap();
        self.write.write_u32::<BigEndian>(self.instruction_count).unwrap();
    }

    pub fn write_operation(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::Nop => { },
            Opcode::Pop => {
                self.sizes_pop_operands(1);
            },
            Opcode::Dup => {
                self.sizes_pop_operands(1); // Dup requires at least one element on the stack.
                self.sizes_push_operands(2);
            },
            _ => panic!("Opcode {:?} not supported for 'write_typed' function.", opcode),
        }

        self.write.write_u8(opcode as u8).unwrap();
        self.instruction_count += 1;
    }

    pub fn write_typed(&mut self, opcode: Opcode, t: Type) {
        match opcode {
            Opcode::Add | Opcode::Sub | Opcode::Mul | Opcode::Div => {
                // These instructions pop 2 elements from the stack,
                // then push 1 element to the stack.
                self.sizes_pop_operands(2);
                self.sizes_push_operands(1);
            },
            Opcode::Print => {
                self.sizes_pop_operands(1);
            },
            _ => panic!("Opcode {:?} not supported for 'write_typed' function.", opcode),
        }

        self.write.write_u8(opcode as u8).unwrap();
        self.write.write_u8(t as u8).unwrap();
        self.instruction_count += 1;
    }

    pub fn write_cst(&mut self, index: ConstantTableIndex) {
        self.sizes_push_operands(1);
        self.write.write_u8(Opcode::Cst as u8).unwrap();
        self.write.write_u16::<BigEndian>(index).unwrap();
        self.instruction_count += 1;
    }

    pub fn write_load(&mut self, var: VariableIndex) {
        self.sizes_push_operands(1);
        self.sizes_used_var(var);
        self.write.write_u8(Opcode::Load as u8).unwrap();
        self.write.write_u16::<BigEndian>(var).unwrap();
        self.instruction_count += 1;
    }

    pub fn write_store(&mut self, var: VariableIndex) {
        self.sizes_pop_operands(1);
        self.sizes_used_var(var);
        self.write.write_u8(Opcode::Store as u8).unwrap();
        self.write.write_u16::<BigEndian>(var).unwrap();
        self.instruction_count += 1;
    }

    pub fn write_ret(&mut self, count: u8) {
        self.sizes_pop_operands(count as u16);
        if self.sizes.return_count < count {
            self.sizes.return_count = count;
        }

        self.write.write_u8(Opcode::Ret as u8).unwrap();
        self.write.write_u8(count).unwrap();
        self.instruction_count += 1;
    }

    fn sizes_used_var(&mut self, var: VariableIndex) {
        self.sizes.locals_count = cmp::max(self.sizes.locals_count, var + 1);
    }

    fn sizes_push_operands(&mut self, amount: u16) {
        assert!(amount < 0x8000); // Should be in range for a safe conversion to i16.
        self.sizes_change_operand_stack_size(amount as i16);
    }

    fn sizes_pop_operands(&mut self, amount: u16) {
        assert!(amount < 0x8000); // Should be in range for a safe conversion to i16.
        self.sizes_change_operand_stack_size(-(amount as i16));
    }

    // Should NOT be used directly, because push and pop sizes could cancel out,
    // although an operation might first pop, for example, 2 elements and then
    // push 1. If we only look at the change after an instruction, we might allow
    // an 'add' operation on a stack of 1 element, which would lead to a buffer
    // underflow when the bytecode is loaded.
    fn sizes_change_operand_stack_size(&mut self, change: i16) {
        let diff = self.current_op_size as i32 + change as i32;
        if diff < 0 {
            panic!("Operand stack underflow by {} elements detected.", diff);
        }

        self.current_op_size = diff as u16;
        if self.sizes.max_operands < self.current_op_size {
            self.sizes.max_operands = self.current_op_size;
        }
    }

}

impl<'a, W: Write + Seek> ConstantTableWriter<'a, W> {

    pub fn new(write: &'a mut W) -> ConstantTableWriter<'a, W> {
        ConstantTableWriter {
            write: write,
        }
    }

    pub fn write_constant_table(&mut self, constant_table: &ConstantTable) {
        self.write.write_u16::<BigEndian>(constant_table.table.len() as u16).unwrap();
        for constant in &constant_table.table {
            self.write_constant(constant);
        }
    }

    fn write_constant(&mut self, constant: &Constant) {
        match *constant {
            Constant::U64(num) => {
                self.write.write_u8(ConstantTag::U64 as u8).unwrap();
                self.write.write_u64::<BigEndian>(num).unwrap();
            },
            Constant::U32(num) => {
                self.write.write_u8(ConstantTag::U32 as u8).unwrap();
                self.write.write_u32::<BigEndian>(num).unwrap();
            },
            Constant::I64(num) => {
                self.write.write_u8(ConstantTag::I64 as u8).unwrap();
                self.write.write_i64::<BigEndian>(num).unwrap();
            },
            Constant::I32(num) => {
                self.write.write_u8(ConstantTag::I32 as u8).unwrap();
                self.write.write_i32::<BigEndian>(num).unwrap();
            },
            Constant::F64(num) => {
                self.write.write_u8(ConstantTag::F64 as u8).unwrap();
                self.write.write_f64::<BigEndian>(num).unwrap();
            },
            Constant::F32(num) => {
                self.write.write_u8(ConstantTag::F32 as u8).unwrap();
                self.write.write_f32::<BigEndian>(num).unwrap();
            },
            Constant::Str(ref string) => {
                self.write.write_u8(ConstantTag::Str as u8).unwrap();
                io::write_string(self.write, string).unwrap();
            },
        }
    }

}

use std::cmp;
use std::io::{Write, Seek, SeekFrom};

use byteorder::{BigEndian, WriteBytesExt};

use bytecode::*;
use function::Sizes;


pub struct InstructionWriter<'a, W: 'a> where W: Write + Seek {
    write: &'a mut W,
    instruction_count: u32,

    pub sizes: Sizes,
    current_op_size: u16,
}


impl<'a, W: Write + Seek> InstructionWriter<'a, W> {

    pub fn new(write: &'a mut W) -> InstructionWriter<'a, W>
            where W: Write + Seek {
        let mut writer = InstructionWriter {
            write: write,
            instruction_count: 0,
            sizes: Sizes::new(0, 0, 0, 0),
            current_op_size: 0,
        };

        // Reserve 4 bytes for the instruction count.
        writer.write.write_u32::<BigEndian>(0).unwrap();

        writer
    }

    /// Sets the instruction count to the correct value.
    pub fn finish(&mut self) {
        self.write.seek(SeekFrom::Start(0)).unwrap();
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
        assert!(amount < 0x8000); // Muss zu i16 konvertierbar sein.
        self.sizes_change_operand_stack_size(amount as i16);
    }

    fn sizes_pop_operands(&mut self, amount: u16) {
        assert!(amount < 0x8000); // Muss zu i16 konvertierbar sein.
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

extern crate alloc;

use std::mem;
use std::ptr;

use runtime::bytecode::*;
use runtime::function::*;

use self::alloc::heap::{allocate, deallocate};


const stack_align: usize = 8;
const stack_element_size: usize = 8;

pub struct Context {
    /// The general stack is 8-byte aligned.
    stack: *mut u8,

    /// The amount of elements on the stack.
    /// NOT the byte size of the stack:
    /// TODO Possibly confusing name.
    stack_size: usize,
}


impl Drop for Context {
    fn drop(&mut self) {
        unsafe { deallocate(self.stack, self.byte_size(), stack_align) };
    }
}


impl Context {
    pub fn new(stack_size: usize) -> Context {
        let stack = unsafe { allocate(stack_size * stack_element_size, stack_align) };
        Context { stack: stack, stack_size: stack_size }
    }

    pub fn set_local(&self, var: VariableIndex, value: u64) {
        unsafe {
            let locals = self.u64_stack_view();
            *locals.offset(var as isize) = value;
        }
    }

    fn u64_stack_view(&self) -> *mut u64 {
        return self.stack as *mut u64;
    }

    /// Calculates the (exclusive) end of the stack.
    fn byte_size(&self) -> usize {
        return self.stack_size * stack_element_size;
    }

    /// Returns a vector of function results.
    pub fn run<'a>(&self, function: &Function<'a>) -> Vec<u64> {
        let return_count = function.return_count as usize;

        // The return stack is filled at 0..return_count.
        self.call(function, return_count, 0);
        let mut result = vec![];
        for i in 0..return_count {
            unsafe {
                result.push(*(self.u64_stack_view().offset(i as isize)));
            }
        }
        result
    }

    pub fn call<'a>(&self, function: &Function<'a>, stack_bottom: usize,
                   stack_return: usize) {
        let insts = &function.instructions;
        let inst_count = function.instructions.len();
        let mut inst_index = 0;

        // Operand stack top is exclusive.
        // The operand stack comes after the locals.
        let mut op_stack_top: usize = stack_bottom + function.locals_size;

        // Checks and prevents stack overflows.
        if (op_stack_top + function.stack_size >= self.byte_size()) {
            panic!("Stack overflow occured!");
        }

        // Locals view.
        let locals: *mut u64 = self.stack as *mut u64;

        // Stack views.
        let sv_u64: *mut u64 = self.stack as *mut u64;
        let sv_u32: *mut u32 = sv_u64 as *mut u32;
        let sv_i64: *mut i64 = sv_u64 as *mut i64;
        let sv_i32: *mut i32 = sv_u64 as *mut i32;
        let sv_f64: *mut f64 = sv_u64 as *mut f64;
        let sv_f32: *mut f32 = sv_u64 as *mut f32;

        while inst_index < inst_count {
            let inst = &insts[inst_index];
            match *inst {
                Instruction::Nop => {

                },

                Instruction::Pop => op_stack_top -= 1,
                Instruction::Dup => unsafe {
                    *sv_u64.offset(op_stack_top as isize) = *sv_u64.offset((op_stack_top - 1) as isize);
                    op_stack_top += 1;
                },

                Instruction::Cst(ref index) => {
                    let constant = &function.constant_table.table[*index as usize];
                    match *constant {
                        Constant::U64(num) => unsafe {
                            *sv_u64.offset(sao(Type::U64, op_stack_top)) = num;
                        },
                        Constant::I64(num) => unsafe {
                            *sv_i64.offset(sao(Type::I64, op_stack_top)) = num;
                        },
                        _ => panic!(format!("Constant at index {} must be a number!", index)),
                    }
                    op_stack_top += 1;
                },

                Instruction::Load(ref var) => unsafe {
                    *sv_u64.offset(op_stack_top as isize) = *locals.offset(*var as isize);
                    op_stack_top += 1;
                },

                Instruction::Store(ref var) => unsafe {
                    op_stack_top -= 1;
                    *locals.offset(*var as isize) = *sv_u64.offset(op_stack_top as isize);
                },

                Instruction::Add(ref t) => unsafe {
                    let left = op_stack_top - 2;
                    let right = op_stack_top - 1;

                    match *t {
                        Type::U64 => *sv_u64.offset(sao(Type::U64, left)) = *sv_u64.offset(sao(Type::U64, left)) + *sv_u64.offset(sao(Type::U64, right)),
                        Type::U32 => *sv_u32.offset(sao(Type::U32, left)) = *sv_u32.offset(sao(Type::U32, left)) + *sv_u32.offset(sao(Type::U32, right)),
                        Type::I64 => {
                            *sv_i64.offset(sao(Type::I64, left)) = *sv_i64.offset(sao(Type::I64, left)) + *sv_i64.offset(sao(Type::I64, right));
                        },
                        _ => panic!("Unsupported type!"),
                    }

                    op_stack_top = right;
                },

                Instruction::Sub(ref t) => unsafe {
                    let left = op_stack_top - 2;
                    let right = op_stack_top - 1;
                    *sv_u64.offset(sao(Type::U64, left)) = *sv_u64.offset(sao(Type::U64, left)) - *sv_u64.offset(sao(Type::U64, right));
                    op_stack_top = right;
                },

                Instruction::Mul(ref t) => unsafe {
                    let left = op_stack_top - 2;
                    let right = op_stack_top - 1;

                    match *t {
                        Type::U64 => *sv_u64.offset(sao(Type::U64, left)) = *sv_u64.offset(sao(Type::U64, left)) * *sv_u64.offset(sao(Type::U64, right)),
                        Type::I64 => {
                            *sv_i64.offset(sao(Type::I64, left)) = *sv_i64.offset(sao(Type::I64, left)) * *sv_i64.offset(sao(Type::I64, right));
                        },
                        _ => panic!("Unsupported type!"),
                    }

                    op_stack_top = right;
                },

                Instruction::Div(ref t) => unsafe {
                    let left = op_stack_top - 2;
                    let right = op_stack_top - 1;
                    *sv_u64.offset(sao(Type::U64, left)) = *sv_u64.offset(sao(Type::U64, left)) / *sv_u64.offset(sao(Type::U64, right));
                    op_stack_top = right;
                },

                Instruction::Print(ref t) => unsafe {

                    match *t {
                        Type::U64 => println!("{}", *sv_u64.offset(sao(Type::U64, op_stack_top - 1))),
                        Type::I64 => {
                            println!("{}", *sv_i64.offset(sao(Type::I64, op_stack_top - 1)));
                        },
                        _ => panic!("Unsupported type!"),
                    }


                    op_stack_top -= 1;
                },

                // _ => panic!(format!("Instruction at index {} not implemented!", inst_index))
            }

            inst_index += 1;
        }
    }
}


/// Stack address offset.
fn sao(t: Type, stack_index: usize) -> isize {
    match t {
        Type::U64 | Type::I64 | Type::F64 => stack_index as isize,
        Type::U32 | Type::I32 | Type::F32 => (stack_index as isize) * 2 + 1,
        _ => panic!("Address could not be calculated!"),
    }
}

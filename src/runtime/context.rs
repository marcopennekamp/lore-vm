extern crate libc;

use std::mem;
use std::ptr;

use runtime::bytecode::*;
use runtime::function::*;

use self::libc::{c_void, size_t, malloc, free};


pub struct Context {
    /// The general stack is 8-byte aligned.
    stack: *mut c_void,

    /// The amount of elements on the stack.
    stack_size: usize,
}


impl Drop for Context {
    fn drop(&mut self) {
        unsafe { free(self.stack) };
    }
}


impl Context {
    pub fn new(stack_size: usize) -> Context {
        let stack = unsafe { malloc((stack_size * 8) as u64) };
        Context { stack: stack, stack_size: stack_size }
    }

    pub fn set_local(&self, var: VariableIndex, value: u64) {
        unsafe {
            let locals = self.stack as *mut u64;
            *locals.offset(var as isize) = value;
        }
    }

    pub fn run<'a>(&self, function: &Function<'a>, stack_bottom: usize) {
        let insts = &function.instructions;
        let inst_count = function.instructions.len();
        let mut inst_index = 0;

        // Operand stack top is exclusive.
        // The operand stack comes after the locals.
        let mut op_stack_top: usize = stack_bottom + function.locals_size;

        // Locals view.
        let locals: *mut u64 = self.stack as *mut u64;

        // Stack views.
        let sv_u64: *mut u64 = self.stack as *mut u64;
        let sv_u32: *mut u32 = sv_u64 as *mut u32;
        // let sv_u32: *mut u32 = unsafe { mem::transmute(sv_u64) };
        let sv_i64: *mut i64 = unsafe { mem::transmute(sv_u64) };
        let sv_i32: *mut i32 = unsafe { mem::transmute(sv_u64) };
        let sv_f64: *mut f64 = unsafe { mem::transmute(sv_u64) };
        let sv_f32: *mut f32 = unsafe { mem::transmute(sv_u64) };

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

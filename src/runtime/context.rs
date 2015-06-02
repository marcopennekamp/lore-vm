extern crate alloc;

use std::ptr;

use runtime::bytecode::*;
use runtime::function::*;

use self::alloc::heap::{allocate, deallocate};


const STACK_ALIGN: usize = 8;
const STACK_ELEMENT_SIZE: usize = 8;

pub struct Context {
    /// The general stack is 8-byte aligned.
    stack: *mut u8,

    /// The amount of elements on the stack.
    stack_length: usize,
}


impl Drop for Context {
    fn drop(&mut self) {
        unsafe { deallocate(self.stack, self.stack_size(), STACK_ALIGN) };
    }
}


/// Direct stack access.
macro_rules! dsa {
    ( $ptr:expr, $pos:expr ) => {
        *$ptr.offset($pos as isize)
    };
}

/// Typed stack access.
macro_rules! tsa {
    ( $type_enum:expr, $ptr:expr, $pos:expr ) => {
        *$ptr.offset(sao($type_enum, $pos))
    };
}

// This workaround prevents an error with the following expression:
//      a $op b
// Where $op is a token tree. The compiler does not recognize the
// token tree as an operator, and instead complains, quite ironically,
// that an operator was expected, but instead $op was found.
// Refer to https://github.com/rust-lang/rust/issues/5846.
macro_rules! workaround_expr {
    ( $a:expr ) => {
        $a
    }
}

macro_rules! stack_op {
    ( $type_enum:expr, $ptr:expr, $top:ident, $op:tt ) => {
        {
            let left = $top - 2;
            let right = $top - 1;
            tsa!($type_enum, $ptr, left) = workaround_expr!(tsa!($type_enum, $ptr, left) $op tsa!($type_enum, $ptr, right));
            $top = right;
        }
    };
}

/// Expects t as a Type enum reference.
macro_rules! match_op {
    ( $stack:ident, $t:ident, $top:ident, $op:tt ) => {
        {
            match *($t) {
                Type::U64 => stack_op!(Type::U64, $stack as *mut u64, $top, $op),
                Type::U32 => stack_op!(Type::U32, $stack as *mut u32, $top, $op),
                Type::I64 => stack_op!(Type::I64, $stack as *mut i64, $top, $op),
                Type::I32 => stack_op!(Type::I32, $stack as *mut i32, $top, $op),
                Type::F64 => stack_op!(Type::F64, $stack as *mut f64, $top, $op),
                Type::F32 => stack_op!(Type::F32, $stack as *mut f32, $top, $op),
                _ => panic!("Unsupported type!"),
            }
        }
    }
}

impl Context {
    pub fn new(stack_length: usize) -> Context {
        let stack = unsafe { allocate(stack_length * STACK_ELEMENT_SIZE, STACK_ALIGN) };
        Context { stack: stack, stack_length: stack_length }
    }

    fn u64_stack_view(&self) -> *mut u64 {
        return self.stack as *mut u64;
    }

    /// Calculates the byte size of the stack.
    fn stack_size(&self) -> usize {
        return self.stack_length * STACK_ELEMENT_SIZE;
    }

    /// Returns a vector of function results.
    pub fn run(&self, function: &Function, arguments: &Vec<u64>) -> Vec<u64> {
        if function.id == INVALID_FUNCTION_ID {
            panic!("The function must be registered with an environment.");
        }

        let argument_count = function.sizes.argument_count as usize;

        if arguments.len() != argument_count {
            panic!("Expected {} arguments but got {}.", arguments.len(), argument_count);
        }

        let return_count = function.sizes.return_count as usize;

        // Push arguments to the locals part of the stack.
        // Locals start at offset return_count.
        for i in 0..argument_count {
            unsafe {
                *self.u64_stack_view().offset((i + return_count) as isize) = arguments[i];
            }
        }

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

    pub fn call(&self, function: &Function, stack_bottom: usize,
                   stack_return: usize) {
        let insts;
        match function.instructions {
            Instructions::Bytecode(ref vec) => insts = vec,
            Instructions::FilePath(..) => {
                panic!("Bytecode expected, but got file path for function '{}'!",
                    function.name);
            }
        }
        let inst_count = insts.len();
        let mut inst_index = 0;

        // Operand stack top is exclusive.
        // The operand stack comes after the locals.
        let mut op_stack_top: usize = stack_bottom + function.sizes.locals_count as usize;

        // Checks and prevents stack overflows.
        if op_stack_top + function.sizes.max_operands as usize >= self.stack_size() {
            panic!("Stack overflow occured!");
        }

        // Locals view.
        let locals: *mut u64 = unsafe {
            (self.stack as *mut u64).offset(function.sizes.return_count as isize)
        };

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

                Instruction::Pop => {
                    op_stack_top -= 1
                },

                Instruction::Dup => unsafe {
                    dsa!(sv_u64, op_stack_top) = dsa!(sv_u64, op_stack_top - 1);
                    op_stack_top += 1;
                },

                Instruction::Cst(ref index) => unsafe {
                    let constant = &function.constant_table.table[*index as usize];
                    match *constant {
                        Constant::U64(num) => { tsa!(Type::U64, sv_u64, op_stack_top) = num; },
                        Constant::U32(num) => { tsa!(Type::U32, sv_u32, op_stack_top) = num; },
                        Constant::I64(num) => { tsa!(Type::I64, sv_i64, op_stack_top) = num; },
                        Constant::I32(num) => { tsa!(Type::I32, sv_i32, op_stack_top) = num; },
                        Constant::F64(num) => { tsa!(Type::F64, sv_f64, op_stack_top) = num; },
                        Constant::F32(num) => { tsa!(Type::F32, sv_f32, op_stack_top) = num; },
                        _ => panic!(format!("Constant at index {} must be a number!", index)),
                    }
                    op_stack_top += 1;
                },

                Instruction::Load(ref var) => unsafe {
                    dsa!(sv_u64, op_stack_top) = dsa!(locals, *var);
                    op_stack_top += 1;
                },

                Instruction::Store(ref var) => unsafe {
                    dsa!(locals, *var) = dsa!(sv_u64, op_stack_top - 1);
                    op_stack_top -= 1;
                },

                Instruction::Add(ref t) => unsafe {
                    match_op!(sv_u64, t, op_stack_top, +);
                },

                Instruction::Sub(ref t) => unsafe {
                    match_op!(sv_u64, t, op_stack_top, -);
                },

                Instruction::Mul(ref t) => unsafe {
                    match_op!(sv_u64, t, op_stack_top, *);
                },

                Instruction::Div(ref t) => unsafe {
                    match_op!(sv_u64, t, op_stack_top, /);
                },

                Instruction::Ret(ref count) => unsafe {
                    let count: usize = *count as usize;
                    let dst = sv_u64.offset(stack_return as isize);
                    let src = sv_u64.offset(op_stack_top as isize - count as isize);
                    ptr::copy(src, dst, count);
                },

                Instruction::Print(ref t) => unsafe {
                    match *t {
                        Type::U64 => println!("{}", tsa!(Type::U64, sv_u64, op_stack_top - 1)),
                        Type::U32 => println!("{}", tsa!(Type::U32, sv_u32, op_stack_top - 1)),
                        Type::I64 => println!("{}", tsa!(Type::I64, sv_i64, op_stack_top - 1)),
                        Type::I32 => println!("{}", tsa!(Type::I32, sv_i32, op_stack_top - 1)),
                        Type::F64 => println!("{}", tsa!(Type::F64, sv_f64, op_stack_top - 1)),
                        Type::F32 => println!("{}", tsa!(Type::F32, sv_f32, op_stack_top - 1)),
                        _ => panic!("Unsupported type!"),
                    }
                    op_stack_top -= 1;
                },
            }
            inst_index += 1;
        }
    }
}

/// Stack address offset.
// TODO Inline?
fn sao(t: Type, stack_index: usize) -> isize {
    match t {
        Type::U64 | Type::I64 | Type::F64 => stack_index as isize,
        Type::U32 | Type::I32 | Type::F32 => (stack_index as isize) * 2 + 1,
        _ => panic!("Address could not be calculated!"),
    }
}

use std::collections::HashMap;

use function::*;


/// Currently NOT thread-safe. TODO How is that in Rust, even?
pub struct Environment {
    functions: Vec<Function>,
    function_names_to_ids: HashMap<String, u32>,
}


impl Environment {

    pub fn new() -> Environment {
        Environment { functions: Vec::new(), function_names_to_ids: HashMap::new() }
    }

    pub fn register_function(&mut self, mut function: Function) -> u32 {
        if function.id != INVALID_FUNCTION_ID {
            panic!("The ID of the function '{}' has already been set.");
        }

        let next_id: u32 = self.functions.len() as u32;
        match self.function_names_to_ids.get(&function.name[..]) {
            Some(..) => panic!("Function '{}' is already registered.", function.name),
            None => {
                // TODO: The String is copied here. That should not be a concern, memory-wise,
                // but we might optimize this with an Rc<T>.
                let name = function.name.clone();
                function.id = next_id;
                self.functions.push(function);
                self.function_names_to_ids.insert(name, next_id);
            },
        }

        next_id
    }

    /// Does not check whether the function exists.
    /// Does NOT load the bytecode.
    pub fn get_function_by_id(&self, id: u32) -> &Function {
        return &self.functions[id as usize];
    }

    /// Does NOT load the bytecode.
    pub fn get_function_by_name(&self, name: &str) -> Option<&Function> {
        match self.function_names_to_ids.get(name) {
            Some(id) => Some(self.get_function_by_id(*id)),
            None => None,
        }
    }

    /// Loads the instructions of the function if they are not already loaded.
    pub fn fetch_function_by_id(&mut self, id: u32) -> &Function {
        let function = &mut self.functions[id as usize];
        let instructions_option = match function.instructions {
            Instructions::FilePath(ref path) => {
                Some(Instructions::from_file(&path[..]))
            },
            Instructions::Bytecode(..) => {
                // We're good, no need to load anything!
                None
            },
        };

        match instructions_option {
            Some(instructions) => {
                function.instructions = instructions;
            },
            None => { },
        };

        function
    }

}

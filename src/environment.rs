use std::collections::HashMap;

use function::*;


/// Currently NOT thread-safe. TODO How is that in Rust, even?
pub struct Environment<'a> {
    functions: Vec<Function>,
    function_names_to_ids: HashMap<&'a str, u32>,
}


impl<'a> Environment<'a> {

    pub fn new() -> Environment<'a> {
        Environment { functions: Vec::new(), function_names_to_ids: HashMap::new() }
    }

    pub fn register_function(&'a mut self, mut function: Function) -> &Function {
        if function.id != INVALID_FUNCTION_ID {
            panic!("The ID of the function '{}' has already been set.");
        }

        let next_id: u32 = self.functions.len() as u32;
        let moved_function;
        match self.function_names_to_ids.get(&function.name[..]) {
            Some(..) => panic!("Function '{}' is already registered.", function.name),
            None => {
                function.id = next_id;
                self.functions.push(function);
                moved_function = &self.functions[next_id as usize];
                self.function_names_to_ids.insert(&moved_function.name, next_id);
            },
        }

        moved_function
    }

    /// Does not check whether the function exists.
    pub fn get_function_by_id(&'a self, id: u32) -> &Function {
        return &self.functions[id as usize];
    }

    pub fn get_function_by_name(&'a self, name: &str) -> Option<&Function> {
        match self.function_names_to_ids.get(name) {
            Some(id) => Some(self.get_function_by_id(*id)),
            None => None,
        }
    }

    /// Loads the instructions of the function if they are not already loaded.
    pub fn fetch_function_by_id(&'a mut self, id: u32) -> &Function {
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

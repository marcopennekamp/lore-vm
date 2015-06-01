use std::collections::HashMap;

use runtime::function::*;


/// Currently NOT thread-safe. TODO How is that in Rust, even?
pub struct Environment<'a> {
    functions: Vec<Function<'a>>,
    function_names_to_ids: HashMap<&'a str, u32>,
}


impl<'a> Environment<'a> {

    pub fn new() -> Environment<'a> {
        Environment { functions: vec![], function_names_to_ids: HashMap::new() }
    }

    pub fn register_function(&'a mut self, mut function: Function<'a>) -> &Function<'a> {
        if function.id != INVALID_FUNCTION_ID {
            panic!("The function '{}' has already been registered. (Its id is set.)");
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
    pub fn get_function_by_id(&'a self, id: u32) -> &Function<'a> {
        return &self.functions[id as usize];
    }

    pub fn get_function_by_name(&'a self, name: &str) -> Option<&Function<'a>> {
        match self.function_names_to_ids.get(name) {
            Some(id) => Some(self.get_function_by_id(*id)),
            None => None,
        }
    }

}

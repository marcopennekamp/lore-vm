use std::collections::HashMap;
use std::sync::Arc;
use std::path::Path;
use std::io::{Seek, SeekFrom};

use function::{INVALID_FUNCTION_ID, Function, Instructions};
use cst::ConstantTable;


/// Currently NOT thread-safe. TODO How is that in Rust, even?
pub struct Environment {
    functions: Vec<Function>,
    function_names_to_ids: HashMap<String, u32>,

    /// The String is the path (without file extension) to the constant table.
    constant_tables: HashMap<String, Arc<ConstantTable>>,
}


impl Environment {

    pub fn new() -> Environment {
        Environment {
            functions: Vec::new(),
            function_names_to_ids: HashMap::new(),
            constant_tables: HashMap::new(),
        }
    }

    pub fn register_function(&mut self, mut function: Function) -> u32 {
        if function.id != INVALID_FUNCTION_ID {
            panic!("The ID of the function '{}' has already been set.", function.name);
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

    /// Loads the constant table on demand, or returns a cached version.
    pub fn fetch_constant_table(&mut self, path: &Path) -> Arc<ConstantTable> {
        let arc_opt = self.constant_tables.get(path.to_str().unwrap()).map(|arc| arc.clone());
        match arc_opt {
            Some(arc) => arc,
            None => {
                // Load constant table, then register it and return it.
                let arc = Arc::new(ConstantTable::from_file(path).unwrap());
                self.constant_tables.insert(path.to_str().unwrap().to_string(), arc.clone());
                arc
            },
        }
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
            Instructions::File { ref path, ref offset } => {
                let result = Function::open_reader(path.as_path());
                if result.is_err() {
                    panic!("Could not open reader for file '{:?}'.", path);
                }

                let mut reader = result.unwrap();
                let result = reader.seek(SeekFrom::Start(*offset));
                if result.is_err() {
                    panic!("Could not seek to the appropriate place for file '{:?}'.", path);
                }

                let result = Instructions::from_read(&mut reader);
                if result.is_err() {
                    panic!("Instructions from file '{:?}' could not be loaded.", path);
                }
                Some(result.unwrap())
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

use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct IdentifierIndex {
    data_types: HashMap<String, i16>,
    flow_types: HashMap<String, i16>,
    runtime_flow_types: HashMap<String, i16>,
    functions: HashMap<String, i16>,
    function_definitions: HashMap<String, i16>,
    module_configuration_definitions: HashMap<String, i16>,
}

fn normalize(s: &str) -> String {
    s.trim().to_ascii_lowercase()
}

impl IdentifierIndex {
    pub fn insert_data_type(&mut self, name: &str, id: i16) -> Option<i16> {
        self.data_types.insert(normalize(name), id)
    }
    pub fn insert_flow_type(&mut self, name: &str, id: i16) -> Option<i16> {
        self.flow_types.insert(normalize(name), id)
    }
    pub fn insert_runtime_flow_type(&mut self, name: &str, id: i16) -> Option<i16> {
        self.runtime_flow_types.insert(normalize(name), id)
    }
    pub fn insert_function(&mut self, name: &str, id: i16) -> Option<i16> {
        self.functions.insert(normalize(name), id)
    }
    pub fn insert_function_definition(&mut self, name: &str, id: i16) -> Option<i16> {
        self.function_definitions.insert(normalize(name), id)
    }
    pub fn insert_module_configuration_definition(&mut self, name: &str, id: i16) -> Option<i16> {
        self.module_configuration_definitions
            .insert(normalize(name), id)
    }

    pub fn has_data_type(&self, name: &str, except: Option<i16>) -> bool {
        self.data_types
            .get(&normalize(name))
            .map(|found| except.map(|e| *found != e).unwrap_or(true))
            .unwrap_or(false)
    }

    pub fn has_flow_type(&self, name: &str, except: Option<i16>) -> bool {
        self.flow_types
            .get(&normalize(name))
            .map(|found| except.map(|e| *found != e).unwrap_or(true))
            .unwrap_or(false)
    }

    pub fn has_runtime_flow_type(&self, name: &str, except: Option<i16>) -> bool {
        self.runtime_flow_types
            .get(&normalize(name))
            .map(|found| except.map(|e| *found != e).unwrap_or(true))
            .unwrap_or(false)
    }

    pub fn has_function_definition(&self, name: &str, except: Option<i16>) -> bool {
        self.function_definitions
            .get(&normalize(name))
            .map(|found| except.map(|e| *found != e).unwrap_or(true))
            .unwrap_or(false)
    }

    pub fn has_module_configuration_definition(&self, name: &str, except: Option<i16>) -> bool {
        self.module_configuration_definitions
            .get(&normalize(name))
            .map(|found| except.map(|e| *found != e).unwrap_or(true))
            .unwrap_or(false)
    }
}

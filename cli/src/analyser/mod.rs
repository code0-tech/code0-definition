use tucana::shared::{DataTypeIdentifier, DefinitionDataType, FlowType, RuntimeFunctionDefinition};
use tucana::shared::data_type_identifier::Type;
use tucana::shared::definition_data_type_rule::Config;

#[derive(Clone)]
pub struct AnalysableDataType {
   pub definition_data_type: DefinitionDataType,
    pub id: i16
}


pub struct AnalysableFlowType {
    pub flow_type: FlowType,
    pub id: i16
}


pub struct AnalysableFunction {
    pub function: RuntimeFunctionDefinition,
    pub id: i16
}

pub struct Analyser {
    pub data_types: Vec<AnalysableDataType>,
    pub flow_types: Vec<AnalysableFlowType>,
    pub functions: Vec<AnalysableFunction>
}

impl Analyser {

    pub fn data_type_identifier_exists(&self, identifier: String, id: i16) -> bool {

        for data_types in &self.data_types {
            if id == data_types.id {
                continue
            }

            if data_types.definition_data_type.identifier.to_lowercase() != identifier.to_lowercase() {
                continue;
            }
            return true;
        }
        false
    }

    pub fn handle_data_type(&self, analysable_data_type: AnalysableDataType, data_type_identifier: DataTypeIdentifier) -> Vec<String> {

        let data_type = analysable_data_type.definition_data_type.clone();

        let id = analysable_data_type.id;
        let mut result = vec![];
        if let Some(r#type) = data_type_identifier.r#type {
            match r#type {
                Type::DataTypeIdentifier(identifier) => {
                    if !self.data_type_identifier_exists(identifier.clone(), id) {
                        println!("A Unknown DataType inside a rule was detected: {}", identifier)
                    }
                }
                Type::GenericType(generic) => {
                    if !self.data_type_identifier_exists(generic.data_type_identifier.clone(), id) {
                        println!("A Unknown DataType inside a rule was detected: {}", generic.data_type_identifier)
                    }

                    for mapper in generic.generic_mappers {
                        if data_type.generic_keys.contains(&mapper.target) {
                            result.push(mapper.target.clone())
                        } else {
                            println!("A GenericKey was used that did not exists in this DataType: {}", mapper.target)
                        }

                        for source in mapper.source {
                            result.append(&mut self.handle_data_type(analysable_data_type.clone(), source))
                        }
                    }
                }
                Type::GenericKey(key) => {
                    result.push(key.clone())
                }
            }
        } else {
            println!("Type is undefined!")
        }

        result
    }

    pub fn analyse_data_type(&self, analysable_data_type: AnalysableDataType)  {
        let id = analysable_data_type.id;
        let data_type = analysable_data_type.definition_data_type.clone();
        println!("Checking: {}", data_type.identifier.clone());
        // Check if Identifier is duplicate
        if self.data_type_identifier_exists(data_type.identifier.clone(), id) {
            println!("Duplicate definition of type {}", data_type.identifier)
        }

        // The variant 0 never should occur
        if data_type.variant == 0 {
            println!("Type {} detected varriant 0", data_type.identifier)
        }

        // Generic Keys are present. Search if they are referenced!
        if !data_type.generic_keys.is_empty() {
            let mut detected_generic_keys: Vec<String> = vec![];
            if data_type.rules.is_empty() {
                println!("Generic Keys are defined but never used!")
            }

            for optional_rule in &data_type.rules {
                if let Some(config) = optional_rule.clone().config {
                    match config {
                        Config::ContainsKey(rule) => {
                            if let Some(data_type_identifier) = rule.data_type_identifier {
                                detected_generic_keys.append(&mut self.handle_data_type(analysable_data_type.clone(), data_type_identifier))
                            } else {
                                println!("DataTypeIdentifier is undefined!")
                            }
                        }
                        Config::ContainsType(rule) => {
                            if let Some(data_type_identifier) = rule.data_type_identifier {
                                detected_generic_keys.append(&mut self.handle_data_type(analysable_data_type.clone(), data_type_identifier))
                            } else {
                                println!("DataTypeIdentifier is undefined!")
                            }
                        }
                        Config::ItemOfCollection(rule) => {
                            if rule.items.is_empty() {
                                println!("ItemOfCollection is defined but does not have any items!")
                            }
                        }
                        Config::NumberRange(_) => {}
                        Config::Regex(_) => {}
                        Config::InputTypes(rule) => {
                            if rule.input_types.is_empty() {
                                println!("InputTypes is defined but does not have any inputs!")
                            }

                            for input_type in &rule.input_types {
                                if let Some(data_type_identifier) = &input_type.data_type_identifier {
                                    detected_generic_keys.append(&mut self.handle_data_type(analysable_data_type.clone(), data_type_identifier.clone()))
                                } else {
                                    println!("DataTypeIdentifier is undefined!")
                                }
                            }
                        }
                        Config::ReturnType(rule) => {
                            if let Some(data_type_identifier) = &rule.data_type_identifier {
                                detected_generic_keys.append(&mut self.handle_data_type(analysable_data_type.clone(), data_type_identifier.clone()))
                            } else {
                                println!("DataTypeIdentifier is undefined!")
                            }
                        }
                        Config::ParentType(rule) => {
                            if let Some(data_type_identifier) = &rule.parent_type {
                                detected_generic_keys.append(&mut self.handle_data_type(analysable_data_type.clone(), data_type_identifier.clone()))
                            } else {
                                println!("DataTypeIdentifier is undefined!")
                            }
                        }
                    }
                }
            }

            //TODO: Handle key not present/not exissting
            for key in &detected_generic_keys {
                println!("Detected key: {}", key);
            }
        } else {
            // Check here for any empty configs!
            for rule in &data_type.rules {
                if rule.config.is_none() {
                    println!("found empty rule!")
                }
            }
        }

        // Check if at least one Translation is present
        if data_type.name.is_empty() {
            println!("At least one translation should be defined")
        }

    }
}

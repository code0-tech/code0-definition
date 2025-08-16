use std::path::Path;
use tucana::shared::{DataTypeIdentifier, DefinitionDataType, FlowType, RuntimeFunctionDefinition};
use tucana::shared::data_type_identifier::Type;
use tucana::shared::definition_data_type_rule::Config;
use code0_definition_reader::reader::{MetaType, ParsableDefinition, Reader};
use crate::formatter::{error, warning};

#[derive(Clone)]
pub struct AnalysableDataType {
    pub original_definition: ParsableDefinition,
    pub definition_data_type: DefinitionDataType,
    pub id: i16
}

pub struct AnalysableFlowType {
    pub original_definition: ParsableDefinition,
    pub flow_type: FlowType,
    pub id: i16
}

pub struct AnalysableFunction {
    pub original_definition: ParsableDefinition,
    pub function: RuntimeFunctionDefinition,
    pub id: i16
}

pub struct Analyser {
    pub data_types: Vec<AnalysableDataType>,
    pub flow_types: Vec<AnalysableFlowType>,
    pub functions: Vec<AnalysableFunction>
}

impl Analyser {

    pub fn new(path: &str) -> Analyser {
        let reader = match Reader::from_path(path) {
            Some(res) => res,
            None => {
                panic!("No definitions behind this path");
            }
        };

        let mut current_index = 0;
        let mut collected_data_types: Vec<AnalysableDataType> = vec![];
        let mut collected_flow_types: Vec<AnalysableFlowType> = vec![];
        let mut collected_functions: Vec<AnalysableFunction> = vec![];

        for features in reader.meta {
            match features.r#type {
                MetaType::FlowType => {
                    for p_flow_type in &features.data {
                        current_index += 1;
                        match serde_json::from_str::<FlowType>(p_flow_type.definition_string.as_str()) {
                            Ok(flow_type) => collected_flow_types.push(AnalysableFlowType {
                                original_definition: p_flow_type.clone(),
                                flow_type,
                                id: current_index,
                            }),
                            Err(err) => {
                                if let Some(str_path) = &p_flow_type.path {
                                    let path = Path::new(str_path);
                                    error(err.to_string(), format!("{:?}:{}:{}", path.display(), p_flow_type.starting_line, 1));
                                }
                            },
                        }
                    }
                },
                MetaType::DataType => {
                    for p_data_type in &features.data {
                        current_index += 1;
                        match serde_json::from_str::<DefinitionDataType>(p_data_type.definition_string.as_str()) {
                            Ok(data_type) => collected_data_types.push(AnalysableDataType {
                                original_definition: p_data_type.clone(),
                                definition_data_type: data_type,
                                id: current_index,
                            }),
                            Err(err) => {
                                if let Some(str_path) = &p_data_type.path {
                                    let path = Path::new(str_path);
                                    error(err.to_string(), format!("{}:{}:{}", path.display(), p_data_type.starting_line, 1));
                                }
                            },
                        }
                    }
                }
                MetaType::RuntimeFunction => {
                    for p_function in &features.data {
                        current_index += 1;
                        match serde_json::from_str::<RuntimeFunctionDefinition>(p_function.definition_string.as_str()) {
                            Ok(function) => collected_functions.push(AnalysableFunction {
                                original_definition: p_function.clone(),
                                function,
                                id: current_index,
                            }),
                            Err(err) => {
                                if let Some(str_path) = &p_function.path {
                                    let path = Path::new(str_path);
                                    error(err.to_string(), format!("{:?}:{}:{}", path.display(), p_function.starting_line, 1));
                                }
                            },
                        }
                    }
                }
            }
        }

        Self {
            data_types: collected_data_types,
            functions: collected_functions,
            flow_types: collected_flow_types,
        }
    }

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

    /// Checks (recursively) if the defined DataTypes are correct
    pub fn handle_data_type(&self, analysable_data_type: AnalysableDataType, data_type_identifier: DataTypeIdentifier) -> Vec<String> {
        let data_type = analysable_data_type.definition_data_type.clone();
        let path = format!(
            "{:?}:{}:{}",
            Path::new(&analysable_data_type.clone().original_definition.path.unwrap_or_default()).display(),
            analysable_data_type.original_definition.starting_line,
            1
        );
        let id = analysable_data_type.id;
        let mut result = vec![];

        if let Some(r#type) = data_type_identifier.r#type {
            match r#type {
                Type::DataTypeIdentifier(identifier) => {
                    if !self.data_type_identifier_exists(identifier.clone(), id) {
                        error(format!("`{}` uses a undefined data_type: `{}`!", analysable_data_type.definition_data_type.identifier, identifier), path);
                    }
                }
                Type::GenericType(generic) => {
                    if !self.data_type_identifier_exists(generic.data_type_identifier.clone(), id) {
                        error(format!("`{}` uses a undefined data_type: `{}`!", analysable_data_type.definition_data_type.identifier, generic.data_type_identifier), path);
                    }

                    for mapper in generic.generic_mappers {
                        if data_type.generic_keys.contains(&mapper.target) {
                            result.push(mapper.target.clone())
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
            error(format!("`{}` has a data_type that's null!", analysable_data_type.definition_data_type.identifier), path);
        }

        result
    }

    pub fn analyse_data_type(&self, analysable_data_type: AnalysableDataType)  {
        let id = analysable_data_type.id;
        let data_type = analysable_data_type.definition_data_type.clone();
        let path = format!(
            "{:?}:{}:{}",
            Path::new(&analysable_data_type.clone().original_definition.path.unwrap_or_default()).display(),
            analysable_data_type.original_definition.starting_line,
            1
        );
        // Check if Identifier is duplicate
        if self.data_type_identifier_exists(data_type.identifier.clone(), id) {
            error(format!("The data_type `{}` is already defined!", data_type.identifier), path.clone());
        }

        // The variant 0 never should occur
        if data_type.variant == 0 {
            error(format!("The variant of `{}` is 0 and thus incorrect!", data_type.identifier), path.clone());
        }

        // Generic Keys are present. Search if they are referenced!
        if !data_type.generic_keys.is_empty() {
            let mut detected_generic_keys: Vec<String> = vec![];
            if data_type.rules.is_empty() {
                error(format!("`{}` defined generic_keys but never uses one!", data_type.identifier), path.clone());
            }

            for optional_rule in &data_type.rules {
                if let Some(config) = optional_rule.clone().config {
                    match config {
                        Config::ContainsKey(rule) => {
                            if let Some(data_type_identifier) = rule.data_type_identifier {
                                detected_generic_keys.append(&mut self.handle_data_type(analysable_data_type.clone(), data_type_identifier))
                            } else {
                                error(format!("`{}` uses a definition_data_type_contains_key_rule that is null!", data_type.identifier), path.clone());
                            }
                        }
                        Config::ContainsType(rule) => {
                            if let Some(data_type_identifier) = rule.data_type_identifier {
                                detected_generic_keys.append(&mut self.handle_data_type(analysable_data_type.clone(), data_type_identifier))
                            } else {
                                error(format!("`{}` uses a definition_data_type_contains_type_rule that is null!", data_type.identifier), path.clone());
                            }
                        }
                        Config::ItemOfCollection(rule) => {
                            if rule.items.is_empty() {
                                error(format!("`{}` uses a definition_data_type_item_of_collection_rule without any defined items!", data_type.identifier), path.clone());
                            }
                        }
                        Config::NumberRange(_) => {}
                        Config::Regex(_) => {}
                        Config::InputTypes(rule) => {
                            if rule.input_types.is_empty() {
                                error(format!("`{}` uses a definition_data_type_input_types_rule without any defined inputs!", data_type.identifier), path.clone());
                            }

                            for input_type in &rule.input_types {
                                if let Some(data_type_identifier) = &input_type.data_type_identifier {
                                    detected_generic_keys.append(&mut self.handle_data_type(analysable_data_type.clone(), data_type_identifier.clone()))
                                } else {
                                    error(format!("`{}` uses a definition_data_type_input_types_rule that has a undefined data_type!", data_type.identifier), path.clone());
                                }
                            }
                        }
                        Config::ReturnType(rule) => {
                            if let Some(data_type_identifier) = &rule.data_type_identifier {
                                detected_generic_keys.append(&mut self.handle_data_type(analysable_data_type.clone(), data_type_identifier.clone()))
                            } else {
                                error(format!("`{}` uses a definition_data_type_return_type_rule that is null!", data_type.identifier), path.clone());
                            }
                        }
                        Config::ParentType(rule) => {
                            if let Some(data_type_identifier) = &rule.parent_type {
                                detected_generic_keys.append(&mut self.handle_data_type(analysable_data_type.clone(), data_type_identifier.clone()))
                            } else {
                                error(format!("`{}` uses a definition_data_type_parent_type_rule that is null!", data_type.identifier), path.clone());
                            }
                        }
                    }
                }
            }

            let defined_but_unused = data_type.generic_keys.iter().filter(|key| !detected_generic_keys.contains(key)).collect::<Vec<&String>>();
            let used_but_undefined = detected_generic_keys.iter().filter(|key| !data_type.generic_keys.contains(key)).collect::<Vec<&String>>();

            for key in defined_but_unused {
                error(format!("`{}` uses a generic_key (`{}`) that's never used!", analysable_data_type.definition_data_type.identifier, key), path.clone());
            }

            for key in used_but_undefined {
                error(format!("`{}` uses a generic_key (`{}`) that's not defined!", analysable_data_type.definition_data_type.identifier, key), path.clone());
            }
        } else {
            // Check here for any empty configs!
            for rule in &data_type.rules {
                if rule.config.is_none() {
                    error(format!("`{}` uses a rule that is null!", analysable_data_type.definition_data_type.identifier), path.clone());
                }
            }
        }

        // Check if at least one Translation is present
        if data_type.name.is_empty() {
            warning(format!("`{}` has no name defined!", analysable_data_type.definition_data_type.identifier), path.clone());
        }
    }
}

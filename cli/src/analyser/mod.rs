mod diagnostics;

use crate::analyser::diagnostics::DiagnosticKind::{
    DuplicateDataTypeIdentifier, DuplicateFlowTypeIdentifier, DuplicateRuntimeParameterIdentifier,
    EmptyGenericMapper, GenericKeyNotInMappingTarget, NullField, UndefinedDataTypeIdentifier,
    UndefinedGenericKey, UndefinedTranslation, UnusedGenericKey,
};
use crate::analyser::diagnostics::{Diagnose, DiagnosticKind, Reporter};
use code0_definition_reader::parser::Parser;
use code0_definition_reader::reader::{Meta, MetaType, Reader};
use tucana::shared::data_type_identifier::Type;
use tucana::shared::definition_data_type_rule::Config;
use tucana::shared::{DataTypeIdentifier, DefinitionDataType, FlowType, RuntimeFunctionDefinition};

#[derive(Clone)]
pub struct AnalysableDataType {
    pub original_definition: Meta,
    pub definition_data_type: DefinitionDataType,
    pub id: i16,
}

#[derive(Clone)]
pub struct AnalysableFlowType {
    pub original_definition: Meta,
    pub flow_type: FlowType,
    pub id: i16,
}

#[derive(Clone)]
pub struct AnalysableFunction {
    pub original_definition: Meta,
    pub function: RuntimeFunctionDefinition,
    pub id: i16,
}

pub struct Analyser {
    reporter: Reporter,
    pub data_types: Vec<AnalysableDataType>,
    pub flow_types: Vec<AnalysableFlowType>,
    pub functions: Vec<AnalysableFunction>,
}

impl Analyser {
    pub fn new(path: &str) -> Analyser {
        let mut reporter = Reporter::default();
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

        for definition in reader.meta {
            match definition.r#type {
                MetaType::FlowType => {
                    current_index += 1;
                    match serde_json::from_str::<FlowType>(definition.definition_string.as_str()) {
                        Ok(flow_type) => collected_flow_types.push(AnalysableFlowType {
                            original_definition: definition.clone(),
                            flow_type,
                            id: current_index,
                        }),
                        Err(err) => {
                            let name = Parser::extract_identifier(
                                definition.definition_string.as_str(),
                                MetaType::FlowType,
                            );
                            let diagnose = Diagnose::new(
                                name,
                                definition.clone(),
                                DiagnosticKind::DeserializationError {
                                    description: err.to_string(),
                                },
                            );
                            reporter.add_report(diagnose);
                        }
                    }
                }
                MetaType::DataType => {
                    current_index += 1;
                    match serde_json::from_str::<DefinitionDataType>(
                        definition.definition_string.as_str(),
                    ) {
                        Ok(data_type) => collected_data_types.push(AnalysableDataType {
                            original_definition: definition.clone(),
                            definition_data_type: data_type,
                            id: current_index,
                        }),
                        Err(err) => {
                            let name = Parser::extract_identifier(
                                definition.definition_string.as_str(),
                                MetaType::DataType,
                            );
                            let diagnose = Diagnose::new(
                                name,
                                definition.clone(),
                                DiagnosticKind::DeserializationError {
                                    description: err.to_string(),
                                },
                            );
                            reporter.add_report(diagnose);
                        }
                    }
                }
                MetaType::RuntimeFunction => {
                    current_index += 1;
                    match serde_json::from_str::<RuntimeFunctionDefinition>(
                        definition.definition_string.as_str(),
                    ) {
                        Ok(function) => collected_functions.push(AnalysableFunction {
                            original_definition: definition.clone(),
                            function,
                            id: current_index,
                        }),
                        Err(err) => {
                            let name = Parser::extract_identifier(
                                definition.definition_string.as_str(),
                                MetaType::RuntimeFunction,
                            );
                            let diagnose = Diagnose::new(
                                name,
                                definition.clone(),
                                DiagnosticKind::DeserializationError {
                                    description: err.to_string(),
                                },
                            );
                            reporter.add_report(diagnose);
                        }
                    }
                }
            }
        }

        Self {
            reporter,
            data_types: collected_data_types,
            functions: collected_functions,
            flow_types: collected_flow_types,
        }
    }

    pub fn data_type_identifier_exists(&self, identifier: String, id: i16) -> bool {
        for data_types in &self.data_types {
            if id == data_types.id {
                continue;
            }

            if data_types.definition_data_type.identifier.to_lowercase()
                != identifier.to_lowercase()
            {
                continue;
            }
            return true;
        }
        false
    }

    /// Checks (recursively) if the defined DataTypes are correct
    pub fn handle_data_type(
        &mut self,
        analysable_data_type: AnalysableDataType,
        data_type_identifier: DataTypeIdentifier,
    ) -> Vec<String> {
        let data_type = analysable_data_type.definition_data_type.clone();
        let id = analysable_data_type.id;
        let mut result = vec![];

        if let Some(r#type) = data_type_identifier.r#type {
            match r#type {
                Type::DataTypeIdentifier(identifier) => {
                    if !self.data_type_identifier_exists(identifier.clone(), id) {
                        self.reporter.add_report(Diagnose::new(
                            analysable_data_type.definition_data_type.identifier,
                            analysable_data_type.original_definition,
                            UndefinedDataTypeIdentifier { identifier },
                        ));
                    }
                }
                Type::GenericType(generic) => {
                    if !self.data_type_identifier_exists(generic.data_type_identifier.clone(), id) {
                        self.reporter.add_report(Diagnose::new(
                            analysable_data_type.definition_data_type.clone().identifier,
                            analysable_data_type.original_definition.clone(),
                            UndefinedDataTypeIdentifier {
                                identifier: generic.data_type_identifier,
                            },
                        ));
                    }

                    if generic.generic_mappers.is_empty() {
                        self.reporter.add_report(Diagnose::new(
                            analysable_data_type.definition_data_type.clone().identifier,
                            analysable_data_type.original_definition.clone(),
                            EmptyGenericMapper,
                        ))
                    }

                    for mapper in generic.generic_mappers {
                        if data_type.generic_keys.contains(&mapper.target) {
                            result.push(mapper.target.clone())
                        }

                        for source in mapper.source {
                            result.append(
                                &mut self.handle_data_type(analysable_data_type.clone(), source),
                            )
                        }
                    }
                }
                Type::GenericKey(key) => result.push(key.clone()),
            }
        } else {
            self.reporter.add_report(Diagnose::new(
                analysable_data_type.definition_data_type.clone().identifier,
                analysable_data_type.original_definition.clone(),
                NullField {
                    field_name: String::from("data_type"),
                },
            ));
        }

        result
    }

    pub fn analyse_data_type(&mut self, analysable_data_type: AnalysableDataType) {
        let id = analysable_data_type.id;
        let data_type = analysable_data_type.definition_data_type.clone();
        // Check if Identifier is duplicate
        if self.data_type_identifier_exists(data_type.identifier.clone(), id) {
            self.reporter.add_report(Diagnose::new(
                analysable_data_type.definition_data_type.clone().identifier,
                analysable_data_type.original_definition.clone(),
                DuplicateDataTypeIdentifier {
                    identifier: data_type.identifier.clone(),
                },
            ));
        }

        // The variant 0 never should occur
        if data_type.variant == 0 {
            self.reporter.add_report(Diagnose::new(
                analysable_data_type.definition_data_type.clone().identifier,
                analysable_data_type.original_definition.clone(),
                DiagnosticKind::ForbiddenVariant,
            ));
        }

        // Generic Keys are present. Search if they are referenced!
        if !data_type.generic_keys.is_empty() {
            let mut detected_generic_keys: Vec<String> = vec![];

            for optional_rule in &data_type.rules {
                if let Some(config) = optional_rule.clone().config {
                    match config {
                        Config::ContainsKey(rule) => {
                            if let Some(data_type_identifier) = rule.data_type_identifier {
                                detected_generic_keys.append(&mut self.handle_data_type(
                                    analysable_data_type.clone(),
                                    data_type_identifier,
                                ))
                            } else {
                                self.reporter.add_report(Diagnose::new(
                                    analysable_data_type.definition_data_type.clone().identifier,
                                    analysable_data_type.original_definition.clone(),
                                    NullField {
                                        field_name: String::from(
                                            "definition_data_type_contains_key_rule",
                                        ),
                                    },
                                ));
                            }
                        }
                        Config::ContainsType(rule) => {
                            if let Some(data_type_identifier) = rule.data_type_identifier {
                                detected_generic_keys.append(&mut self.handle_data_type(
                                    analysable_data_type.clone(),
                                    data_type_identifier,
                                ))
                            } else {
                                self.reporter.add_report(Diagnose::new(
                                    analysable_data_type.definition_data_type.clone().identifier,
                                    analysable_data_type.original_definition.clone(),
                                    NullField {
                                        field_name: String::from(
                                            "definition_data_type_contains_type_rule",
                                        ),
                                    },
                                ));
                            }
                        }
                        Config::ItemOfCollection(rule) => {
                            if rule.items.is_empty() {
                                self.reporter.add_report(Diagnose::new(
                                    analysable_data_type.definition_data_type.clone().identifier,
                                    analysable_data_type.original_definition.clone(),
                                    NullField {
                                        field_name: String::from(
                                            "definition_data_type_item_of_collection_rule",
                                        ),
                                    },
                                ));
                            }
                        }
                        Config::NumberRange(_) => {}
                        Config::Regex(_) => {}
                        Config::InputTypes(rule) => {
                            if rule.input_types.is_empty() {
                                self.reporter.add_report(Diagnose::new(
                                    analysable_data_type.definition_data_type.clone().identifier,
                                    analysable_data_type.original_definition.clone(),
                                    NullField {
                                        field_name: String::from(
                                            "definition_data_type_input_types_rule",
                                        ),
                                    },
                                ));
                            }

                            for input_type in &rule.input_types {
                                if let Some(data_type_identifier) = &input_type.data_type_identifier
                                {
                                    detected_generic_keys.append(&mut self.handle_data_type(
                                        analysable_data_type.clone(),
                                        data_type_identifier.clone(),
                                    ))
                                } else {
                                    self.reporter.add_report(Diagnose::new(
                                        analysable_data_type
                                            .definition_data_type
                                            .clone()
                                            .identifier,
                                        analysable_data_type.original_definition.clone(),
                                        UndefinedDataTypeIdentifier {
                                            identifier: data_type.identifier.clone(),
                                        },
                                    ));
                                }
                            }
                        }
                        Config::ReturnType(rule) => {
                            if let Some(data_type_identifier) = &rule.data_type_identifier {
                                detected_generic_keys.append(&mut self.handle_data_type(
                                    analysable_data_type.clone(),
                                    data_type_identifier.clone(),
                                ))
                            } else {
                                self.reporter.add_report(Diagnose::new(
                                    analysable_data_type.definition_data_type.clone().identifier,
                                    analysable_data_type.original_definition.clone(),
                                    NullField {
                                        field_name: String::from(
                                            "definition_data_type_return_type_rule",
                                        ),
                                    },
                                ));
                            }
                        }
                        Config::ParentType(rule) => {
                            if let Some(data_type_identifier) = &rule.parent_type {
                                detected_generic_keys.append(&mut self.handle_data_type(
                                    analysable_data_type.clone(),
                                    data_type_identifier.clone(),
                                ))
                            } else {
                                self.reporter.add_report(Diagnose::new(
                                    analysable_data_type.definition_data_type.clone().identifier,
                                    analysable_data_type.original_definition.clone(),
                                    NullField {
                                        field_name: String::from(
                                            "definition_data_type_parent_type_rule",
                                        ),
                                    },
                                ));
                            }
                        }
                    }
                }
            }

            let defined_but_unused = data_type
                .generic_keys
                .iter()
                .filter(|key| !detected_generic_keys.contains(key))
                .collect::<Vec<&String>>();
            let used_but_undefined = detected_generic_keys
                .iter()
                .filter(|key| !data_type.generic_keys.contains(key))
                .collect::<Vec<&String>>();

            for key in defined_but_unused {
                self.reporter.add_report(Diagnose::new(
                    analysable_data_type.definition_data_type.clone().identifier,
                    analysable_data_type.original_definition.clone(),
                    UnusedGenericKey { key: key.clone() },
                ));
            }

            for key in used_but_undefined {
                self.reporter.add_report(Diagnose::new(
                    analysable_data_type.definition_data_type.clone().identifier,
                    analysable_data_type.original_definition.clone(),
                    UndefinedGenericKey { key: key.clone() },
                ));
            }
        } else {
            // Check here for any empty configs!
            for rule in &data_type.rules {
                if rule.config.is_none() {
                    self.reporter.add_report(Diagnose::new(
                        analysable_data_type.definition_data_type.clone().identifier,
                        analysable_data_type.original_definition.clone(),
                        NullField {
                            field_name: String::from("rule"),
                        },
                    ));
                }
            }
        }

        // Check if at least one Translation is present
        if data_type.name.is_empty() {
            self.reporter.add_report(Diagnose::new(
                analysable_data_type.definition_data_type.clone().identifier,
                analysable_data_type.original_definition.clone(),
                UndefinedTranslation {
                    translation_field: String::from("name"),
                },
            ));
        }
    }

    pub fn analyse_flow_type(&mut self, analysable_flow_type: AnalysableFlowType) {
        let flow = analysable_flow_type.flow_type.clone();
        let original_definition = analysable_flow_type.original_definition;
        let name = flow.identifier;

        // Check if at least one Translation is present
        if flow.name.is_empty() {
            self.reporter.add_report(Diagnose::new(
                name.clone(),
                original_definition.clone(),
                UndefinedTranslation {
                    translation_field: String::from("name"),
                },
            ));
        }

        if flow.description.is_empty() {
            self.reporter.add_report(Diagnose::new(
                name.clone(),
                original_definition.clone(),
                UndefinedTranslation {
                    translation_field: String::from("description"),
                },
            ));
        }

        if flow.documentation.is_empty() {
            self.reporter.add_report(Diagnose::new(
                name.clone(),
                original_definition.clone(),
                UndefinedTranslation {
                    translation_field: String::from("documentation"),
                },
            ));
        }

        // Check if input identifier exists
        if let Some(identifier) = flow.input_type_identifier {
            if !self.data_type_identifier_exists(identifier.clone(), -1) {
                self.reporter.add_report(Diagnose::new(
                    name.clone(),
                    original_definition.clone(),
                    UndefinedDataTypeIdentifier { identifier },
                ));
            }
        }

        // Check if return identifier exists
        if let Some(identifier) = flow.return_type_identifier {
            if !self.data_type_identifier_exists(identifier.clone(), -1) {
                self.reporter.add_report(Diagnose::new(
                    name.clone(),
                    original_definition.clone(),
                    UndefinedDataTypeIdentifier { identifier },
                ));
            }
        }

        // Check if flow type identifier already exists
        for flow_type in &self.flow_types {
            if analysable_flow_type.id == flow_type.id {
                continue;
            }

            if flow_type.flow_type.identifier.to_lowercase() == name.clone().to_lowercase() {
                self.reporter.add_report(Diagnose::new(
                    name.clone(),
                    original_definition.clone(),
                    DuplicateFlowTypeIdentifier { identifier: name },
                ));
                break;
            }
        }
    }

    pub fn analyse_runtime_function(&mut self, analysable_function: AnalysableFunction) {
        let name = analysable_function.function.runtime_name.clone();
        let function = analysable_function.function;
        let original = analysable_function.original_definition;
        let id = analysable_function.id;

        // Check if at least one Translation is present
        if function.name.is_empty() {
            self.reporter.add_report(Diagnose::new(
                name.clone(),
                original.clone(),
                UndefinedTranslation {
                    translation_field: String::from("name"),
                },
            ));
        }

        if function.description.is_empty() {
            self.reporter.add_report(Diagnose::new(
                name.clone(),
                original.clone(),
                UndefinedTranslation {
                    translation_field: String::from("description"),
                },
            ));
        }

        if function.documentation.is_empty() {
            self.reporter.add_report(Diagnose::new(
                name.clone(),
                original.clone(),
                UndefinedTranslation {
                    translation_field: String::from("documentation"),
                },
            ));
        }

        // Check if runtime function  already exists
        for func in &self.functions {
            if func.id == id {
                continue;
            }

            if func.function.runtime_name.to_lowercase() == name.clone().to_lowercase() {
                self.reporter.add_report(Diagnose::new(
                    name.clone(),
                    original.clone(),
                    DuplicateFlowTypeIdentifier {
                        identifier: name.clone(),
                    },
                ));
                break;
            }
        }

        let mut detected_generic_keys: Vec<String> = vec![];
        if let Some(identifier) = function.return_type_identifier {
            detected_generic_keys.append(&mut self.handle_function_data_type_identifier(
                name.clone(),
                original.clone(),
                identifier,
            ));
        }

        let mut parameter_names: Vec<String> = vec![];
        for parameter in function.runtime_parameter_definitions {
            // Check if at least one Translation is present
            if parameter.name.is_empty() {
                self.reporter.add_report(Diagnose::new(
                    name.clone(),
                    original.clone(),
                    UndefinedTranslation {
                        translation_field: String::from("name"),
                    },
                ));
            }

            if parameter.description.is_empty() {
                self.reporter.add_report(Diagnose::new(
                    name.clone(),
                    original.clone(),
                    UndefinedTranslation {
                        translation_field: String::from("description"),
                    },
                ));
            }

            if parameter.documentation.is_empty() {
                self.reporter.add_report(Diagnose::new(
                    name.clone(),
                    original.clone(),
                    UndefinedTranslation {
                        translation_field: String::from("documentation"),
                    },
                ));
            }

            // Check if data_type exists
            if let Some(identifier) = parameter.data_type_identifier {
                detected_generic_keys.append(&mut self.handle_function_data_type_identifier(
                    name.clone(),
                    original.clone(),
                    identifier,
                ));
            } else {
                self.reporter.add_report(Diagnose::new(
                    name.clone(),
                    original.clone(),
                    NullField {
                        field_name: String::from("data_type"),
                    },
                ));
            }

            if parameter_names.contains(&parameter.runtime_name) {
                self.reporter.add_report(Diagnose::new(
                    name.clone(),
                    original.clone(),
                    DuplicateRuntimeParameterIdentifier {
                        identifier: parameter.runtime_name.clone(),
                    },
                ));
            }

            parameter_names.push(parameter.runtime_name);
        }

        let defined_but_unused = function
            .generic_keys
            .iter()
            .filter(|key| !detected_generic_keys.contains(key))
            .collect::<Vec<&String>>();
        let used_but_undefined = detected_generic_keys
            .iter()
            .filter(|key| !function.generic_keys.contains(key))
            .collect::<Vec<&String>>();

        for key in defined_but_unused {
            self.reporter.add_report(Diagnose::new(
                name.clone(),
                original.clone(),
                UnusedGenericKey { key: key.clone() },
            ));
        }

        for key in used_but_undefined {
            self.reporter.add_report(Diagnose::new(
                name.clone(),
                original.clone(),
                UndefinedGenericKey { key: key.clone() },
            ));
        }
    }

    fn handle_function_data_type_identifier(
        &mut self,
        name: String,
        original: Meta,
        identifier: DataTypeIdentifier,
    ) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        if let Some(r#type) = identifier.r#type {
            match r#type {
                Type::DataTypeIdentifier(data_type) => {
                    if !self.data_type_identifier_exists(data_type.clone(), -1) {
                        self.reporter.add_report(Diagnose::new(
                            name.clone(),
                            original.clone(),
                            UndefinedDataTypeIdentifier {
                                identifier: data_type.clone(),
                            },
                        ))
                    };
                }
                Type::GenericType(generic_type) => {
                    if !self
                        .data_type_identifier_exists(generic_type.data_type_identifier.clone(), -1)
                    {
                        self.reporter.add_report(Diagnose::new(
                            name.clone(),
                            original.clone(),
                            UndefinedDataTypeIdentifier {
                                identifier: generic_type.data_type_identifier.clone(),
                            },
                        ))
                    }

                    if generic_type.generic_mappers.is_empty() {
                        self.reporter.add_report(Diagnose::new(
                            name.clone(),
                            original.clone(),
                            EmptyGenericMapper,
                        ))
                    }

                    for mapper in &generic_type.generic_mappers {
                        for source in mapper.source.clone() {
                            result.append(&mut self.handle_function_data_type_identifier(
                                name.clone(),
                                original.clone(),
                                source,
                            ))
                        }

                        if !self.generic_key_in_target(
                            mapper.target.clone(),
                            generic_type.data_type_identifier.clone(),
                        ) {
                            self.reporter.add_report(Diagnose::new(
                                name.clone(),
                                original.clone(),
                                GenericKeyNotInMappingTarget {
                                    key: mapper.target.clone(),
                                    target: generic_type.data_type_identifier.clone(),
                                },
                            ))
                        }
                    }
                }
                Type::GenericKey(key) => result.push(key.clone()),
            }
        }

        result
    }

    fn generic_key_in_target(&mut self, key: String, target: String) -> bool {
        let data_types: Vec<DefinitionDataType> = self
            .data_types
            .iter()
            .map(|d| d.definition_data_type.clone())
            .collect();
        for data_type in data_types {
            if target.to_lowercase() != data_type.identifier.to_lowercase() {
                continue;
            }

            return data_type.generic_keys.contains(&key);
        }

        false
    }

    pub fn report(&mut self, will_exit: bool) {
        for data_type in self.data_types.clone() {
            self.analyse_data_type(data_type.clone());
        }

        for flow_type in self.flow_types.clone() {
            self.analyse_flow_type(flow_type.clone());
        }

        for functions in self.functions.clone() {
            self.analyse_runtime_function(functions.clone());
        }

        self.reporter.run_report(will_exit);
    }
}

use super::core::{
    AnalysableDataType, AnalysableFlowType, AnalysableFunction, AnalysableFunctionDefinition,
    AnalysableModuleConfigurationDefinition, AnalysableModuleDefinition, AnalysableRuntimeFlowType,
    Analyser,
};
use crate::diagnostics::diagnose::Diagnose;
use crate::diagnostics::kinds::DiagnosticKind;
use crate::diagnostics::reporter::Reporter;
use crate::parser::{ModuleConfiguration, Parser};
use crate::reader::MetaType;
use crate::{analyser::index_identifier::IdentifierIndex, reader::Reader};
use tucana::shared::{
    DefinitionDataType, FlowType, FunctionDefinition, ModuleConfigurationDefinition,
    RuntimeFlowType, RuntimeFunctionDefinition,
};

pub fn load_from_path(path: &str) -> Analyser {
    let mut reporter = Reporter::default();
    let reader = Reader::from_path(path).expect("No definitions behind this path");

    let mut current_index: i16 = 0;
    let mut collected_data_types: Vec<AnalysableDataType> = vec![];
    let mut collected_flow_types: Vec<AnalysableFlowType> = vec![];
    let mut collected_functions: Vec<AnalysableFunction> = vec![];
    let mut collected_runtime_flow_types: Vec<AnalysableRuntimeFlowType> = vec![];
    let mut collected_function_definitions: Vec<AnalysableFunctionDefinition> = vec![];
    let mut collected_module_configuration_definitions: Vec<
        AnalysableModuleConfigurationDefinition,
    > = vec![];
    let mut collected_module_definitions: Vec<AnalysableModuleDefinition> = vec![];
    let mut index = IdentifierIndex::default();

    for definition in reader.meta {
        match definition.r#type {
            MetaType::FlowType => {
                current_index += 1;
                match serde_json::from_str::<FlowType>(definition.definition_string.as_str()) {
                    Ok(flow_type) => {
                        if let Some(_prev) =
                            index.insert_flow_type(&flow_type.identifier, current_index)
                        {
                            reporter.add(Diagnose::new(
                                flow_type.identifier.clone(),
                                definition.clone(),
                                DiagnosticKind::DuplicateFlowTypeIdentifier {
                                    identifier: flow_type.identifier.clone(),
                                },
                            ));
                        }
                        collected_flow_types.push(AnalysableFlowType {
                            original_definition: definition.clone(),
                            flow_type,
                            id: current_index,
                        });
                    }
                    Err(err) => {
                        let name = Parser::extract_identifier(
                            definition.definition_string.as_str(),
                            MetaType::FlowType,
                        );
                        reporter.add(Diagnose::new(
                            name,
                            definition.clone(),
                            DiagnosticKind::DeserializationError {
                                description: err.to_string(),
                            },
                        ));
                    }
                }
            }
            MetaType::DataType => {
                current_index += 1;
                match serde_json::from_str::<DefinitionDataType>(
                    definition.definition_string.as_str(),
                ) {
                    Ok(data_type) => {
                        if let Some(_prev) =
                            index.insert_data_type(&data_type.identifier, current_index)
                        {
                            reporter.add(Diagnose::new(
                                data_type.identifier.clone(),
                                definition.clone(),
                                DiagnosticKind::DuplicateDataTypeIdentifier {
                                    identifier: data_type.identifier.clone(),
                                },
                            ));
                        }
                        collected_data_types.push(AnalysableDataType {
                            original_definition: definition.clone(),
                            definition_data_type: data_type,
                            id: current_index,
                        });
                    }
                    Err(err) => {
                        let name = Parser::extract_identifier(
                            definition.definition_string.as_str(),
                            MetaType::DataType,
                        );
                        reporter.add(Diagnose::new(
                            name,
                            definition.clone(),
                            DiagnosticKind::DeserializationError {
                                description: err.to_string(),
                            },
                        ));
                    }
                }
            }
            MetaType::RuntimeFunction => {
                current_index += 1;
                match serde_json::from_str::<RuntimeFunctionDefinition>(
                    definition.definition_string.as_str(),
                ) {
                    Ok(function) => {
                        if let Some(_prev) =
                            index.insert_function(&function.runtime_name, current_index)
                        {
                            reporter.add(Diagnose::new(
                                function.runtime_name.clone(),
                                definition.clone(),
                                DiagnosticKind::DuplicateRuntimeFunctionIdentifier {
                                    identifier: function.runtime_name.clone(),
                                },
                            ));
                        }
                        collected_functions.push(AnalysableFunction {
                            original_definition: definition.clone(),
                            function,
                            id: current_index,
                        });
                    }
                    Err(err) => {
                        let name = Parser::extract_identifier(
                            definition.definition_string.as_str(),
                            MetaType::RuntimeFunction,
                        );
                        reporter.add(Diagnose::new(
                            name,
                            definition.clone(),
                            DiagnosticKind::DeserializationError {
                                description: err.to_string(),
                            },
                        ));
                    }
                }
            }
            MetaType::RuntimeFlowType => {
                current_index += 1;
                match serde_json::from_str::<RuntimeFlowType>(definition.definition_string.as_str())
                {
                    Ok(runtime_flow_type) => {
                        if let Some(_prev) = index
                            .insert_runtime_flow_type(&runtime_flow_type.identifier, current_index)
                        {
                            reporter.add(Diagnose::new(
                                runtime_flow_type.identifier.clone(),
                                definition.clone(),
                                DiagnosticKind::DuplicateRuntimeFlowTypeIdentifier {
                                    identifier: runtime_flow_type.identifier.clone(),
                                },
                            ));
                        }
                        collected_runtime_flow_types.push(AnalysableRuntimeFlowType {
                            original_definition: definition.clone(),
                            runtime_flow_type,
                            id: current_index,
                        });
                    }
                    Err(err) => {
                        let name = Parser::extract_identifier(
                            definition.definition_string.as_str(),
                            MetaType::RuntimeFlowType,
                        );
                        reporter.add(Diagnose::new(
                            name,
                            definition.clone(),
                            DiagnosticKind::DeserializationError {
                                description: err.to_string(),
                            },
                        ));
                    }
                }
            }
            MetaType::Function => {
                current_index += 1;
                match serde_json::from_str::<FunctionDefinition>(
                    definition.definition_string.as_str(),
                ) {
                    Ok(function_definition) => {
                        if let Some(_prev) = index.insert_function_definition(
                            &function_definition.runtime_name,
                            current_index,
                        ) {
                            reporter.add(Diagnose::new(
                                function_definition.runtime_name.clone(),
                                definition.clone(),
                                DiagnosticKind::DuplicateFunctionIdentifier {
                                    identifier: function_definition.runtime_name.clone(),
                                },
                            ));
                        }
                        collected_function_definitions.push(AnalysableFunctionDefinition {
                            original_definition: definition.clone(),
                            function_definition,
                            id: current_index,
                        });
                    }
                    Err(err) => {
                        let name = Parser::extract_identifier(
                            definition.definition_string.as_str(),
                            MetaType::Function,
                        );
                        reporter.add(Diagnose::new(
                            name,
                            definition.clone(),
                            DiagnosticKind::DeserializationError {
                                description: err.to_string(),
                            },
                        ));
                    }
                }
            }
            MetaType::Configs => {
                current_index += 1;
                match serde_json::from_str::<ModuleConfigurationDefinition>(
                    definition.definition_string.as_str(),
                ) {
                    Ok(module_configuration_definition) => {
                        if let Some(_prev) = index.insert_module_configuration_definition(
                            &module_configuration_definition.identifier,
                            current_index,
                        ) {
                            reporter.add(Diagnose::new(
                                module_configuration_definition.identifier.clone(),
                                definition.clone(),
                                DiagnosticKind::DuplicateModuleConfigurationIdentifier {
                                    identifier: module_configuration_definition.identifier.clone(),
                                },
                            ));
                        }
                        collected_module_configuration_definitions.push(
                            AnalysableModuleConfigurationDefinition {
                                original_definition: definition.clone(),
                                module_configuration_definition,
                                id: current_index,
                            },
                        );
                    }
                    Err(err) => {
                        let name = Parser::extract_identifier(
                            definition.definition_string.as_str(),
                            MetaType::Configs,
                        );
                        reporter.add(Diagnose::new(
                            name,
                            definition.clone(),
                            DiagnosticKind::DeserializationError {
                                description: err.to_string(),
                            },
                        ));
                    }
                }
            }
            MetaType::ModuleDefinition => {
                current_index += 1;
                match serde_json::from_str::<ModuleConfiguration>(
                    definition.definition_string.as_str(),
                ) {
                    Ok(module_definition) => {
                        collected_module_definitions.push(AnalysableModuleDefinition {
                            original_definition: definition.clone(),
                            module_definition,
                            id: current_index,
                        });
                    }
                    Err(err) => {
                        let name = Parser::extract_identifier(
                            definition.definition_string.as_str(),
                            MetaType::ModuleDefinition,
                        );
                        reporter.add(Diagnose::new(
                            name,
                            definition.clone(),
                            DiagnosticKind::DeserializationError {
                                description: err.to_string(),
                            },
                        ));
                    }
                }
            }
        }
    }
    Analyser {
        reporter,
        index,
        data_types: collected_data_types,
        flow_types: collected_flow_types,
        functions: collected_functions,
        runtime_flow_types: collected_runtime_flow_types,
        function_definitions: collected_function_definitions,
        module_configuration_definitions: collected_module_configuration_definitions,
        module_definitions: collected_module_definitions,
    }
}

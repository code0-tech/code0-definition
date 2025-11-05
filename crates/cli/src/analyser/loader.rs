use super::core::{AnalysableDataType, AnalysableFlowType, AnalysableFunction, Analyser};
use crate::analyser::index_identifier::IdentifierIndex;
use crate::diagnostics::diagnose::Diagnose;
use crate::diagnostics::kinds::DiagnosticKind;
use crate::diagnostics::reporter::Reporter;
use crate::parser::{MetaType, Parser, Reader};
use tucana::shared::{DefinitionDataType, FlowType, RuntimeFunctionDefinition};

pub fn load_from_path(path: &str) -> Analyser {
    let mut reporter = Reporter::default();
    let reader = Reader::from_path(path).expect("No definitions behind this path");

    let mut current_index: i16 = 0;
    let mut collected_data_types: Vec<AnalysableDataType> = vec![];
    let mut collected_flow_types: Vec<AnalysableFlowType> = vec![];
    let mut collected_functions: Vec<AnalysableFunction> = vec![];
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
        }
    }
    Analyser {
        reporter,
        index,
        data_types: collected_data_types,
        flow_types: collected_flow_types,
        functions: collected_functions,
    }
}

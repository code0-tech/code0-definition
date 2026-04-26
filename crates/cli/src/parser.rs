use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tucana::shared::{
    DefinitionDataType, FlowType, FunctionDefinition, Module, ModuleConfigurationDefinition,
    RuntimeFlowType, RuntimeFunctionDefinition, Translation,
};

use crate::reader::{Meta, MetaType, Reader};

#[derive(Serialize, Clone, Debug)]
pub struct DefinitionError {
    pub definition: String,
    pub definition_type: crate::reader::MetaType,
    pub path: String,
    pub error: String,
}

#[derive(Debug)]
pub struct Parser {
    pub modules: Vec<DefinitionModule>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ModuleConfiguration {
    pub identifier: String,
    pub name: Vec<Translation>,
    pub description: Vec<Translation>,
    pub documentation: String,
    pub author: String,
    pub icon: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub version: String,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct DefinitionModule {
    // Configuration of module `module.json`
    pub config: ModuleConfiguration,
    // DataTypes of module `/data_types`
    pub data_types: Vec<DefinitionDataType>,
    // FlowTypes of module `/flow_types`
    pub flow_types: Vec<FlowType>,
    // RuntimeFlowTypes of module `/runtime_flow_types`
    pub runtime_flow_types: Vec<RuntimeFlowType>,
    // Functions of module `/functions`
    pub functions: Vec<FunctionDefinition>,
    // RuntimeFunctions of module `/runtime_functions`
    pub runtime_functions: Vec<RuntimeFunctionDefinition>,
    // Settings of module `/configurations`
    pub module_configs: Vec<ModuleConfigurationDefinition>,

    // Errors found while parsing
    pub errors: Vec<DefinitionError>,
}

impl DefinitionModule {
    pub fn into_module(self) -> Module {
        Module {
            identifier: self.config.identifier,
            name: self.config.name,
            description: self.config.description,
            documentation: self.config.documentation,
            author: self.config.author,
            icon: self.config.icon,
            version: self.config.version,
            flow_types: self.flow_types,
            runtime_flow_types: self.runtime_flow_types,
            function_definitions: self.functions,
            runtime_function_definitions: self.runtime_functions,
            definition_data_types: self.data_types,
            configurations: self.module_configs,
        }
    }
}

impl Parser {
    pub fn from_path(path: &str) -> Option<Self> {
        let reader = Reader::from_path(path)?;

        Some(Self::from_reader(reader))
    }

    pub fn from_reader(reader: Reader) -> Self {
        let mut modules: Vec<DefinitionModule> = vec![];
        let mut module_indices_by_name: HashMap<String, usize> = HashMap::new();

        for meta in &reader.meta {
            let module_index = if let Some(index) = module_indices_by_name.get(&meta.name) {
                *index
            } else {
                let mut new_mod = DefinitionModule::default();
                Parser::append_meta(&mut new_mod, meta);
                modules.push(new_mod);
                let new_index = modules.len() - 1;
                module_indices_by_name.insert(meta.name.clone(), new_index);
                continue;
            };

            Parser::append_meta(&mut modules[module_index], meta);
        }

        Parser { modules }
    }

    pub fn extract_identifier(definition: &str, meta_type: MetaType) -> String {
        let field_name = match meta_type {
            MetaType::RuntimeFunction | MetaType::Function => "runtime_name",
            _ => "identifier",
        };

        // Look for the field pattern: "field_name": "value" or "field_name":"value"
        if let Some(start) = definition.find(&format!("\"{field_name}\"")) {
            // Find the colon after the field name
            if let Some(colon_pos) = definition[start..].find(':') {
                let after_colon = &definition[start + colon_pos + 1..];

                // Skip whitespace and find the opening quote
                let trimmed = after_colon.trim_start();
                if let Some(stripped) = trimmed.strip_prefix('"') {
                    // Find the closing quote
                    if let Some(end_quote) = stripped.find('"') {
                        return trimmed[1..end_quote + 1].to_string();
                    }
                }
            }
        }

        // Fallback: return the whole definition if identifier can't be extracted
        definition.to_string()
    }

    fn append_meta(feature: &mut DefinitionModule, meta: &Meta) {
        let definition = meta.definition_string.as_str();
        match meta.r#type {
            MetaType::DataType => match serde_json::from_str::<DefinitionDataType>(definition) {
                Ok(data_type) => feature.data_types.push(data_type),
                Err(err) => feature.errors.push(DefinitionError {
                    definition: Parser::extract_identifier(definition, MetaType::DataType),
                    definition_type: MetaType::DataType,
                    path: meta.path.clone(),
                    error: err.to_string(),
                }),
            },
            MetaType::FlowType => match serde_json::from_str::<FlowType>(definition) {
                Ok(flow_type) => feature.flow_types.push(flow_type),
                Err(err) => feature.errors.push(DefinitionError {
                    definition: Parser::extract_identifier(definition, MetaType::FlowType),
                    definition_type: MetaType::FlowType,
                    path: meta.path.clone(),
                    error: err.to_string(),
                }),
            },
            MetaType::RuntimeFunction => {
                match serde_json::from_str::<RuntimeFunctionDefinition>(definition) {
                    Ok(func) => feature.runtime_functions.push(func),
                    Err(err) => feature.errors.push(DefinitionError {
                        definition: Parser::extract_identifier(
                            definition,
                            MetaType::RuntimeFunction,
                        ),
                        definition_type: MetaType::RuntimeFunction,
                        path: meta.path.clone(),
                        error: err.to_string(),
                    }),
                }
            }
            MetaType::RuntimeFlowType => {
                match serde_json::from_str::<RuntimeFlowType>(definition) {
                    Ok(v) => feature.runtime_flow_types.push(v),
                    Err(err) => feature.errors.push(DefinitionError {
                        definition: Parser::extract_identifier(
                            definition,
                            MetaType::RuntimeFlowType,
                        ),
                        definition_type: MetaType::RuntimeFlowType,
                        path: meta.path.clone(),
                        error: err.to_string(),
                    }),
                }
            }
            MetaType::Function => match serde_json::from_str::<FunctionDefinition>(definition) {
                Ok(v) => feature.functions.push(v),
                Err(err) => feature.errors.push(DefinitionError {
                    definition: Parser::extract_identifier(definition, MetaType::Function),
                    definition_type: MetaType::Function,
                    path: meta.path.clone(),
                    error: err.to_string(),
                }),
            },
            MetaType::Configs => {
                match serde_json::from_str::<ModuleConfigurationDefinition>(definition) {
                    Ok(v) => feature.module_configs.push(v),
                    Err(err) => feature.errors.push(DefinitionError {
                        definition: Parser::extract_identifier(definition, MetaType::Configs),
                        definition_type: MetaType::Configs,
                        path: meta.path.clone(),
                        error: err.to_string(),
                    }),
                }
            }
            MetaType::ModuleDefinition => {
                match serde_json::from_str::<ModuleConfiguration>(definition) {
                    Ok(v) => feature.config = v,
                    Err(err) => feature.errors.push(DefinitionError {
                        definition: Parser::extract_identifier(
                            definition,
                            MetaType::ModuleDefinition,
                        ),
                        definition_type: MetaType::ModuleDefinition,
                        path: meta.path.clone(),
                        error: err.to_string(),
                    }),
                }
            }
        }
    }
}

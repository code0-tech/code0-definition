use crate::reader::{MetaType, Reader};
use serde::Serialize;
use tucana::shared::{DefinitionDataType, FlowType, RuntimeFunctionDefinition};

#[derive(Serialize, Clone, Debug)]
pub struct DefinitionError {
    pub definition: String,
    pub definition_type: MetaType,
    pub error: String,
}

#[derive(Debug)]
pub struct Parser {
    pub features: Vec<Feature>,
}

#[derive(Serialize, Clone, Debug)]
pub struct Feature {
    pub name: String,
    pub data_types: Vec<DefinitionDataType>,
    pub flow_types: Vec<FlowType>,
    pub runtime_functions: Vec<RuntimeFunctionDefinition>,
    pub errors: Vec<DefinitionError>,
}

impl Feature {
    fn new(name: String) -> Self {
        Feature {
            name,
            data_types: Vec::new(),
            flow_types: Vec::new(),
            runtime_functions: Vec::new(),
            errors: Vec::new(),
        }
    }
}

impl Parser {

    pub fn from_path(path: &str) -> Option<Self> {
        let reader = match Reader::from_path(path) {
            Some(reader) => reader,
            None => return None,
        };

        Some(Self::from_reader(reader))
    }

    pub fn from_reader(reader: Reader) -> Self {
        let mut features: Vec<Feature> = vec![];

        for meta in &reader.meta {
            let feature = features.iter_mut().find(|f| f.name == meta.name);

            if let Some(existing) = feature {
                Parser::append_meta(existing, meta);
            } else {
                let mut new_feature = Feature::new(meta.name.clone());
                Parser::append_meta(&mut new_feature, meta);
                features.push(new_feature);
            }
        }

        Parser { features }
    }

    fn append_meta(feature: &mut Feature, meta: &crate::reader::Meta) {
        for definition in &meta.data {
            match meta.r#type {
                MetaType::DataType => {
                    match serde_json::from_str::<DefinitionDataType>(definition) {
                        Ok(data_type) => feature.data_types.push(data_type),
                        Err(err) => feature.errors.push(DefinitionError {
                            definition: definition.to_string(),
                            definition_type: MetaType::DataType,
                            error: err.to_string()
                        })
                    }
                }
                MetaType::FlowType => match serde_json::from_str::<FlowType>(definition) {
                    Ok(flow_type) => feature.flow_types.push(flow_type),
                    Err(err) => feature.errors.push(DefinitionError {
                        definition: definition.to_string(),
                        definition_type: MetaType::FlowType,
                        error: err.to_string()
                    })
                },
                MetaType::RuntimeFunction => {
                    match serde_json::from_str::<RuntimeFunctionDefinition>(definition) {
                        Ok(func) => feature.runtime_functions.push(func),
                        Err(err) => feature.errors.push(DefinitionError {
                            definition: definition.to_string(),
                            definition_type: MetaType::RuntimeFunction,
                            error: err.to_string()
                        })
                    }
                }
            }
        }
    }
}

use serde::Serialize;
use std::io::ErrorKind;
use std::{
    fs::{self, DirEntry},
    io::Error,
    path::Path,
};
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
        let reader = Reader::from_path(path)?;

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

    pub fn extract_identifier(definition: &str, meta_type: MetaType) -> String {
        let field_name = match meta_type {
            MetaType::DataType | MetaType::FlowType => "identifier",
            MetaType::RuntimeFunction => "runtime_name",
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

    fn append_meta(feature: &mut Feature, meta: &Meta) {
        let definition = meta.definition_string.as_str();
        match meta.r#type {
            MetaType::DataType => match serde_json::from_str::<DefinitionDataType>(definition) {
                Ok(data_type) => feature.data_types.push(data_type),
                Err(err) => feature.errors.push(DefinitionError {
                    definition: Parser::extract_identifier(definition, MetaType::DataType),
                    definition_type: MetaType::DataType,
                    error: err.to_string(),
                }),
            },
            MetaType::FlowType => match serde_json::from_str::<FlowType>(definition) {
                Ok(flow_type) => feature.flow_types.push(flow_type),
                Err(err) => feature.errors.push(DefinitionError {
                    definition: Parser::extract_identifier(definition, MetaType::FlowType),
                    definition_type: MetaType::FlowType,
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
                        error: err.to_string(),
                    }),
                }
            }
        }
    }
}

#[derive(Serialize, Debug, Clone, Copy)]
pub enum MetaType {
    FlowType,
    DataType,
    RuntimeFunction,
}

impl std::fmt::Display for MetaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaType::FlowType => write!(f, "FlowType"),
            MetaType::DataType => write!(f, "DataType"),
            MetaType::RuntimeFunction => write!(f, "RuntimeFunction"),
        }
    }
}

pub struct Reader {
    pub meta: Vec<Meta>,
}

#[derive(Clone, Debug)]
pub struct Meta {
    pub name: String,
    pub r#type: MetaType,
    pub definition_string: String,
    pub path: String,
}

impl Meta {
    pub fn read_from_file<P>(name: String, r#type: MetaType, file_path: P) -> Result<Meta, Error>
    where
        P: AsRef<Path>,
    {
        let path = match file_path.as_ref().to_str() {
            Some(path) => path,
            None => return Err(Error::new(ErrorKind::InvalidInput, "Invalid path")),
        };

        if !path.ends_with("json") {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "File {} does not end with .json",
                    file_path.as_ref().display()
                ),
            ));
        }

        let content = match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(err) => {
                println!("Error reading file: {err}");
                return Err(err);
            }
        };

        Ok(Meta {
            name,
            r#type,
            definition_string: content,
            path: path.to_string(),
        })
    }
}

/// Reader
///
/// Expecting the file system to look like:
/// - <path>
///   - <feature>
///     - <flow_types>
///     - <data_types>
///     - <runtime_functions>
///    - <feature>
///     - <flow_types>
///     - <data_types>
///     - <runtime_functions>
impl Reader {
    pub fn from_path(path: &str) -> Option<Reader> {
        let mut result: Vec<Meta> = vec![];

        // Reading the path folder
        for feature_path in fs::read_dir(path).unwrap() {
            let feature_path_result = match feature_path {
                Ok(path) => path,
                Err(_) => continue,
            };

            let feature_name = match get_file_name(&feature_path_result) {
                Some(file_name) => file_name,
                None => continue,
            };

            // Reading the feature folder
            for type_path in fs::read_dir(feature_path_result.path()).unwrap() {
                let type_path_result = match type_path {
                    Ok(path) => path,
                    Err(_) => continue,
                };

                let meta_type = match get_file_name(&type_path_result) {
                    Some(name) => match name.as_str() {
                        "flow_type" => MetaType::FlowType,
                        "data_type" => MetaType::DataType,
                        "runtime_definition" => MetaType::RuntimeFunction,
                        _ => continue,
                    },
                    None => continue,
                };

                // Reading the type folder
                for definition_path in fs::read_dir(type_path_result.path()).unwrap() {
                    let definition_path_result = match definition_path {
                        Ok(path) => path,
                        Err(_) => continue,
                    };

                    if definition_path_result.file_type().unwrap().is_file() {
                        let meta = Meta::read_from_file(
                            feature_name.clone(),
                            meta_type,
                            definition_path_result.path(),
                        );

                        if let Ok(meta_result) = meta {
                            result.push(meta_result);
                        }
                    } else {
                        for sub_definition_path in
                            fs::read_dir(definition_path_result.path()).unwrap()
                        {
                            let sub_definition_path_result = match sub_definition_path {
                                Ok(path) => path,
                                Err(_) => continue,
                            };

                            let meta = Meta::read_from_file(
                                feature_name.clone(),
                                meta_type,
                                sub_definition_path_result.path(),
                            );

                            if let Ok(meta_result) = meta {
                                result.push(meta_result);
                            }
                        }
                    }
                }
            }
        }

        Some(Reader { meta: result })
    }
}

fn get_file_name(entry: &DirEntry) -> Option<String> {
    entry
        .file_name()
        .to_str()
        .map(|file_name| file_name.to_string())
}

use std::{
    fs::{self, DirEntry},
    io::Error,
    path::Path,
};

#[derive(Debug, Clone, Copy)]
pub enum MetaType {
    FlowType,
    DataType,
    RuntimeFunction,
}

#[derive(Debug)]
pub struct Reader {
    pub meta: Vec<Meta>,
}

#[derive(Debug)]
pub struct Meta {
    pub name: String,
    pub r#type: MetaType,
    pub data: Vec<String>,
}

impl Meta {
    pub fn read_from_file<P>(name: String, r#type: MetaType, file_path: P) -> Result<Meta, Error>
    where
        P: AsRef<Path>,
    {
        let mut inside_code = false;
        let mut current_block = vec![];
        let mut code_snippets = vec![];

        let content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(err) => {
                println!("Error reading file: {}", err);
                return Err(err);
            }
        };

        for line in content.lines() {
            if line.contains("```") {
                inside_code = !inside_code;

                if !inside_code {
                    let code_snippet = current_block.join(" ");
                    code_snippets.push(code_snippet);
                    current_block.clear();
                }
            }

            if inside_code {
                if line.starts_with("```") {
                    continue;
                }

                current_block.push(line.to_string());
            }
        }

        Ok(Meta {
            name: name,
            r#type: r#type,
            data: code_snippets,
        })
    }
}

/// Reader
///
/// Expecting the file system too look like:
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

                        match meta {
                            Ok(meta_result) => {
                                result.push(meta_result);
                            }
                            Err(err) => {
                                println!("Error reading meta: {:?}", err);
                            }
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

                            match meta {
                                Ok(meta_result) => {
                                    result.push(meta_result);
                                }
                                Err(err) => {
                                    println!("Error reading meta: {:?}", err);
                                }
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
    if let Some(file_name) = entry.file_name().to_str() {
        Some(file_name.to_string())
    } else {
        None
    }
}

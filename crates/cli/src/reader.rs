use std::{
    fs::{self, DirEntry},
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

use serde::Serialize;

#[derive(Serialize, Debug, Clone, Copy)]
pub enum MetaType {
    ModuleDefinition,
    FlowType,
    RuntimeFlowType,
    DataType,
    RuntimeFunction,
    Function,
    Configs,
}

impl std::fmt::Display for MetaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetaType::FlowType => write!(f, "FlowType"),
            MetaType::DataType => write!(f, "DataType"),
            MetaType::RuntimeFunction => write!(f, "RuntimeFunction"),
            MetaType::RuntimeFlowType => write!(f, "RuntimeFlowType"),
            MetaType::Function => write!(f, "Function"),
            MetaType::Configs => write!(f, "Configs"),
            MetaType::ModuleDefinition => write!(f, "ModuleDefinition"),
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
///   - <module>
///     - <flow_types>
///     - <data_types>
///     - <runtime_functions>
///     - module.json
///    - <module>
///     - <flow_types>
///     - <data_types>
///     - <runtime_functions>
///     - module.json
impl Reader {
    pub fn from_path(path: &str) -> Option<Reader> {
        let mut result: Vec<Meta> = vec![];
        let root = Path::new(path);

        if !root.exists() || !root.is_dir() {
            return None;
        }

        for module_path in find_module_directories(root) {
            let module_name = module_name_from_paths(root, &module_path);

            // Handle direct module definition file.
            let module_definition_file = module_path.join("module.json");
            if module_definition_file.is_file()
                && let Ok(meta_result) = Meta::read_from_file(
                    module_name.clone(),
                    MetaType::ModuleDefinition,
                    module_definition_file,
                )
            {
                result.push(meta_result);
            }

            // Handle all typed definition directories.
            let type_entries = match fs::read_dir(&module_path) {
                Ok(entries) => entries,
                Err(_) => continue,
            };

            for type_path in type_entries {
                let type_path_result = match type_path {
                    Ok(path) => path,
                    Err(_) => continue,
                };

                let file_type = match type_path_result.file_type() {
                    Ok(file_type) => file_type,
                    Err(_) => continue,
                };

                if !file_type.is_dir() {
                    continue;
                }

                let meta_type = match get_file_name(&type_path_result) {
                    Some(name) => match meta_type_from_dir_name(name.as_str()) {
                        Some(meta_type) => meta_type,
                        None => continue,
                    },
                    None => continue,
                };

                let mut definition_files = vec![];
                collect_json_files_recursively(type_path_result.path(), &mut definition_files);
                definition_files.sort();

                for definition_file in definition_files {
                    let meta =
                        Meta::read_from_file(module_name.clone(), meta_type, definition_file);

                    if let Ok(meta_result) = meta {
                        result.push(meta_result);
                    }
                }
            }
        }

        Some(Reader { meta: result })
    }
}

fn find_module_directories(root: &Path) -> Vec<PathBuf> {
    let mut module_directories = vec![];
    let mut stack = vec![root.to_path_buf()];

    while let Some(current) = stack.pop() {
        let entries = match fs::read_dir(&current) {
            Ok(entries) => entries,
            Err(_) => continue,
        };

        let mut sub_directories = vec![];
        let mut looks_like_module = false;

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let entry_type = match entry.file_type() {
                Ok(entry_type) => entry_type,
                Err(_) => continue,
            };

            if entry_type.is_file() {
                if entry.file_name().to_str() == Some("module.json") {
                    looks_like_module = true;
                }
                continue;
            }

            if !entry_type.is_dir() {
                continue;
            }

            let directory_name = match get_file_name(&entry) {
                Some(name) => name,
                None => continue,
            };

            if meta_type_from_dir_name(directory_name.as_str()).is_some() {
                looks_like_module = true;
            } else {
                sub_directories.push(entry.path());
            }
        }

        if looks_like_module {
            module_directories.push(current);
        } else {
            stack.extend(sub_directories);
        }
    }

    module_directories.sort();
    module_directories
}

fn module_name_from_paths(root: &Path, module_path: &Path) -> String {
    let relative = module_path
        .strip_prefix(root)
        .ok()
        .and_then(|p| p.to_str())
        .unwrap_or_default();

    if relative.is_empty() || relative == "." {
        module_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("module")
            .to_string()
    } else {
        relative.to_string()
    }
}

fn meta_type_from_dir_name(name: &str) -> Option<MetaType> {
    match name {
        "runtime_flow_type" | "runtime_flow_types" => Some(MetaType::RuntimeFlowType),
        "flow_type" | "flow_types" => Some(MetaType::FlowType),
        "data_type" | "data_types" => Some(MetaType::DataType),
        "runtime_functions" => Some(MetaType::RuntimeFunction),
        "functions" => Some(MetaType::Function),
        "configuration" | "configurations" => Some(MetaType::Configs),
        "module" => Some(MetaType::ModuleDefinition),
        _ => None,
    }
}

fn collect_json_files_recursively(path: PathBuf, result: &mut Vec<PathBuf>) {
    let dir_entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in dir_entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(_) => continue,
        };

        let entry_path = entry.path();
        if file_type.is_file() {
            let is_json = entry_path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("json"))
                .unwrap_or(false);

            if is_json {
                result.push(entry_path);
            }
        } else if file_type.is_dir() {
            collect_json_files_recursively(entry_path, result);
        }
    }
}

fn get_file_name(entry: &DirEntry) -> Option<String> {
    entry
        .file_name()
        .to_str()
        .map(|file_name| file_name.to_string())
}

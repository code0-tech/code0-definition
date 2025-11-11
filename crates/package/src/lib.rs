use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::path::Path;
use std::{fs, io};
use tucana::shared::{DefinitionDataType, FlowType, RuntimeFunctionDefinition};
use walkdir::WalkDir;

#[derive(Deserialize, Debug)]
pub struct Feature {
    pub name: String,
    pub data_types: Vec<DefinitionDataType>,
    pub flow_types: Vec<FlowType>,
    pub functions: Vec<RuntimeFunctionDefinition>,
}

pub fn read_features(path: &str) -> Result<Vec<Feature>, io::Error> {
    let definitions = Path::new(path);

    match read_feature_content(definitions) {
        Ok(features) => {
            log::info!("Loaded Successfully {} features", features.len());
            Ok(features)
        }
        Err(err) => {
            panic!("Cannot loaded content: {}", err)
        }
    }
}

fn read_feature_content(dir: &Path) -> Result<Vec<Feature>, io::Error> {
    let mut features: Vec<Feature> = Vec::new();
    let readdir = match fs::read_dir(dir) {
        Ok(readdir) => readdir,
        Err(err) => {
            log::error!("Failed to read directory {}: {}", dir.display(), err);
            return Err(err);
        }
    };

    for entry_result in readdir {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(err) => {
                log::error!("Failed to read directory entry: {}", err);
                continue;
            }
        };

        let path = entry.path();

        if path.is_dir() {
            let feature_name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            let data_types_path = path.join("data_type");
            let data_types: Vec<DefinitionDataType> = collect_definitions(&data_types_path)?;

            let flow_types_path = path.join("flow_type");
            let flow_types: Vec<FlowType> = collect_definitions(&flow_types_path)?;

            let functions_path = path.join("runtime_definition");
            let functions: Vec<RuntimeFunctionDefinition> = collect_definitions(&functions_path)?;

            let feature = Feature {
                name: feature_name,
                data_types,
                flow_types,
                functions,
            };

            features.push(feature);
        }
    }

    Ok(features)
}

fn collect_definitions<T>(dir: &Path) -> Result<Vec<T>, io::Error>
where
    T: DeserializeOwned,
{
    let mut definitions = Vec::new();

    if !dir.exists() {
        return Ok(definitions);
    }

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
            let content = match fs::read_to_string(path) {
                Ok(content) => content,
                Err(err) => {
                    log::error!("Failed to read file {}: {}", path.display(), err);
                    continue;
                }
            };

            match serde_json::from_str::<T>(&content) {
                Ok(def) => definitions.push(def),
                Err(e) => {
                    log::error!("Failed to parse JSON in file {}: {}", path.display(), e);
                }
            }
        }
    }

    Ok(definitions)
}

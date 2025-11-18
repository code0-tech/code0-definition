mod r#enum;
mod r#struct;

use crate::r#enum::reader_error::ReaderError;
use crate::r#struct::feature::Feature;
use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;
use tucana::shared::{DefinitionDataType, FlowType, RuntimeFunctionDefinition, Version};
use walkdir::WalkDir;

pub struct Reader {
    should_break: bool,
    accepted_features: Vec<String>,
    accepted_version: Option<Version>,
    path: String,
}

impl Reader {
    pub fn configure(
        path: String,
        should_break: bool,
        accepted_features: Vec<String>,
        accepted_version: Option<Version>,
    ) -> Self {
        Self {
            should_break,
            accepted_features,
            accepted_version,
            path,
        }
    }

    pub fn read_features(&self) -> Result<Vec<Feature>, ReaderError> {
        let definitions = Path::new(&self.path);

        match self.read_feature_content(definitions) {
            Ok(features) => {
                log::info!("Loaded Successfully {} features", features.len());
                Ok(features)
            }
            Err(err) => {
                log::error!("Failed to read features from {}", &self.path);
                Err(ReaderError::ReadFeatureError {
                    path: self.path.to_string(),
                    source: Box::new(err),
                })
            }
        }
    }

    fn read_feature_content(&self, dir: &Path) -> Result<Vec<Feature>, ReaderError> {
        let mut features: Vec<Feature> = Vec::new();
        let readdir = match fs::read_dir(dir) {
            Ok(readdir) => readdir,
            Err(err) => {
                log::error!("Failed to read directory {}: {}", dir.display(), err);
                return Err(ReaderError::ReadDirectoryError {
                    path: dir.to_path_buf(),
                    error: err,
                });
            }
        };

        for entry_result in readdir {
            let entry = match entry_result {
                Ok(entry) => entry,
                Err(err) => {
                    log::error!("Failed to read directory entry: {}", err);
                    return Err(ReaderError::DirectoryEntryError(err));
                }
            };

            let path = entry.path();

            if path.is_dir() {
                let feature_name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                if !self.accepted_features.is_empty()
                    && !self.accepted_features.contains(&feature_name)
                {
                    log::info!("Skipping feature: {}", feature_name);
                    continue;
                }

                let data_types_path = path.join("data_type");
                let data_types: Vec<DefinitionDataType> =
                    self.collect_definitions(&data_types_path)?;

                let flow_types_path = path.join("flow_type");
                let flow_types: Vec<FlowType> = self.collect_definitions(&flow_types_path)?;

                let functions_path = path.join("runtime_definition");
                let functions =
                    match self.collect_definitions::<RuntimeFunctionDefinition>(&functions_path) {
                        Ok(func) => func
                            .into_iter()
                            .filter(|v| v.version == self.accepted_version)
                            .collect(),
                        Err(err) => {
                            if self.should_break {
                                return Err(ReaderError::ReadFeatureError {
                                    path: functions_path.to_string_lossy().to_string(),
                                    source: Box::new(err),
                                });
                            } else {
                                continue;
                            }
                        }
                    };

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

    fn collect_definitions<T>(&self, dir: &Path) -> Result<Vec<T>, ReaderError>
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
                        return Err(ReaderError::ReadFileError {
                            path: path.to_path_buf(),
                            error: err,
                        });
                    }
                };

                match serde_json::from_str::<T>(&content) {
                    Ok(def) => definitions.push(def),
                    Err(e) => {
                        if self.should_break {
                            log::error!("Failed to parse JSON in file {}: {}", path.display(), e);
                            return Err(ReaderError::JsonError {
                                path: path.to_path_buf(),
                                error: e,
                            });
                        } else {
                            log::warn!("Skipping invalid JSON file {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }

        Ok(definitions)
    }
}

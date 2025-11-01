use crate::parser::Parser;
use prost::Message;
use std::fs;
use std::io::Write;

pub fn bundle(path: Option<String>, out: Option<String>) {
    let dir_path = path.unwrap_or_else(|| "./definitions".to_string());
    let out_path = out.unwrap_or_else(|| "./bundles".to_string());
    match fs::create_dir_all(&out_path) {
        Ok(_) => {}
        Err(err) => {
            panic!("Error creating output directory: {:?}", err);
        }
    }

    let parser = match Parser::from_path(dir_path.as_str()) {
        Some(reader) => reader,
        None => {
            panic!("Error reading definitions");
        }
    };

    for feature in parser.features {
        feature.data_types.iter().for_each(|data_type| {
            let mut buf = Vec::new();
            if data_type.encode(&mut buf).is_ok() {
                let path = format!(
                    "{}/{}_{}_{}.pb",
                    &out_path,
                    feature.name,
                    "data_type",
                    data_type.identifier.to_lowercase()
                );
                fs::File::create(&path)
                    .expect("abc")
                    .write_all(&buf)
                    .expect("a");
            }
        });

        feature.flow_types.iter().for_each(|flow_type| {
            let mut buf = Vec::new();
            if flow_type.encode(&mut buf).is_ok() {
                let path = format!(
                    "{}/{}_{}_{}.pb",
                    &out_path,
                    feature.name,
                    "flow_type",
                    flow_type.identifier.to_lowercase()
                );
                fs::File::create(&path)
                    .expect("abc")
                    .write_all(&buf)
                    .expect("a");
            }
        });

        feature.runtime_functions.iter().for_each(|function| {
            let mut buf = Vec::new();
            if function.encode(&mut buf).is_ok() {
                let path = format!(
                    "{}/{}_{}_{}.pb",
                    &out_path,
                    feature.name,
                    "function",
                    function.runtime_name.to_lowercase()
                );
                fs::File::create(&path)
                    .expect("abc")
                    .write_all(&buf)
                    .expect("a");
            }
        });
    }
}

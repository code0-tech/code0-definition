use crate::table::{feature_table, summary_table};
use code0_definition_reader::parser::Parser;

pub fn search_feature(name: Option<String>, path: Option<String>) {
    let dir_path = path.unwrap_or_else(|| "./definitions".to_string());

    let parser = match Parser::from_path(dir_path.as_str()) {
        Some(reader) => reader,
        None => {
            panic!("Error reading definitions");
        }
    };

    if let Some(feature_name) = name {
        let mut features_to_report = Vec::new();
        for feature in &parser.features {
            if feature.name == feature_name {
                feature_table(feature);
                features_to_report.push(feature.clone());
            }
        }
        summary_table(&features_to_report);
    } else {
        for feature in &parser.features {
            feature_table(feature);
        }
        summary_table(&parser.features);
    }
}

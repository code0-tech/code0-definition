use crate::analyser::core::Analyser;
use crate::formatter::{success, success_table};
use crate::parser::{Feature, Parser};
use crate::table::{feature_table, summary_table};

pub fn search_feature(name: Option<String>, path: Option<String>) {
    let dir_path = path.unwrap_or_else(|| "./definitions".to_string());

    let parser = match Parser::from_path(dir_path.as_str()) {
        Some(reader) => reader,
        None => {
            panic!("Error reading definitions");
        }
    };

    let mut analyser = Analyser::new(dir_path.as_str());
    analyser.report(true);

    let features = match name {
        None => parser.features.clone(),
        Some(feature_name) => parser
            .features
            .iter()
            .filter(|f| f.name.to_lowercase() == feature_name.to_lowercase())
            .cloned()
            .collect::<Vec<Feature>>(),
    };

    for feature in &features {
        let (flow_type_rows, data_type_rows, function_rows) = feature_table(feature);

        if !flow_type_rows.is_empty() {
            success(format!(
                "The feature (`{}`) detected {} flow_types.",
                feature.name,
                flow_type_rows.len()
            ));
            success_table(flow_type_rows)
        }

        if !data_type_rows.is_empty() {
            success(format!(
                "The feature (`{}`) detected {} data_types.",
                feature.name,
                data_type_rows.len()
            ));
            success_table(data_type_rows)
        }

        if !function_rows.is_empty() {
            success(format!(
                "The feature (`{}`) detected {} runtime_function_definition.",
                feature.name,
                function_rows.len()
            ));
            success_table(function_rows)
        }
    }

    let summary = summary_table(&features);
    success_table(summary);

    success(format!(
        "Defined a total of {} Features with {} FlowTypes {} DataTypes and {} Functions!",
        parser.features.iter().len(),
        parser
            .features
            .iter()
            .map(|f| f.flow_types.len())
            .sum::<usize>(),
        parser
            .features
            .iter()
            .map(|f| f.data_types.len())
            .sum::<usize>(),
        parser
            .features
            .iter()
            .map(|f| f.runtime_functions.len())
            .sum::<usize>()
    ))
}

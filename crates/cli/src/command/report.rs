use crate::analyser::core::Analyser;
use crate::formatter::{success, success_table};
use crate::parser::Parser;
use crate::table::summary_table;

pub fn report_errors(path: Option<String>) {
    let dir_path = path.unwrap_or_else(|| "./definitions".to_string());

    let parser = match Parser::from_path(dir_path.as_str()) {
        Some(reader) => reader,
        None => {
            panic!("Error reading definitions");
        }
    };

    let mut analyser = Analyser::new(dir_path.as_str());
    analyser.report(true);

    let rows = summary_table(&parser.features);
    success_table(rows);

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

use crate::table::{error_table, summary_table};
use code0_definition_reader::parser::Parser;

pub fn report_errors(path: Option<String>) {
    let dir_path = path.unwrap_or_else(|| "./definitions".to_string());

    let parser = match Parser::from_path(dir_path.as_str()) {
        Some(reader) => reader,
        None => {
            panic!("Error reading definitions");
        }
    };
    error_table(&parser.features);
    summary_table(&parser.features);
}

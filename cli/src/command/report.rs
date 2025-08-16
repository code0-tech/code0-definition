use crate::table::{error_table, summary_table};
use code0_definition_reader::parser::Parser;
use crate::analyser::{Analyser};

pub fn report_errors(path: Option<String>) {
    let dir_path = path.unwrap_or_else(|| "./definitions".to_string());

    let parser = match Parser::from_path(dir_path.as_str()) {
        Some(reader) => reader,
        None => {
            panic!("Error reading definitions");
        }
    };

   let mut analyser = Analyser::new(dir_path.as_str());

    for data_type in analyser.data_types.clone() {
       analyser.analyse_data_type(data_type.clone());
    }

    for flow_type in analyser.flow_types.clone() {
        analyser.analyse_flow_type(flow_type.clone());
    }

    analyser.report();

    error_table(&parser.features);
    summary_table(&parser.features);
}

use crate::table::{error_table, summary_table};
use code0_definition_reader::parser::Parser;
use crate::analyser::{AnalysableDataType, AnalysableFlowType, AnalysableFunction, Analyser};

pub fn report_errors(path: Option<String>) {
    let dir_path = path.unwrap_or_else(|| "./definitions".to_string());

    let parser = match Parser::from_path(dir_path.as_str()) {
        Some(reader) => reader,
        None => {
            panic!("Error reading definitions");
        }
    };

    let mut index = 0;
    let collected_data_types = parser
        .features
        .iter()
        .map(|f| f.data_types.clone())
        .flatten()
        .map(|d| {
            index = index + 1;
            return AnalysableDataType {
                definition_data_type: d.clone(),
                id: index
            }
         })
        .collect::<Vec<_>>();

    let collected_functions = parser
        .features
        .iter()
        .map(|f| f.runtime_functions.clone())
        .flatten()
        .map(|d| {
            index = index + 1;
            return AnalysableFunction {
                function: d.clone(),
                id: index
            }
        })
        .collect::<Vec<_>>();

    let collected_flow_types = parser
        .features
        .iter()
        .map(|f| f.flow_types.clone())
        .flatten()
        .map(|d| {
            index = index + 1;
            return AnalysableFlowType {
                flow_type: d.clone(),
                id: index
            }
        })
        .collect::<Vec<_>>();

    let analyser = Analyser {
        data_types: collected_data_types.clone(),
        functions: collected_functions,
        flow_types: collected_flow_types,
    };

    for data_type in collected_data_types {
        analyser.analyse_data_type(data_type);
    }

    error_table(&parser.features);
    summary_table(&parser.features);
}

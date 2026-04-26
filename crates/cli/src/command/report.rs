use tucana::shared::Module;

use crate::analyser::core::Analyser;
use crate::command::parse_errors::fail_on_parser_errors;
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
    fail_on_parser_errors(&parser);

    let mut analyser = Analyser::new(dir_path.as_str());
    analyser.report(true, true);
    let mods: &Vec<Module> = &parser
        .modules
        .clone()
        .into_iter()
        .map(|x| x.into_module())
        .collect();

    let rows = summary_table(mods);
    success_table(rows);

    success(format!(
        "Defined a total of {} Modules with {} RuntimeFlowTypes, {} FlowTypes, {} DataTypes, {} RuntimeFunctions, {} Functions and {} Module Configs!",
        mods.iter().len(),
        mods.iter()
            .map(|f| f.runtime_flow_types.len())
            .sum::<usize>(),
        mods.iter().map(|f| f.flow_types.len()).sum::<usize>(),
        mods.iter()
            .map(|f| f.definition_data_types.len())
            .sum::<usize>(),
        mods.iter()
            .map(|f| f.runtime_function_definitions.len())
            .sum::<usize>(),
        mods.iter()
            .map(|f| f.function_definitions.len())
            .sum::<usize>(),
        mods.iter().map(|f| f.configurations.len()).sum::<usize>(),
    ))
}

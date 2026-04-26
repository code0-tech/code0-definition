use tucana::shared::Module;

use crate::analyser::core::Analyser;
use crate::command::parse_errors::fail_on_parser_errors;
use crate::formatter::{success, success_table};
use crate::parser::{DefinitionModule, Parser};
use crate::table::{module_table, summary_table};

pub fn search_module(name: Option<String>, path: Option<String>) {
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

    let modules = match name {
        None => parser.modules.clone(),
        Some(feature_name) => parser
            .modules
            .iter()
            .filter(|m| m.config.identifier.to_lowercase() == feature_name.to_lowercase())
            .cloned()
            .collect::<Vec<DefinitionModule>>(),
    };

    for module in &modules {
        let (
            runtime_flow_type_rows,
            flow_type_rows,
            data_type_rows,
            runtime_function_rows,
            function_rows,
            configuration_rows,
        ) = module_table(&module.clone().into_module());

        if !runtime_flow_type_rows.is_empty() {
            success(format!(
                "The module (`{}`) detected {} runtime_flow_types.",
                module.config.identifier,
                runtime_flow_type_rows.len()
            ));
            success_table(runtime_flow_type_rows)
        }

        if !flow_type_rows.is_empty() {
            success(format!(
                "The module (`{}`) detected {} flow_types.",
                module.config.identifier,
                flow_type_rows.len()
            ));
            success_table(flow_type_rows)
        }

        if !data_type_rows.is_empty() {
            success(format!(
                "The module (`{}`) detected {} data_types.",
                module.config.identifier,
                data_type_rows.len()
            ));
            success_table(data_type_rows)
        }

        if !runtime_function_rows.is_empty() {
            success(format!(
                "The module (`{}`) detected {} runtime_function_definition.",
                module.config.identifier,
                runtime_function_rows.len()
            ));
            success_table(runtime_function_rows)
        }

        if !function_rows.is_empty() {
            success(format!(
                "The module (`{}`) detected {} function_definition.",
                module.config.identifier,
                function_rows.len()
            ));
            success_table(function_rows)
        }

        if !configuration_rows.is_empty() {
            success(format!(
                "The module (`{}`) detected {} module_configurations.",
                module.config.identifier,
                configuration_rows.len()
            ));
            success_table(configuration_rows)
        }
    }

    let mods: &Vec<Module> = &parser
        .modules
        .clone()
        .into_iter()
        .map(|x| x.into_module())
        .collect();

    let summary = summary_table(&mods);
    success_table(summary);

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

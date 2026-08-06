use tucana::shared::Module;

use crate::{
    analyser::core::Analyser, command::parse_errors::fail_on_parser_errors,
    command::push::module_service_client_impl::SagittariusModuleServiceClient, parser::Parser,
};

mod auth;
mod module_service_client_impl;

fn apply_version_to_module(mut module: Module, version: String) -> Module {
    module.version = version.clone();

    for data_type in &mut module.definition_data_types {
        data_type.version = version.clone();
    }
    for flow_type in &mut module.flow_types {
        flow_type.version = version.clone();
    }
    for runtime_flow_type in &mut module.runtime_flow_types {
        runtime_flow_type.version = version.clone();
    }
    for function in &mut module.function_definitions {
        function.version = version.clone();
    }
    for runtime_function in &mut module.runtime_function_definitions {
        runtime_function.version = version.clone();
    }

    module
}

pub async fn push(
    token: String,
    url: String,
    version_option: Option<String>,
    path: Option<String>,
) {
    let dir_path = path.unwrap_or_else(|| "./definitions".to_string());

    let version = match version_option {
        None => String::from("0.0.0"),
        Some(v) => v,
    };

    let mut analyzer = Analyser::new(dir_path.as_str());
    analyzer.report(false, true);

    let parser = match Parser::from_path(dir_path.as_str()) {
        Some(parser) => parser,
        None => {
            panic!("Error reading definitions");
        }
    };
    fail_on_parser_errors(&parser);

    let mods = parser
        .modules
        .into_iter()
        .map(|definition_module| {
            let module = definition_module.into_module();
            apply_version_to_module(module, version.clone())
        })
        .collect::<Vec<_>>();

    let mut client = SagittariusModuleServiceClient::new(url, token).await;
    client.update(mods).await;
}

use tucana::shared::{
    FlowType, FlowTypeSetting, FunctionDefinition, Module, ParameterDefinition, RuntimeFlowType, RuntimeFlowTypeSetting, RuntimeFunctionDefinition, RuntimeParameterDefinition
};

use crate::{analyser::core::Analyser, command::parse_errors::fail_on_parser_errors, parser::{DefinitionModule, Parser}};

fn runtime_function_into_function(
    runtime_function: &RuntimeFunctionDefinition,
) -> FunctionDefinition {
    let parameter: Vec<ParameterDefinition> = runtime_function
        .runtime_parameter_definitions
        .iter()
        .map(|x| runtime_parameter_into_parameter(x))
        .collect();
    FunctionDefinition {
        runtime_name: runtime_function.runtime_name.clone(),
        parameter_definitions: parameter,
        signature: runtime_function.signature.clone(),
        throws_error: runtime_function.throws_error,
        name: runtime_function.name.clone(),
        description: runtime_function.description.clone(),
        documentation: runtime_function.documentation.clone(),
        deprecation_message: runtime_function.deprecation_message.clone(),
        display_message: runtime_function.display_message.clone(),
        alias: runtime_function.alias.clone(),
        linked_data_type_identifiers: runtime_function.linked_data_type_identifiers.clone(),
        version: runtime_function.version.clone(),
        display_icon: runtime_function.display_icon.clone(),
        definition_source: runtime_function.definition_source.clone(),
        runtime_definition_name: runtime_function.runtime_name.clone(),
        design: runtime_function.design.clone(),
    }
}

fn runtime_parameter_into_parameter(
    runtime_parameter: &RuntimeParameterDefinition,
) -> ParameterDefinition {
    ParameterDefinition {
        runtime_name: runtime_parameter.runtime_name.clone(),
        default_value: runtime_parameter.default_value.clone(),
        optional: runtime_parameter.optional.clone(),
        hidden: runtime_parameter.hidden.clone(),
        name: runtime_parameter.name.clone(),
        description: runtime_parameter.description.clone(),
        documentation: runtime_parameter.documentation.clone(),
        runtime_definition_name: runtime_parameter.runtime_name.clone(),
    }
}

fn runtime_flow_type_into_flow_type(runtime_flow_type: &RuntimeFlowType) -> FlowType {
    let settings: Vec<FlowTypeSetting> = runtime_flow_type
        .runtime_settings
        .iter()
        .map(|x| runtime_flow_setting_into_flow_setting(x))
        .collect();
    FlowType {
        identifier: runtime_flow_type.identifier.clone(),
        settings: settings,
        editable: runtime_flow_type.editable.clone(),
        name: runtime_flow_type.name.clone(),
        description: runtime_flow_type.description.clone(),
        documentation: runtime_flow_type.documentation.clone(),
        display_message: runtime_flow_type.display_message.clone(),
        alias: runtime_flow_type.alias.clone(),
        version: runtime_flow_type.version.clone(),
        display_icon: runtime_flow_type.display_icon.clone(),
        definition_source: runtime_flow_type.definition_source.clone(),
        linked_data_type_identifiers: runtime_flow_type.linked_data_type_identifiers.clone(),
        signature: runtime_flow_type.signature.clone(),
        runtime_identifier: runtime_flow_type.identifier.clone(),
    }
}

fn runtime_flow_setting_into_flow_setting(
    runtime_flow_setting: &RuntimeFlowTypeSetting,
) -> FlowTypeSetting {
    FlowTypeSetting {
        identifier: runtime_flow_setting.identifier.clone(),
        unique: runtime_flow_setting.unique.clone(),
        default_value: runtime_flow_setting.default_value.clone(),
        name: runtime_flow_setting.name.clone(),
        description: runtime_flow_setting.description.clone(),
        optional: runtime_flow_setting.optional.clone(),
        hidden: runtime_flow_setting.hidden.clone(),
    }
}

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

fn configure_module(definition_module: &DefinitionModule, version: String) -> Module {
    let mut module = definition_module.clone().into_module();

    module.function_definitions = module.runtime_function_definitions.iter().map(|x| runtime_function_into_function(x)).collect();
    module.flow_types = module.runtime_flow_types.iter().map(|x| runtime_flow_type_into_flow_type(x)).collect();

    apply_version_to_module(module, version)
}

pub async fn publish(version: String, path: Option<String>) {
    let dir_path = path.unwrap_or_else(|| "./definitions".to_string());

    let mut analyzer = Analyser::new(dir_path.as_str());
    analyzer.report(false, true);

    let parser = match Parser::from_path(dir_path.as_str()) {
        Some(parser) => parser,
        None => {
            panic!("Error reading definitions");
        }
    };
    fail_on_parser_errors(&parser);

    let modules: Vec<Module> = parser.modules.into_iter().map(|x| configure_module(&x, version.clone())).collect();

    //TODO: Gen out files
}

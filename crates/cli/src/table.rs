use tabled::Tabled;
use tucana::shared::{
    DefinitionDataType, FlowType, FunctionDefinition, Module, ModuleConfigurationDefinition,
    RuntimeFlowType, RuntimeFunctionDefinition,
};

#[derive(Tabled)]
pub struct RuntimeFlowTypeRow {
    #[tabled(rename = "#")]
    index: usize,
    #[tabled(rename = "Identifier")]
    identifier: String,
}

#[derive(Tabled)]
pub struct FlowTypeRow {
    #[tabled(rename = "#")]
    index: usize,
    #[tabled(rename = "Identifier")]
    identifier: String,
}

#[derive(Tabled)]
pub struct DataTypeRow {
    #[tabled(rename = "#")]
    index: usize,
    #[tabled(rename = "Identifier")]
    identifier: String,
}

#[derive(Tabled)]
pub struct RuntimeFunctionRow {
    #[tabled(rename = "#")]
    index: usize,
    #[tabled(rename = "Runtime Name")]
    runtime_name: String,
}

#[derive(Tabled)]
pub struct FunctionRow {
    #[tabled(rename = "#")]
    index: usize,
    #[tabled(rename = "Runtime Name")]
    runtime_name: String,
}

#[derive(Tabled)]
pub struct ConfigurationRow {
    #[tabled(rename = "#")]
    index: usize,
    #[tabled(rename = "Runtime Name")]
    identifier: String,
}

#[derive(Tabled)]
pub struct ModuleSummaryRow {
    #[tabled(rename = "Module")]
    identifier: String,
    #[tabled(rename = "Runtime Flow Types")]
    runtime_flow_types: usize,
    #[tabled(rename = "Flow Types")]
    flow_types: usize,
    #[tabled(rename = "Data Types")]
    data_types: usize,
    #[tabled(rename = "Runtime Functions")]
    runtime_functions: usize,
    #[tabled(rename = "Functions")]
    functions: usize,
    #[tabled(rename = "Module Configurations")]
    configurations: usize,
}

pub fn module_table(
    module: &Module,
) -> (
    Vec<RuntimeFlowTypeRow>,
    Vec<FlowTypeRow>,
    Vec<DataTypeRow>,
    Vec<RuntimeFunctionRow>,
    Vec<FunctionRow>,
    Vec<ConfigurationRow>,
) {
    let runtime_flow_type_rows: Vec<RuntimeFlowTypeRow> = module
        .runtime_flow_types
        .iter()
        .enumerate()
        .map(
            |(i, RuntimeFlowType { identifier, .. })| RuntimeFlowTypeRow {
                index: i + 1,
                identifier: identifier.clone(),
            },
        )
        .collect();

    let flow_type_rows: Vec<FlowTypeRow> = module
        .flow_types
        .iter()
        .enumerate()
        .map(|(i, FlowType { identifier, .. })| FlowTypeRow {
            index: i + 1,
            identifier: identifier.clone(),
        })
        .collect();

    let data_type_rows: Vec<DataTypeRow> = module
        .definition_data_types
        .iter()
        .enumerate()
        .map(|(i, DefinitionDataType { identifier, .. })| DataTypeRow {
            index: i + 1,
            identifier: identifier.clone(),
        })
        .collect();

    let runtime_function_rows: Vec<RuntimeFunctionRow> = module
        .runtime_function_definitions
        .iter()
        .enumerate()
        .map(
            |(i, RuntimeFunctionDefinition { runtime_name, .. })| RuntimeFunctionRow {
                index: i + 1,
                runtime_name: runtime_name.clone(),
            },
        )
        .collect();

    let function_rows: Vec<FunctionRow> = module
        .function_definitions
        .iter()
        .enumerate()
        .map(|(i, FunctionDefinition { runtime_name, .. })| FunctionRow {
            index: i + 1,
            runtime_name: runtime_name.clone(),
        })
        .collect();

    let configuration_rows: Vec<ConfigurationRow> = module
        .configurations
        .iter()
        .enumerate()
        .map(
            |(i, ModuleConfigurationDefinition { identifier, .. })| ConfigurationRow {
                index: i + 1,
                identifier: identifier.clone(),
            },
        )
        .collect();

    (
        runtime_flow_type_rows,
        flow_type_rows,
        data_type_rows,
        runtime_function_rows,
        function_rows,
        configuration_rows,
    )
}

pub fn summary_table(modules: &[Module]) -> Vec<ModuleSummaryRow> {
    modules
        .iter()
        .map(|module| ModuleSummaryRow {
            identifier: module.identifier.clone(),
            runtime_flow_types: module.runtime_flow_types.len(),
            flow_types: module.flow_types.len(),
            data_types: module.definition_data_types.len(),
            runtime_functions: module.runtime_function_definitions.len(),
            functions: module.function_definitions.len(),
            configurations: module.configurations.len(),
        })
        .collect()
}

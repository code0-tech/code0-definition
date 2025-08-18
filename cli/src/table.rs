use code0_definition_reader::parser::Feature;
use tabled::{Tabled};
use tucana::shared::{DefinitionDataType, FlowType, RuntimeFunctionDefinition};

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
pub struct ErrorRow {
    #[tabled(rename = "#")]
    index: usize,
    #[tabled(rename = "Type")]
    definition_type: String,
    #[tabled(rename = "Definition")]
    definition: String,
    #[tabled(rename = "Error")]
    error: String,
}

#[derive(Tabled)]
pub struct FeatureSummaryRow {
    #[tabled(rename = "Feature")]
    feature_name: String,
    #[tabled(rename = "Flow Types")]
    flow_types: usize,
    #[tabled(rename = "Data Types")]
    data_types: usize,
    #[tabled(rename = "Runtime Functions")]
    runtime_functions: usize,
}

#[derive(Tabled)]
pub struct GeneralErrorRow {
    #[tabled(rename = "#")]
    index: usize,
    #[tabled(rename = "Feature")]
    feature_name: String,
    #[tabled(rename = "Type")]
    definition_type: String,
    #[tabled(rename = "Definition")]
    definition: String,
    #[tabled(rename = "Error")]
    error: String,
}

pub fn feature_table(
    feature: &Feature,
) -> (Vec<FlowTypeRow>, Vec<DataTypeRow>, Vec<RuntimeFunctionRow>) {
    let flow_type_rows: Vec<FlowTypeRow> = feature
        .flow_types
        .iter()
        .enumerate()
        .map(|(i, FlowType { identifier, .. })| FlowTypeRow {
            index: i + 1,
            identifier: identifier.clone(),
        })
        .collect();

    let data_type_rows: Vec<DataTypeRow> = feature
        .data_types
        .iter()
        .enumerate()
        .map(|(i, DefinitionDataType { identifier, .. })| DataTypeRow {
            index: i + 1,
            identifier: identifier.clone(),
        })
        .collect();

    let runtime_function_rows: Vec<RuntimeFunctionRow> = feature
        .runtime_functions
        .iter()
        .enumerate()
        .map(
            |(i, RuntimeFunctionDefinition { runtime_name, .. })| RuntimeFunctionRow {
                index: i + 1,
                runtime_name: runtime_name.clone(),
            },
        )
        .collect();

    (flow_type_rows, data_type_rows, runtime_function_rows)
}

pub fn summary_table(features: &Vec<Feature>) -> Vec<FeatureSummaryRow> {
    features
        .iter()
        .map(|feature| FeatureSummaryRow {
            feature_name: feature.name.clone(),
            flow_types: feature.flow_types.len(),
            data_types: feature.data_types.len(),
            runtime_functions: feature.runtime_functions.len(),
        })
        .collect()
}

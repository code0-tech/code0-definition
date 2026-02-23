use crate::analyser::core::Analyser;
use crate::command::push::data_type_client_impl::SagittariusDataTypeServiceClient;
use crate::command::push::flow_type_client_impl::SagittariusFlowTypeServiceClient;
use crate::command::push::function_client_impl::SagittariusRuntimeFunctionServiceClient;

mod auth;
mod data_type_client_impl;
mod flow_type_client_impl;
mod function_client_impl;

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
    let mut data_type_client =
        SagittariusDataTypeServiceClient::new(url.clone(), token.clone()).await;
    let mut flow_type_client =
        SagittariusFlowTypeServiceClient::new(url.clone(), token.clone()).await;
    let mut function_client = SagittariusRuntimeFunctionServiceClient::new(url, token).await;

    analyzer.report(false, true);

    data_type_client
        .update_data_types(
            analyzer
                .data_types
                .iter()
                .map(|d| {
                    let mut def = d.definition_data_type.clone();
                    def.version = version.clone();
                    return def;
                })
                .collect(),
        )
        .await;
    flow_type_client
        .update_flow_types(
            analyzer
                .flow_types
                .iter()
                .map(|d| {
                    let mut def = d.flow_type.clone();
                    def.version = version.clone();
                    return def;
                })
                .collect(),
        )
        .await;
    function_client
        .update_runtime_function_definitions(
            analyzer
                .functions
                .iter()
                .map(|d| {
                    let mut def = d.function.clone();
                    def.version = version.clone();
                    return def;
                })
                .collect(),
        )
        .await;
}

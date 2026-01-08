use crate::analyser::core::Analyser;
use crate::command::push::data_type_client_impl::SagittariusDataTypeServiceClient;
use crate::command::push::flow_type_client_impl::SagittariusFlowTypeServiceClient;
use crate::command::push::function_client_impl::SagittariusRuntimeFunctionServiceClient;
use crate::formatter::{default, info};
use notify::event::ModifyKind;
use notify::{EventKind, RecursiveMode, Watcher, recommended_watcher};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

mod auth;
mod data_type_client_impl;
mod flow_type_client_impl;
mod function_client_impl;

pub async fn push(token: String, url: String, path: Option<String>) {
    let dir_path = path.unwrap_or_else(|| "./definitions".to_string());

    info(format!("Watching directory: {dir_path}"));
    info(String::from("Press Ctrl+C to stop watching..."));

    {
        Analyser::new(dir_path.as_str()).report(false, true);
    }

    // Set up file watcher
    let (tx, rx) = channel();
    let mut watcher = recommended_watcher(tx).unwrap();
    watcher
        .watch(std::path::Path::new(&dir_path), RecursiveMode::Recursive)
        .unwrap();

    let mut last_run = Instant::now();

    let mut data_type_client =
        SagittariusDataTypeServiceClient::new(url.clone(), token.clone()).await;
    let mut flow_type_client =
        SagittariusFlowTypeServiceClient::new(url.clone(), token.clone()).await;
    let mut function_client = SagittariusRuntimeFunctionServiceClient::new(url, token).await;

    loop {
        if let Ok(Ok(event)) = rx.recv() {
            match event.kind {
                EventKind::Modify(modify) => {
                    if let ModifyKind::Data(_) = modify
                        && last_run.elapsed() > Duration::from_millis(500)
                    {
                        default(String::from(
                            "\n\n\n--------------------------------------------------------------------------\n\n",
                        ));
                        info(String::from("Change detected! Regenerating report..."));
                        let mut analyzer = Analyser::new(dir_path.as_str());

                        // No errors when reporter is empty!
                        if analyzer.reporter.is_empty() {
                            data_type_client
                                .update_data_types(
                                    analyzer
                                        .data_types
                                        .iter()
                                        .map(|d| d.definition_data_type.clone())
                                        .collect(),
                                )
                                .await;
                            flow_type_client
                                .update_flow_types(
                                    analyzer
                                        .flow_types
                                        .iter()
                                        .map(|d| d.flow_type.clone())
                                        .collect(),
                                )
                                .await;
                            function_client
                                .update_runtime_function_definitions(
                                    analyzer
                                        .functions
                                        .iter()
                                        .map(|d| d.function.clone())
                                        .collect(),
                                )
                                .await;
                        }

                        analyzer.report(false, true);

                        last_run = Instant::now();
                    }
                }
                EventKind::Remove(_) => {
                    if last_run.elapsed() > Duration::from_millis(500) {
                        default(String::from(
                            "\n\n\n--------------------------------------------------------------------------\n\n",
                        ));
                        info(String::from("Change detected! Regenerating report..."));
                        let mut analyzer = Analyser::new(dir_path.as_str());

                        // No errors when reporter is empty!
                        if analyzer.reporter.is_empty() {
                            data_type_client
                                .update_data_types(
                                    analyzer
                                        .data_types
                                        .iter()
                                        .map(|d| d.definition_data_type.clone())
                                        .collect(),
                                )
                                .await;
                            flow_type_client
                                .update_flow_types(
                                    analyzer
                                        .flow_types
                                        .iter()
                                        .map(|d| d.flow_type.clone())
                                        .collect(),
                                )
                                .await;
                            function_client
                                .update_runtime_function_definitions(
                                    analyzer
                                        .functions
                                        .iter()
                                        .map(|d| d.function.clone())
                                        .collect(),
                                )
                                .await;
                        }

                        analyzer.report(false, true);
                        last_run = Instant::now();
                    }
                }
                _ => {}
            }
        }
    }
}

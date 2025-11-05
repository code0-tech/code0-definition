use crate::command::push::auth::get_authorization_metadata;
use tonic::Extensions;
use tonic::Request;
use tonic::transport::Channel;
use tucana::sagittarius::RuntimeFunctionDefinitionUpdateRequest as SagittariusRuntimeFunctionUpdateRequest;
use tucana::sagittarius::runtime_function_definition_service_client::RuntimeFunctionDefinitionServiceClient;
use tucana::shared::RuntimeFunctionDefinition;

pub struct SagittariusRuntimeFunctionServiceClient {
    client: RuntimeFunctionDefinitionServiceClient<Channel>,
    token: String,
}

impl SagittariusRuntimeFunctionServiceClient {
    pub async fn new(sagittarius_url: String, token: String) -> Self {
        let client = match RuntimeFunctionDefinitionServiceClient::connect(sagittarius_url).await {
            Ok(client) => {
                log::info!("Successfully connected to Sagittarius RuntimeFunction Endpoint!");
                client
            }
            Err(err) => panic!(
                "Failed to connect to Sagittarius (RuntimeFunction Endpoint): {:?}",
                err
            ),
        };

        Self { client, token }
    }

    pub async fn update_runtime_function_definitions(
        &mut self,
        runtime_functions: Vec<RuntimeFunctionDefinition>,
    ) {
        let request = Request::from_parts(
            get_authorization_metadata(&self.token),
            Extensions::new(),
            SagittariusRuntimeFunctionUpdateRequest { runtime_functions },
        );

        match self.client.update(request).await {
            Ok(response) => {
                log::info!(
                    "Successfully transferred RuntimeFunctions. Did Sagittarius updated them? {:?}",
                    &response
                );
            }
            Err(err) => {
                log::error!("Failed to update RuntimeFunctions: {:?}", err);
            }
        };
    }
}

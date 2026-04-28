use crate::command::push::auth::get_authorization_metadata;
use crate::formatter::{error_without_trace, info};
use tonic::{Extensions, Request, transport::Channel};
use tucana::sagittarius::ModuleUpdateRequest;
use tucana::sagittarius::module_service_client::ModuleServiceClient;
use tucana::shared::Module;

pub struct SagittariusModuleServiceClient {
    client: ModuleServiceClient<Channel>,
    token: String,
}

impl SagittariusModuleServiceClient {
    pub async fn new(sagittarius_url: String, token: String) -> Self {
        let client = match ModuleServiceClient::connect(sagittarius_url).await {
            Ok(client) => {
                info(String::from(
                    "Successfully connected to Sagittarius Module Service Endpoint!",
                ));
                client
            }
            Err(err) => panic!(
                "Failed to connect to Sagittarius (Module Service Endpoint): {:?}",
                err
            ),
        };
        Self { client, token }
    }

    pub async fn update(&mut self, modules: Vec<Module>) {
        let request = Request::from_parts(
            get_authorization_metadata(&self.token),
            Extensions::new(),
            ModuleUpdateRequest { modules },
        );

        match self.client.update(request).await {
            Ok(response) => {
                info(format!(
                    "Successfully transferred data types. Did Sagittarius updated them? {:?}",
                    &response.into_inner().success
                ));
            }
            Err(err) => {
                error_without_trace(format!("Failed to update DataTypes: {:?}", err));
            }
        };
    }
}

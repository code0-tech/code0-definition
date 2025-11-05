use tonic::Extensions;
use tonic::Request;
use tonic::transport::Channel;
use tucana::sagittarius::FlowTypeUpdateRequest as SagittariusFlowTypeUpdateRequest;
use tucana::sagittarius::flow_type_service_client::FlowTypeServiceClient;
use tucana::shared::FlowType;
use crate::command::push::auth::get_authorization_metadata;

pub struct SagittariusFlowTypeServiceClient {
    client: FlowTypeServiceClient<Channel>,
    token: String,
}

impl SagittariusFlowTypeServiceClient {
    pub async fn new(sagittarius_url: String, token: String) -> Self {
        let client = match FlowTypeServiceClient::connect(sagittarius_url).await {
            Ok(client) => {
                log::info!("Successfully connected to Sagittarius FlowType Endpoint!");
                client
            }
            Err(err) => panic!(
                "Failed to connect to Sagittarius (FlowType Endpoint): {:?}",
                err
            ),
        };

        Self { client, token }
    }

    pub async fn update_flow_types(
        &mut self,
        flow_types: Vec<FlowType>,
    ) {
        let request = Request::from_parts(
            get_authorization_metadata(&self.token),
            Extensions::new(),
            SagittariusFlowTypeUpdateRequest {
                flow_types,
            },
        );

       match self.client.update(request).await {
            Ok(response) => {
                log::info!(
                    "Successfully transferred FlowTypes. Did Sagittarius updated them? {:?}",
                    &response
                );
            }
            Err(err) => {
                log::error!("Failed to update FlowTypes: {:?}", err);
            }
        };
    }
}
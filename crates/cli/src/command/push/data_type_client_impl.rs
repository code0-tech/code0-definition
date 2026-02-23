use crate::command::push::auth::get_authorization_metadata;
use crate::formatter::{error_without_trace, info};
use tonic::{Extensions, Request, transport::Channel};
use tucana::sagittarius::{
    DataTypeUpdateRequest as SagittariusDataTypeUpdateRequest,
    data_type_service_client::DataTypeServiceClient,
};
use tucana::shared::DefinitionDataType;

pub struct SagittariusDataTypeServiceClient {
    client: DataTypeServiceClient<Channel>,
    token: String,
}

impl SagittariusDataTypeServiceClient {
    pub async fn new(sagittarius_url: String, token: String) -> Self {
        let client = match DataTypeServiceClient::connect(sagittarius_url).await {
            Ok(client) => {
                info(String::from(
                    "Successfully connected to Sagittarius DataType Endpoint!",
                ));
                client
            }
            Err(err) => panic!(
                "Failed to connect to Sagittarius (DataType Endpoint): {:?}",
                err
            ),
        };

        Self { client, token }
    }

    pub async fn update_data_types(&mut self, data_types: Vec<DefinitionDataType>) {
        let request = Request::from_parts(
            get_authorization_metadata(&self.token),
            Extensions::new(),
            SagittariusDataTypeUpdateRequest { data_types },
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

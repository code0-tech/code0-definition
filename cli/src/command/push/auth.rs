use std::str::FromStr;
use tonic::metadata::{MetadataMap, MetadataValue};

pub fn get_authorization_metadata(token: &str) -> MetadataMap {
    let metadata_value = MetadataValue::from_str(token).unwrap_or_else(|error| {
        panic!(
            "An error occurred trying to convert runtime_token into metadata: {}",
            error
        );
    });

    let mut map = MetadataMap::new();
    map.insert("authorization", metadata_value);
    map
}

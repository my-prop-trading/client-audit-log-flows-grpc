service_sdk::macros::use_settings!();

use serde_derive::{Deserialize, Serialize};

#[derive(
    service_sdk::my_settings_reader::SettingsModel,
    SdkSettingsTraits,
    AutoGenerateSettingsTraits,
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
pub struct SettingsModel {
    #[serde(rename = "PostgresConnString")]
    pub postgres_conn_string: String,
    #[serde(rename = "SeqConnString")]
    pub seq_conn_string: String,
    #[serde(rename = "MyTelemetry")]
    pub my_telemetry: String,
    #[serde(rename = "DbEncodeKey")]
    pub db_encode_key: String,
}

#[async_trait::async_trait]
impl GrpcClientSettings for SettingsReader {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        panic!("Unknown grpc service name {}", name);
    }
}

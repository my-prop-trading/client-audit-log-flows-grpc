use my_grpc_extensions::GrpcClientSettings;
use my_service_bus_tcp_client::MyServiceBusSettings;
use my_settings_reader::SettingsModel;
use serde::{Deserialize, Serialize};

#[derive(SettingsModel, Serialize, Deserialize, Debug, Clone)]
pub struct SettingsModel {
    #[serde(rename = "PostgresConnString")]
    pub postgres_conn_string: String,
    #[serde(rename = "SeqConnString")]
    pub seq_conn_string: String,
    #[serde(rename = "MyTelemetry")]
    pub my_telemetry: String,
    #[serde(rename = "ServiceBusHostPort")]
    pub service_bus_host_port: String,
    #[serde(rename = "DbEncodeKey")]
    pub db_encode_key: String,
}

#[async_trait::async_trait]
impl my_telemetry_writer::MyTelemetrySettings for SettingsReader {
    async fn get_telemetry_url(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.my_telemetry.clone()
    }
}

#[async_trait::async_trait]
impl my_seq_logger::SeqSettings for SettingsReader {
    async fn get_conn_string(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.seq_conn_string.clone()
    }
}

#[async_trait::async_trait]
impl my_postgres::PostgresSettings for SettingsReader {
    async fn get_connection_string(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.postgres_conn_string.clone()
    }
}

#[async_trait::async_trait]
impl MyServiceBusSettings for SettingsReader {
    async fn get_host_port(&self) -> String {
        let read_access = self.settings.read().await;
        read_access.service_bus_host_port.clone()
    }
}

#[async_trait::async_trait]
impl GrpcClientSettings for SettingsReader {
    async fn get_grpc_url(&self, name: &'static str) -> String {
        panic!("Unknown grpc service name {}", name);
    }
}

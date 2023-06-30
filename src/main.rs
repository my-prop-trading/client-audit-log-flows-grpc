use std::sync::Arc;

use my_seq_logger::SeqLogger;

mod app;
mod bg;
mod flows;
mod grpc;
mod grpc_client;
mod postgres;
mod settings;

pub mod client_audit_logs_grpc {
    tonic::include_proto!("client_audit_logs");
}


#[tokio::main]
async fn main() {
    let settings_reader = crate::settings::SettingsReader::new(".client-audit-logs-flows-grpc").await;

    let settings_reader = Arc::new(settings_reader);

    my_logger::LOGGER.populate_app_and_version(crate::app::APP_NAME.to_string(), app::APP_VERSION.to_string()).await;
    SeqLogger::enable_from_connection_string(settings_reader.clone());

    let telemetry_writer = my_telemetry_writer::MyTelemetryWriter::new(
        crate::app::APP_NAME.to_string(),
        settings_reader.clone(),
    );

    let app = app::AppContext::new(&settings_reader).await;

    let app = Arc::new(app);
    http_is_alive_shared::start_up::start_server(
        app::APP_NAME.to_string(),
        app::APP_VERSION.to_string(),
        app.app_states.clone(),
    );

    telemetry_writer.start(app.app_states.clone(), my_logger::LOGGER.clone());

    app.sb_client.start().await;
    tokio::spawn(crate::grpc::server::start(app.clone(), 8888));

    app.app_states.wait_until_shutdown().await;
}

use std::sync::Arc;

use client_audit_logs_grpc::client_audit_logs_grpc_service_server::ClientAuditLogsGrpcServiceServer;

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

    let mut service_context = service_sdk::ServiceContext::new(settings_reader.clone()).await;

    let app = Arc::new(app::AppContext::new(&settings_reader, &service_context).await);

    let grpc_service = grpc::grpc_service::GrpcService::new(app.clone());

    service_context.configure_grpc_server(|builder| {
        builder.add_grpc_service(ClientAuditLogsGrpcServiceServer::new(grpc_service.clone()))
    });

    service_context.start_application().await;

}

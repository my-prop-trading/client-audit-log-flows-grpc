use rust_extensions::date_time::DateTimeAsMicroseconds;

use super::server::GrpcService;
use crate::client_audit_logs_grpc::client_audit_logs_grpc_service_server::ClientAuditLogsGrpcService;
use crate::client_audit_logs_grpc::*;
use crate::postgres::dto::*;

#[async_trait::async_trait]
impl ClientAuditLogsGrpcService for GrpcService {
    async fn get_client_audit_log_paginated(
        &self,
        request: tonic::Request<GetClientAuditLogPaginatedRequest>,
    ) -> Result<tonic::Response<GetClientAuditLogPaginatedResponse>, tonic::Status> {
        let my_telemetry: my_grpc_extensions::GrpcServerTelemetryContext =
            my_grpc_extensions::get_telemetry(
                &request.metadata(),
                request.remote_addr(),
                "get_client_audit_log_paginated",
            );

        let request = request.into_inner();

        let res = self
            .app
            .postgres_repo
            .try_get_by_client_id(
                request.client_id,
                request.limit as usize,
                request.offset as usize,
                my_telemetry.get_ctx(),
            )
            .await
            .unwrap();

        let items = res
            .into_iter()
            .map(|mut x| {
                x.decrypt_fields(&self.app.aes_key);
                return x.into();
            })
            .collect::<Vec<ClientAuditLog>>();

        Ok(tonic::Response::new(GetClientAuditLogPaginatedResponse {
            items,
        }))
    }

    async fn create_client_audit_log(
        &self,
        request: tonic::Request<CreateClientAuditLogRequest>,
    ) -> Result<tonic::Response<CreateClientAuditLogResponse>, tonic::Status> {
        let my_telemetry = my_grpc_extensions::get_telemetry(
            &request.metadata(),
            request.remote_addr(),
            "create_client_audit_log",
        );

        let request = request.into_inner();

        if let Some(response) = self.app.create_log_cache.get(&request.process_id) {
            return Ok(tonic::Response::new(response.clone()));
        }

        let mut dto = ClientAuditLogDto {
            id: uuid::Uuid::new_v4().to_string(),
            client_id: request.client_id,
            created_at: DateTimeAsMicroseconds::now(),
            new_context: request.new_context,
            prev_context: request.prev_context,
            user_id: request.user_id,
            log_type: ClientAuditLogTypeDto::from_db_value(request.r#type),
        };

        let response = CreateClientAuditLogResponse {
            response: Some(create_client_audit_log_response::Response::Body(
                CreateClientAuditLogResponseBody {
                    client_audit_log: Some(dto.clone().into()),
                },
            )),
        };

        self.app.create_log_cache.insert(
            request.process_id,
            response.clone(),
        ).await;

        dto.encrypt_fields(&self.app.aes_key);

        self.app
            .postgres_repo
            .insert_or_update(dto, my_telemetry.get_ctx())
            .await
            .unwrap();

        return Ok(tonic::Response::new(response));
    }

    async fn ping(
        &self,
        request: tonic::Request<()>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let _my_telemetry =
            my_grpc_extensions::get_telemetry(&request.metadata(), request.remote_addr(), "ping");

        Ok(tonic::Response::new(()))
    }
}

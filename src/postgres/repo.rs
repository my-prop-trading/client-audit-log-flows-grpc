use std::sync::Arc;
use std::time::Duration;
use service_sdk::my_postgres::{PostgresConnection, SqlOperationWithRetries};

service_sdk::macros::use_my_postgres!();

use super::dto::*;

pub const CLIENT_AUDIT_LOGS_TABLE_NAME: &str = "client_audit_logs";
pub const CLIENT_AUDIT_LOGS_PK_NAME: &str = "client_audit_logs_pk";

pub struct ClientAuditLogPostgres {
    postgres: SqlOperationWithRetries,
}

impl ClientAuditLogPostgres {
    pub async fn new(psql_conn: &Arc<PostgresConnection>) -> Self {
        Self {
            postgres: MyPostgres::from_connection_string(Arc::clone(psql_conn))
            .with_table_schema_verification::<ClientAuditLogDto>(
                CLIENT_AUDIT_LOGS_TABLE_NAME,
                CLIENT_AUDIT_LOGS_PK_NAME.to_string().into(),
            )
            .build()
            .await
            .with_retries(5, Duration::from_millis(200)),
        }
    }

    pub async fn try_get_by_client_id(
        &self,
        client_id: Option<String>,
        limit: usize,
        offset: usize,
        my_telemetry_context: &MyTelemetryContext,
    ) -> Result<Vec<ClientAuditLogDto>, MyPostgresError> {
        if let Some(client_id) = client_id {
            let result: Vec<ClientAuditLogDto> = self
                .postgres
                .query_rows::<ClientAuditLogDto, WhereByIdWithPaginationModel>(
                    CLIENT_AUDIT_LOGS_TABLE_NAME,
                    Some(&WhereByIdWithPaginationModel {
                        client_id: &client_id,
                        limit,
                        offset,
                    }),
                    Some(my_telemetry_context),
                )
                .await?;

            return Ok(result);
        }

        let result: Vec<ClientAuditLogDto> = self
            .postgres
            .query_rows::<ClientAuditLogDto, WhereByAllWithPaginationModel>(
                CLIENT_AUDIT_LOGS_TABLE_NAME,
                Some(&WhereByAllWithPaginationModel {
                    limit,
                    offset,
                }),
                Some(my_telemetry_context),
            )
            .await?;

        Ok(result)
    }

    pub async fn insert_or_update(
        &self,
        dto: ClientAuditLogDto,
        telemetry_context: &MyTelemetryContext,
    ) -> Result<(), MyPostgresError> {
        self.postgres
            .insert_or_update_db_entity(
                CLIENT_AUDIT_LOGS_TABLE_NAME,
                UpdateConflictType::OnPrimaryKeyConstraint(CLIENT_AUDIT_LOGS_PK_NAME.to_string().into()),
                &dto,
                Some(telemetry_context),
            )
            .await?;
        Ok(())
    }
}

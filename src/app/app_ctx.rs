use encryption::aes::AesKey;
use service_sdk::{rust_extensions::AppStates, ServiceContext};
use std::sync::Arc;

use crate::postgres::ClientAuditLogPostgres;
use moka::future::Cache;

pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub struct AppContext {
    pub postgres_repo: Arc<ClientAuditLogPostgres>,
    pub app_states: Arc<AppStates>,
    pub create_log_cache:
        Arc<Cache<String, crate::client_audit_logs_grpc::CreateClientAuditLogResponse>>,
    pub aes_key: AesKey,
}

impl AppContext {
    pub async fn new(
        settings_reader: &Arc<crate::settings::SettingsReader>,
        service_context: &ServiceContext,
    ) -> Self {
        let settings = settings_reader.get_settings().await;
        let aes_key = AesKey::new(settings.db_encode_key.as_bytes());
        let postgres_repo = Arc::new(ClientAuditLogPostgres::new(settings_reader.clone()).await);

        let create_account_cache: Cache<
            String,
            crate::client_audit_logs_grpc::CreateClientAuditLogResponse,
        > = Cache::new(1000);

        Self {
            postgres_repo,
            app_states: Arc::new(AppStates::create_initialized()),
            create_log_cache: Arc::new(create_account_cache),
            aes_key,
        }
    }
}

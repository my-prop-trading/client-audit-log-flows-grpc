use encryption::aes::AesKey;
use my_service_bus_tcp_client::MyServiceBusClient;
use rust_extensions::AppStates;
use std::sync::Arc;

use crate::postgres::ClientAuditLogPostgres;
use moka::future::Cache;

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
pub struct AppContext {
    pub postgres_repo: Arc<ClientAuditLogPostgres>,
    pub app_states: Arc<AppStates>,
    pub create_log_cache:
        Arc<Cache<String, crate::client_audit_logs_grpc::CreateClientAuditLogResponse>>,
    pub aes_key: AesKey,
    //pub payment_order_publisher: Arc<MyServiceBusPublisher<PaymentOrderSbModel>>,
    pub sb_client: Arc<MyServiceBusClient>,
}

impl AppContext {
    pub async fn new(settings_reader: &Arc<crate::settings::SettingsReader>) -> Self {
        let settings = settings_reader.get_settings().await;
        let aes_key = AesKey::new(settings.db_encode_key.as_bytes());
        let postgres_repo = Arc::new(ClientAuditLogPostgres::new(settings_reader.clone()).await);

        let create_account_cache: Cache<
            String,
            crate::client_audit_logs_grpc::CreateClientAuditLogResponse,
        > = Cache::new(1000);

        let sb_client = Arc::new(MyServiceBusClient::new(
            &APP_NAME,
            &APP_VERSION.clone(),
            settings_reader.clone(),
            my_logger::LOGGER.clone(),
        ));

        //let payment_order_publisher = Arc::new(sb_client.get_publisher(true).await);

        /*         sb_client
        .subscribe(
            APP_NAME.to_string(),
            TopicQueueType::PermanentWithSingleConnection,
            Arc::new(crate::bg::OrderPaidJob::new(
                trader_accounts_flows_grpc_service.clone(),
                postgres_repo.clone(),
            )),
        )
        .await; */

        Self {
            postgres_repo,
            app_states: Arc::new(AppStates::create_initialized()),
            create_log_cache: Arc::new(create_account_cache),
            aes_key,
            sb_client,
        }
    }
}

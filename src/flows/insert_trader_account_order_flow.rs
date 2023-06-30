use crate::{
    app::AppContext,
    orders_grpc::CreateTraderAccountOrderRequest,
    postgres::{
        dto::{OrderDetailsDto, OrderDto},
        BrokerSerializable, PhaseTraderPackageAttemptSerializable, TraderPackageSerializable,
    },
};
use my_telemetry::MyTelemetryContext;
use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Debug)]
pub enum CreateTraderAccountOrderFlowError {
    PackageNotFound,
}

pub async fn insert_trader_account_order_flow(
    app: &AppContext,
    request: &CreateTraderAccountOrderRequest,
    telemetry_ctx: &MyTelemetryContext,
) -> Result<(String, DateTimeAsMicroseconds), CreateTraderAccountOrderFlowError> {
    let client_id = request.client_id.clone();
    let trading_package_id = request.trading_package_id.clone();
    let published_packages = app
        .trader_package_flows_grpc_service
        .get_all_published(
            crate::trader_packages_grpc::GetAllPublishedRequest {},
            telemetry_ctx,
        )
        .await
        .unwrap()
        .items;

    let package = published_packages
        .into_iter()
        .find(|x| x.id == trading_package_id);

    if let Some(package) = package {
        let price_currency = package.price_currency.clone();
        let package_ser = TraderPackageSerializable {
            trading_package_id: package.id.clone(),
            title: package.title,
            account_balance: package.account_balance,
            account_balance_currency: package.account_balance_currency,
            price: package.price,
            price_currency: package.price_currency,
            leverage: package.leverage,
            label: package.label,
            phase1_daily_drawdown: package.phase1_daily_drawdown,
            phase1_overall_drawdown: package.phase1_overall_drawdown,
            phase1_target_profit: package.phase1_target_profit,
            phase1_duration: package.phase1_duration,
            phase1_min_trading_days: package.phase1_min_trading_days,
            phase1_min_opened_positions: package.phase1_min_opened_positions,
            phase1_revenue_share: package.phase1_revenue_share,
            phase1_refund: package.phase1_refund,
            phase1_attempts: package
                .phase1_attempts
                .iter()
                .map(|x| PhaseTraderPackageAttemptSerializable {
                    id: x.id.clone(),
                    trading_package_id: x.trading_package_id.clone(),
                    price: x.price,
                    price_currency: x.price_currency.clone(),
                })
                .collect::<Vec<PhaseTraderPackageAttemptSerializable>>(),
            phase2_daily_drawdown: package.phase2_daily_drawdown,
            phase2_overall_drawdown: package.phase2_overall_drawdown,
            phase2_target_profit: package.phase2_target_profit,
            phase2_duration: package.phase2_duration,
            phase2_min_trading_days: package.phase2_min_trading_days,
            phase2_min_opened_positions: package.phase2_min_opened_positions,
            phase2_revenue_share: package.phase2_revenue_share,
            phase2_refund: package.phase2_refund,
            phase2_attempts: package
                .phase2_attempts
                .iter()
                .map(|x| PhaseTraderPackageAttemptSerializable {
                    id: x.id.clone(),
                    trading_package_id: x.trading_package_id.clone(),
                    price: x.price,
                    price_currency: x.price_currency.clone(),
                })
                .collect::<Vec<PhaseTraderPackageAttemptSerializable>>(),
            daily_drawdown: package.daily_drawdown,
            overall_drawdown: package.overall_drawdown,
            revenue_share: package.revenue_share,
            target_profit: package.target_profit,
            broker: BrokerSerializable::from(request.broker_model.clone()),
            trading_platform: crate::postgres::TradingPlatformSerializable::from(
                request.trading_platform.clone(),
            ),
        };

        let serialized_package = serde_json::to_string(&package_ser).unwrap();
        let order_id = uuid::Uuid::new_v4().to_string();

        app.personal_data_flows_grpc_service
            .set_for_order(
                crate::pd_grpc::SetOrderPersonalDataRequest {
                    order_id: order_id.clone(),
                    order_personal_data_model: Some(crate::pd_grpc::OrderPersonalDataModel {
                        client_id: client_id.clone(),
                        first_name: request.first_name.clone(),
                        last_name: request.last_name.clone(),
                        city: request.city.clone(),
                        country: request.country.clone(),
                        zip_code: request.zip_code.clone(),
                        address: request.address.clone(),
                        phone: request.phone.clone(),
                    }),
                    process_id: format!("create_order_{}", order_id),
                },
                telemetry_ctx,
            )
            .await;

        let now = DateTimeAsMicroseconds::now();
        let dto = OrderDto {
            id: order_id.clone(),
            client_id: client_id.clone(),
            price: package.price,
            price_currency: price_currency,
            created_at: Some(now),
            status: crate::postgres::dto::OrderStatusDto::Pending,
            order_type: crate::postgres::dto::OrderType::Challenge,
        };

        let order_details = OrderDetailsDto {
            order_id: order_id.clone(),
            trading_package: serialized_package,
        };

        app.postgres_repo
            .insert_or_update(dto, telemetry_ctx)
            .await
            .unwrap();

        app.postgres_repo
            .insert_or_update_details(order_details, telemetry_ctx)
            .await
            .unwrap();

        return Ok((order_id, now));
    } else {
        return Err(CreateTraderAccountOrderFlowError::PackageNotFound);
    }
}

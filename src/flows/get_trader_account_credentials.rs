/* use crate::{
    app::AppContext,
    postgres::dto::{
        TraderAccountLoginCredentialsDto,
        TradingPlatformDto,
    },
    trader_accounts_grpc::{
        get_trader_account_credentials_response, GetTraderAccountCredentialsRequest,
        GetTraderAccountCredentialsResponse, GetTraderAccountCredentialsResponseBody,
        TraderAccountError,
    },
};
use my_telemetry::MyTelemetryContext;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Debug)]
pub enum GetTraderCredentailsFlowError {
}

pub async fn get_trader_credentials_flow(
    app: &AppContext,
    request: GetTraderAccountCredentialsRequest,
    telemetry_ctx: &MyTelemetryContext,
) -> Result<GetTraderAccountCredentialsResponse, GetTraderCredentailsFlowError> {
    let client_id = request.client_id;
    let account_id = request.account_id;
    let process_id = format!("create-trader-account-creds:{}", account_id);

    let trader_account = app
        .postgres_repo
        .try_get_by_account_id(&account_id, telemetry_ctx)
        .await
        .unwrap();

    if trader_account.is_none() {
        return Ok(GetTraderAccountCredentialsResponse {
            response: Some(get_trader_account_credentials_response::Response::Error(
                TraderAccountError::NoAccount.into(),
            )),
        });
    }

    let trader_account = trader_account.unwrap();

    if trader_account.client_id != client_id {
        return Ok(GetTraderAccountCredentialsResponse {
            response: Some(get_trader_account_credentials_response::Response::Error(
                TraderAccountError::Unauthorized.into(),
            )),
        });
    }

    let creds = app
        .postgres_repo
        .get_creds(&account_id, telemetry_ctx)
        .await
        .unwrap();

    let server = match trader_account.trading_platform {
        TradingPlatformDto::MetaTrader4 => {
            if let Some(server) = app.metatrader4_server_map.get(&trader_account.broker) {
                server.clone()
            } else {
                "".into()
            }
        }
        TradingPlatformDto::MetaTrader5 => {
            if let Some(server) = app.metatrader5_server_map.get(&trader_account.broker) {
                server.clone()
            } else {
                "".into()
            }
        }
    };

    if let Some(mut creds) = creds {
        creds.decrypt_fields(&app.aes_key);
        let response: GetTraderAccountCredentialsResponse = GetTraderAccountCredentialsResponse {
            response: Some(get_trader_account_credentials_response::Response::Body(
                GetTraderAccountCredentialsResponseBody {
                    client_id: creds.client_id,
                    account_id: creds.account_id,
                    trade_password: creds.trade_password,
                    view_password: creds.view_password,
                    login: creds.login.parse::<i64>().unwrap(),
                    server: server,
                },
            )),
        };

        return Ok(response);
    }

    let personal_data = app
        .personal_data_flows_grpc_service
        .get(&client_id, telemetry_ctx)
        .await
        .unwrap();

    if personal_data.first_name.is_none() || personal_data.last_name.is_none() {
        let response = GetTraderAccountCredentialsResponse {
            response: Some(get_trader_account_credentials_response::Response::Error(
                TraderAccountError::NoPersonalData as i32,
            )),
        };
        return Ok(response);
    }

    let full_name = format!(
        "{} {}",
        personal_data.first_name(),
        personal_data.last_name()
    );

    let view_password = generate_password();
    let trade_password = generate_password();
    let view_pass_response = view_password.clone();
    let trade_pass_response = trade_password.clone();

    let login = match trader_account.trading_platform {
        TradingPlatformDto::MetaTrader4 => app
            .metatrader4_flows_grpc_service
            .create_account(
                full_name,
                trade_password.clone(),
                view_password.clone(),
                client_id.clone(),
                process_id,
                telemetry_ctx,
            )
            .await
            .unwrap(),
        TradingPlatformDto::MetaTrader5 => app
            .metatrader5_flows_grpc_service
            .create_account(
                full_name,
                trade_password.clone(),
                view_password.clone(),
                client_id.clone(),
                process_id,
                telemetry_ctx,
            )
            .await
            .unwrap(),
    };

    let login_response = login.clone();
    let now = DateTimeAsMicroseconds::now();
    let mut dto = TraderAccountLoginCredentialsDto {
        account_id: account_id.clone(),
        updated_date: Some(now),
        created_date: Some(now),
        client_id: client_id.clone(),
        login: format!("{}", login),
        trade_password: trade_password,
        view_password: view_password,
        sequence: 0,
    };
    dto.encrypt_fields(&app.aes_key);
    app.postgres_repo
        .insert_creds(&dto, telemetry_ctx)
        .await
        .unwrap();

    let response = GetTraderAccountCredentialsResponse {
        response: Some(get_trader_account_credentials_response::Response::Body(
            GetTraderAccountCredentialsResponseBody {
                client_id: client_id,
                account_id: account_id,
                trade_password: trade_pass_response,
                view_password: view_pass_response,
                login: login_response,
                server: server,
            },
        )),
    };

    return Ok(response);
}

fn generate_password() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect()
}
 */
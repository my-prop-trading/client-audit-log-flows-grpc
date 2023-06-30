use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhaseTraderPackageAttemptSerializable {
    pub id: String,
    pub trading_package_id: String,
    pub price: f64,
    pub price_currency: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraderPackageSerializable {
    pub trading_package_id: String,
    pub title: String,
    pub account_balance: f64,
    pub account_balance_currency: String,
    pub price: f64,
    pub price_currency: String,
    pub leverage: i32,
    pub label: String,

    pub phase1_daily_drawdown: f64,
    pub phase1_overall_drawdown: f64,
    pub phase1_target_profit: f64,
    pub phase1_duration: i32,
    pub phase1_min_trading_days: Option<i32>,
    pub phase1_min_opened_positions: Option<i32>,
    pub phase1_revenue_share: Option<f64>,
    pub phase1_refund: f64,
    pub phase1_attempts: Vec<PhaseTraderPackageAttemptSerializable>,

    pub phase2_daily_drawdown: f64,
    pub phase2_overall_drawdown: f64,
    pub phase2_target_profit: f64,
    pub phase2_duration: i32,
    pub phase2_min_trading_days: Option<i32>,
    pub phase2_min_opened_positions: Option<i32>,
    pub phase2_revenue_share: Option<f64>,
    pub phase2_refund: f64,
    pub phase2_attempts: Vec<PhaseTraderPackageAttemptSerializable>,

    pub daily_drawdown: f64,
    pub overall_drawdown: f64,

    pub revenue_share: f64,
    pub target_profit: f64,

    pub trading_platform: TradingPlatformSerializable,
    pub broker: BrokerSerializable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradingPlatformSerializable {
    MetaTrader4 = 0,
    MetaTrader5 = 1,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BrokerSerializable {
    Welltrade = 0,
}

impl From<i32> for TradingPlatformSerializable {
    fn from(item: i32) -> Self {
        match item {
            0 => TradingPlatformSerializable::MetaTrader4,
            1 => TradingPlatformSerializable::MetaTrader5,
            _ => panic!("Invalid value!"),
        }
    }
}

impl From<TradingPlatformSerializable> for i32 {
    fn from(item: TradingPlatformSerializable) -> Self {
        item as i32
    }
}

impl From<i32> for BrokerSerializable {
    fn from(item: i32) -> Self {
        match item {
            0 => BrokerSerializable::Welltrade,
            _ => panic!("Invalid value!"),
        }
    }
}

impl From<BrokerSerializable> for i32 {
    fn from(item: BrokerSerializable) -> Self {
        item as i32
    }
}
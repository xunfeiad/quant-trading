use super::super::order::InstrumentType;
use super::channel::{IndexCandleChannel, MarkPriceChannel};
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderArg {
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,

    // 产品id
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
    // 交易品种 适用于交割/永续/期权
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,
}
/**
*  {
     "channel": "account",
     "extraParams": "
        {
          \"updateInterval\": \"0\"
        }
      "
   }
*/
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountArg {
    pub ccy: Option<String>,
    #[serde(rename = "extraParams")]
    pub extra_params: Option<String>,
    #[serde(rename = "updateInterval")]
    pub update_interval: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionArg {
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
    #[serde(rename = "extraParams")]
    pub extra_params: Option<String>,
    #[serde(rename = "updateInterval")]
    pub update_interval: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceAndPositionArg;

#[derive(Debug, Serialize, Deserialize)]
pub struct LiquidationWarningArg {
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,
    #[serde(rename = "instFamily")]
    pub inst_family: Option<String>,
    #[serde(rename = "instId")]
    pub inst_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountGreeksArg {
    pub ccy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstrumentArg {
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginArgs {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    pub passphrase: String,
    pub timestamp: f64,
    pub sign: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WsPrivateArg {
    Login(LoginArgs),
    Order(OrderArg),
    Account(AccountArg),
    Position(PositionArg),
    BalanceAndPosition(BalanceAndPositionArg),
    LiquidationWarning(LiquidationWarningArg),
    AccountGreeks(AccountGreeksArg),
    Instrument(InstrumentArg),
}

impl WsPrivateArg {
    pub fn as_value(&self) -> Result<Value> {
        let mut ws_args = match self {
            Self::Order(arg) => serde_json::to_value(arg)?,
            Self::Account(arg) => serde_json::to_value(arg)?,
            Self::Position(arg) => serde_json::to_value(arg)?,
            Self::BalanceAndPosition(_) => json!({}), // Empty object for BalanceAndPosition
            Self::LiquidationWarning(arg) => serde_json::to_value(arg)?,
            Self::AccountGreeks(arg) => serde_json::to_value(arg)?,
            Self::Instrument(arg) => serde_json::to_value(arg)?,
            Self::Login(arg) => serde_json::to_value(arg)?,
        };

        if ws_args.get("channel").is_none() {
            // If the channel field is not set, we need to add it
            let channel = self.as_str();
            ws_args["channel"] = json!(channel);
        }

        Ok(ws_args)
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Order(_) => "orders",
            Self::Account(_) => "account",
            Self::Position(_) => "positions",
            Self::BalanceAndPosition(_) => "balance_and_position",
            Self::LiquidationWarning(_) => "liquidation_warning",
            Self::AccountGreeks(_) => "account_greeks",
            Self::Instrument(_) => "instruments",
            Self::Login(_) => "login",
        }
    }
}

/// 产品频道，
#[derive(Debug, Serialize, Deserialize)]
pub struct WsPublicInstTypeArg {
    #[serde(rename = "instType")]
    pub inst_type: InstrumentType,
}

/// 持仓总量频道
/// 资金费率频道
/// 限价频道
/// 标记价格
/// 指数行情
/// 标记价格k线
#[derive(Debug, Serialize, Deserialize)]
pub struct WsPublicInstIdArg {
    #[serde(rename = "instId")]
    pub inst_id: String,
}

/// 期权定价频道
#[derive(Debug, Serialize, Deserialize)]
pub struct WsPublicInstFamilyArg {
    #[serde(rename = "instFamily")]
    pub inst_family: String,
}

pub struct WsPublicNoneArg;

#[derive(Debug, Serialize, Deserialize)]
pub enum WsPublicArg {
    // instruments
    Instrument(WsPublicInstTypeArg),
    // open-interest
    OpenInterest(WsPublicInstIdArg),
    // funding-rate
    FundingRate(WsPublicInstIdArg),
    // price-limit
    LimitPrice(WsPublicInstIdArg),
    // opt-summary
    OptionSummary(WsPublicInstFamilyArg),
    // estimated-price
    EstimatedPrice(
        WsPublicInstIdArg,
        WsPublicInstTypeArg,
        WsPublicInstFamilyArg,
    ),
    // mark-price
    MarkPrice(WsPublicInstIdArg),
    // index-tickers
    IndexTickerPrice(WsPublicInstIdArg),
    // mark-price-kline
    MarkPriceKline(WsPublicInstIdArg, MarkPriceChannel),
    // index-candle
    IndexCandle(WsPublicInstIdArg, IndexCandleChannel),
    // liquidation-orders
    LiquidationOrders(WsPublicInstTypeArg),
    // adl-warning
    AdlWarning(WsPublicInstTypeArg, WsPublicInstFamilyArg),
    // economic-calendar
    EconomicCalendar(WsPublicInstTypeArg),
}

impl WsPublicArg {
    pub fn as_str(&self) -> &str {
        match self {
            WsPublicArg::Instrument(_) => "instruments",
            WsPublicArg::OpenInterest(_) => "open-interest",
            WsPublicArg::FundingRate(_) => "funding-rate",
            WsPublicArg::LimitPrice(_) => "price-limit",
            WsPublicArg::OptionSummary(_) => "opt-summary",
            WsPublicArg::EstimatedPrice(_, _, _) => "estimated-price",
            WsPublicArg::MarkPrice(_) => "mark-price",
            WsPublicArg::IndexTickerPrice(_) => "index-tickers",
            WsPublicArg::MarkPriceKline(_, _) => "mark-price-kline",
            WsPublicArg::IndexCandle(_, _) => "index-candle",
            WsPublicArg::LiquidationOrders(_) => "liquidation-orders",
            WsPublicArg::AdlWarning(_, _) => "adl-warning",
            WsPublicArg::EconomicCalendar(_) => "economic-calendar",
        }
    }

    pub fn as_value(&self) -> Result<Value> {
        let mut ws_args = match self {
            Self::EstimatedPrice(inst_id, inst_type, inst_family) => {
                let mut value = serde_json::to_value(inst_id)?;
                value["instId"] = json!(inst_id.inst_id);
                value["instType"] = json!(inst_type.inst_type);
                value["instFamily"] = json!(inst_family.inst_family);
                value
            }
            Self::IndexCandle(arg, channel) => {
                let mut value = serde_json::to_value(arg)?;
                value["channel"] = json!(channel.as_str());
                value
            }

            Self::AdlWarning(inst_type, inst_family) => {
                let mut value = serde_json::json!({});
                value["instType"] = json!(inst_type.inst_type);
                value["instFamily"] = json!(inst_family.inst_family);
                value
            }
            Self::OpenInterest(arg)
            | Self::FundingRate(arg)
            | Self::LimitPrice(arg)
            | Self::MarkPrice(arg)
            | Self::IndexTickerPrice(arg) => serde_json::to_value(arg)?,
            Self::MarkPriceKline(arg, channel) => {
                let mut value = serde_json::to_value(arg)?;
                value["channel"] = json!(channel.as_str());
                value
            }
            Self::OptionSummary(arg) => serde_json::to_value(arg)?,
            Self::LiquidationOrders(arg) | Self::EconomicCalendar(arg) | Self::Instrument(arg)=> serde_json::to_value(arg)?,
        };

        if ws_args.get("channel").is_none() {
            // If the channel field is not set, we need to add it
            let channel = self.as_str();
            ws_args["channel"] = json!(channel);
        }
        Ok(ws_args)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WsArg {
    Public(WsPublicArg),
    Private(WsPrivateArg),
    Business,
}

impl WsArg {
    pub fn new_public(arg: WsPublicArg) -> Self {
        WsArg::Public(arg)
    }
    pub fn new_private(arg: WsPrivateArg) -> Self {
        WsArg::Private(arg)
    }
    pub fn as_value(&self) -> Result<Value> {
        let ws_args = match self {
            WsArg::Public(arg) => arg.as_value()?,
            WsArg::Private(arg) => arg.as_value()?,
            WsArg::Business => json!({}), // Placeholder for business channel
        };
        Ok(ws_args)
    }

    pub fn as_str(&self) -> &str {
        match self {
            WsArg::Public(_) => "public",
            WsArg::Private(arg) => arg.as_str(),
            WsArg::Business => "business",
        }
    }
}

type WsClientBody = Vec<Value>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct OkexWsClientBodys {
    pub public_body: WsClientBody,
    pub private_body: WsClientBody,
    pub business_body: WsClientBody,
}

impl OkexWsClientBodys {
    pub fn new() -> Self {
        OkexWsClientBodys {
            public_body: Vec::new(),
            private_body: Vec::new(),
            business_body: Vec::new(),
        }
    }
}
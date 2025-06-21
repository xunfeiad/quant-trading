// use crate::error::Result;
// use crate::schema::okex::order::InstrumentType;
// use serde::{Deserialize, Serialize};
// use serde_json::{Value, json};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct OrderArg {
//     #[serde(rename = "instType")]
//     pub inst_type: InstrumentType,

//     // 产品id
//     #[serde(rename = "instId")]
//     pub inst_id: Option<String>,
//     // 交易品种 适用于交割/永续/期权
//     #[serde(rename = "instFamily")]
//     pub inst_family: Option<String>,
// }
// /**
// *  {
//      "channel": "account",
//      "extraParams": "
//         {
//           \"updateInterval\": \"0\"
//         }
//       "
//    }
// */
// #[derive(Debug, Serialize, Deserialize)]
// pub struct AccountArg {
//     pub ccy: Option<String>,
//     #[serde(rename = "extraParams")]
//     pub extra_params: Option<String>,
//     #[serde(rename = "updateInterval")]
//     pub update_interval: Option<u64>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct PositionArg {
//     #[serde(rename = "instType")]
//     pub inst_type: InstrumentType,
//     #[serde(rename = "instFamily")]
//     pub inst_family: Option<String>,
//     #[serde(rename = "instId")]
//     pub inst_id: Option<String>,
//     #[serde(rename = "extraParams")]
//     pub extra_params: Option<String>,
//     #[serde(rename = "updateInterval")]
//     pub update_interval: Option<u64>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct BalanceAndPositionArg;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct LiquidationWarningArg {
//     #[serde(rename = "instType")]
//     pub inst_type: InstrumentType,
//     #[serde(rename = "instFamily")]
//     pub inst_family: Option<String>,
//     #[serde(rename = "instId")]
//     pub inst_id: Option<String>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct AccountGreeksArg {
//     pub ccy: Option<String>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct InstrumentArg {
//     #[serde(rename = "instType")]
//     pub inst_type: InstrumentType,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct LoginArgs {
//     #[serde(rename = "apiKey")]
//     pub api_key: String,
//     pub passphrase: String,
//     pub timestamp: f64,
//     pub sign: String,
// }
// pub enum WsPrivateArg {
//     Login(LoginArgs),
//     Order(OrderArg),
//     Account(AccountArg),
//     Position(PositionArg),
//     BalanceAndPosition(BalanceAndPositionArg),
//     LiquidationWarning(LiquidationWarningArg),
//     AccountGreeks(AccountGreeksArg),
//     Instrument(InstrumentArg),
// }

// pub enum WsArgs {
//     Public(WsPublicArg),
//     Private(WsPrivateArg),
// }

// impl WsPrivateArg {
//     pub fn as_value(&self) -> Result<Value> {
//         let mut ws_args = match self {
//             WsArgs::Order(arg) => serde_json::to_value(arg)?,
//             WsArgs::Account(arg) => serde_json::to_value(arg)?,
//             WsArgs::Position(arg) => serde_json::to_value(arg)?,
//             WsArgs::BalanceAndPosition(_) => json!({
//                 "channel": "balance_and_position",
//             }), // Empty object for BalanceAndPosition
//             WsArgs::LiquidationWarning(arg) => serde_json::to_value(arg)?,
//             WsArgs::AccountGreeks(arg) => serde_json::to_value(arg)?,
//             WsArgs::Instrument(arg) => serde_json::to_value(arg)?,
//             WsArgs::Login(arg) => serde_json::to_value(arg)?,
//         };

//         ws_args["channel"] = json!(self.as_str());

//         Ok(ws_args)
//     }

//     pub fn as_str(&self) -> &str {
//         match self {
//             WsArgs::Order(_) => "orders",
//             WsArgs::Account(_) => "account",
//             WsArgs::Position(_) => "positions",
//             WsArgs::BalanceAndPosition(_) => "balance_and_position",
//             WsArgs::LiquidationWarning(_) => "liquidation_warning",
//             WsArgs::AccountGreeks(_) => "account_greeks",
//             WsArgs::Instrument(_) => "instruments",
//             WsArgs::Login(_) => "login",
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct WsPublicArg{
//     #[serde(rename = "instId")]
//     inst_id: String,
// }

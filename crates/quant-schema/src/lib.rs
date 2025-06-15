use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};


/// Trading symbol information
/// This struct represents a trading symbol, which includes its name and other relevant details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol{

    /// Unique identifier for the symbol
    pub name: String,

    /// Base asset of the symbol, e.g., "BTC" for Bitcoin
    /// This is the asset that is being traded, such as Bitcoin or Ethereum.
    /// For example, in "BTC/USD", "BTC" is the base asset.
    pub base_asset: String,

    /// Quote asset of the symbol, e.g., "USD" for US Dollar
    /// This is the asset that is used to price the base asset.
    /// For example, in "BTC/USD", "USD" is the quote asset.
    pub quote_asset: String,

    /// TODO: Add more fields as needed, such as exchange, market type, etc.
    /// Symbol status, e.g., "TRADING", "HALT", "LIQUIDATING"
    pub status: String,

    /// Minimum quantity for trading this symbol
    /// This is the minimum amount of the base asset that can be traded.
    /// For example, if the minimum quantity is 0.001 BTC, then min_qty must be multiple of 0.001.
    pub min_qty: f64,

    /// Tick size for trading this symbol
    /// This is the smallest price increment that can be used when placing orders.
    /// For example, if the tick size is 0.01 USD, then orders must be placed in multiples of 0.01 USD.
    pub tick_size: f64,

    /// Minimum notional value for trading this symbol
    ///  This is the minimum total value of an order, calculated as `min_qty * tick_size`.
    /// For example, if the minimum notional is 10 USD, then the total value of an order must be at least 10 USD.
    pub min_notional: f64
}
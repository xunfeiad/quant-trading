pub mod error;
pub mod okex;

use serde::{Deserialize, Serialize};

/// Trading symbol information
/// This struct represents a trading symbol, which includes its name and other relevant details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    /// Name for the symbol, e.g., "BTC/USD"
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
    pub min_notional: f64,
}

/// Ticker data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    /// Trading symbol for this ticker
    pub symbol: Symbol,

    /// Exchange where this ticker is traded
    /// This is the name of the exchange where the ticker is listed, such as "Binance", "Coinbase", etc.
    pub exchange: String,

    /// Timestamp of the last update, UTC format
    pub timestamp: f64,

    /// Last price for the symbol
    pub last_price: f64,

    /// Bid price for the symbol
    /// This is the highest price that a buyer is willing to pay for the base asset.
    /// For example, if the bid price is 50000 USD, then buyers are willing to pay up to 50000 USD for 1 BTC.
    pub bid_price: f64,

    /// Ask price for the symbol
    /// This is the lowest price that a seller is willing to accept for the base asset.
    /// For example, if the ask price is 50010 USD, then sellers are willing to sell 1 BTC for at least 50010 USD.
    pub ask_price: f64,

    /// Volume for the symbol over the last 24 hours
    pub volume_24h: f64,

    ///  Quote volume for the symbol over the last 24 hours
    pub quote_volumn_24h: f64,

    /// 24-hour high price for the symbol
    pub high_24h: f64,

    /// 24-hour low price for the symbol
    pub low_24h: f64,

    /// 24-hour open price for the symbol
    pub open_24h: f64,

    /// 24-hour close price for the symbol
    pub close_24h: f64,

    /// 24-hour price change percentage
    pub price_change_24h: f64,

    /// 24-hour price change in percentage(absolute value)
    pub price_change_percent_24h: f64,

    /// weight average price(WAP) for the symbol over the last 24 hours
    ///  This is the average price of the base asset over the last 24 hours, weighted by volume.
    /// For example, if the WAP is 50000 USD, then the average price of 1 BTC over the last 24 hours is 50000 USD.
    /// It is calculated as the sum of (price * volume) / total volume.
    /// This is useful for understanding the average price at which the asset has traded over the last 24 hours.
    pub weighted_avg_price_24h: f64,

    /// Previous close price for the symbol
    pub prev_close_price: f64,

    /// count of trades in the last 24 hours
    pub count_24h: u64,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum Exchange {
    #[default]
    #[serde(rename = "okex")]
    Okex,
    #[serde(rename = "binance")]
    Binance,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OkexWsChannel {
    /// 订单频道
    #[serde(rename = "orders")]
    Order,

    /// 账户频道
    #[serde(rename = "account")]
    Account,

    /// 持仓频道
    #[serde(rename = "positions")]
    Position,

    /// 账户余额和持仓频道
    #[serde(rename = "balance_and_position")]
    BalanceAndPosition,

    /// 爆仓风险预警推送频道
    #[serde(rename = "liquidation-warning")]
    LiquidationWarning,

    /// 账户Greeks频道（期权相关）
    #[serde(rename = "account-greeks")]
    AccountGreeks,
}

impl OkexWsChannel {
    pub fn as_str(&self) -> &str {
        match self {
            OkexWsChannel::Order => "orders",
            OkexWsChannel::Account => "account",
            OkexWsChannel::Position => "positions",
            OkexWsChannel::BalanceAndPosition => "balance_and_position",
            OkexWsChannel::LiquidationWarning => "liquidation-warning",
            OkexWsChannel::AccountGreeks => "account-greeks",
        }
    }
}

/// WebSocket 频道类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WsChannelType {
    /// 公共频道 - 不需要认证的市场数据
    Public,
    /// 私有频道 - 需要认证的账户数据
    Private,
    /// 业务频道 - 业务相关数据
    Business,
}

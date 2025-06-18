use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrder {
    id: String,
    op: String,
    args: Vec<Instrument>,
    #[serde(rename = "expTime")]
    pub exp_time: Option<String>,
}

/// 具体的参数
#[derive(Serialize, Deserialize, Debug)]
pub struct Instrument {
    /// 产品id，如 `BTC-USD`
    #[serde(rename = "instId")]
    pub inst_id: String,

    /// 交易模式
    /// * `isolated` - 逐仓模式
    /// * `cross` - 全仓模式
    /// * `cash` - 现金模式
    /// * `spot_isolated` - 现货逐仓模式
    #[serde(rename = "tdMode")]
    pub td_mode: TradingMode,

    /// 保证金币种
    #[serde(rename = "ccy")]
    pub ccy: String,

    #[serde(rename = "clOrdId")]
    pub cl_ord_id: Option<String>,
    pub tag: Option<String>,

    pub side: Side,
    #[serde(rename = "posSide")]
    pub pos_side: Option<PositionSide>,
    #[serde(rename = "ordType")]
    pub ord_type: OrderType,
    /// 委托数量
    #[serde(rename = "sz")]
    pub sz: String,
    /// 委托价格
    #[serde(rename = "px")]
    pub px: String,

    #[serde(rename = "pxUsd")]
    pub px_usd: Option<String>,

    /// 以隐含波动率进行期权下单，必须指定 `pxVol`
    ///  * 例如：`pxVol = "0.2"` 表示 20% 的隐含波动率
    /// * 如果市场预期未来价格会大起大落，IV 就会上升；
    /// * 如果市场认为未来很平稳，IV 就会降低。
    #[serde(rename = "pxVol")]
    pub px_vol: Option<String>,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "tgtCcy")]
    pub tgt_ccy: Option<QuantityUnit>,
    #[serde(rename = "banAmend")]
    pub ban_amend: Option<bool>,
    #[serde(rename = "quickMgnType")]
    pub quick_mgn_type: Option<bool>,
    #[serde(rename = "stpId")]
    pub stp_id: Option<String>,

    #[serde(rename = "stpMode")]
    pub stp_mode: Option<SelfTradePreventionMode>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum TradingMode {
    /// 逐仓模式 - 每个持仓独立管理保证金
    #[serde(rename = "isolated")]
    Isolated,

    /// 全仓模式 - 所有持仓共享账户保证金
    #[serde(rename = "cross")]
    Cross,

    #[default]
    /// 现金模式 - 现货交易，无杠杆
    #[serde(rename = "cash")]
    Cash,

    /// 现货逐仓模式 - 现货带单专用
    #[serde(rename = "spot_isolated")]
    SpotIsolated,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Side {
    #[serde(rename = "buy")]
    Buy,

    #[serde(rename = "sell")]
    Sell,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub enum PositionSide {
    #[default]
    #[serde(rename = "net")]
    Net,

    #[serde(rename = "long")]
    Long,

    #[serde(rename = "short")]
    Short,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    /// 市价单 - 立即按市场价格成交
    /// * 立即成交，按当前市场最优价格
    /// * 适用于：快速进出场，不关心价格滑点
    /// * 不需要指定价格
    #[serde(rename = "market")]
    Market,

    /// 限价单 - 按指定价格或更好价格成交
    /// * 按指定价格或更好价格成交
    /// * 适用于：精确控制成交价格
    #[serde(rename = "limit")]
    Limit,

    /// 只做Maker单 - 只能作为挂单方，不能立即成交
    /// 只能挂单，不能立即成交
    #[serde(rename = "post_only")]
    PostOnly,

    /// 全部成交或立即取消 - 要么全部成交，要么全部取消
    /// 要么全部成交，要么全部取消
    /// 适用于：必须完整执行的大额订单
    #[serde(rename = "fok")]
    FillOrKill,

    /// 立即成交并取消剩余 - 立即成交能成交的部分，取消剩余
    /// 适用于：尽快成交，不关心是否全部成交
    #[serde(rename = "ioc")]
    ImmediateOrCancel,

    /// 市价委托立即成交并取消剩余 - 合约专用的市价IOC
    #[serde(rename = "optimal_limit_ioc")]
    OptimalLimitIoc,

    /// 做市商保护 - 期权专用，防止异常成交
    #[serde(rename = "mmp")]
    MarketMakerProtection,

    /// 做市商保护+只做Maker - 期权专用组合
    #[serde(rename = "mmp_and_post_only")]
    MmpAndPostOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuantityUnit {
    /// 基础货币 - 交易的主要币种 (如 BTC)
    #[serde(rename = "base_ccy")]
    BaseCurrency,

    /// 计价货币 - 用于定价的币种 (如 USDT)
    #[serde(rename = "quote_ccy")]
    QuoteCurrency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SelfTradePreventionMode {
    /// 取消挂单方 - 当发生自成交时，取消作为Maker的订单
    #[serde(rename = "cancel_maker")]
    CancelMaker,

    /// 取消吃单方 - 当发生自成交时，取消作为Taker的订单
    #[serde(rename = "cancel_taker")]
    CancelTaker,

    /// 取消双方 - 当发生自成交时，取消双方订单
    #[serde(rename = "cancel_both")]
    CancelBoth,
}

pub struct CreateOrderResponse {
    pub id: String,
    pub op: String,

    pub code: String,
    pub msg: String,
    pub data: Vec<OrderData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderData {
    #[serde(rename = "ordId")]
    pub ord_id: String,
    #[serde(rename = "clOrdId")]
    pub cl_ord_id: Option<String>,
    pub tag: Option<String>,
    pub ts: String,
    #[serde(rename = "sCode")]
    pub s_code: String,
    #[serde(rename = "sMsg")]
    pub s_msg: String,
    #[serde(rename = "inTime")]
    pub in_time: String,
    #[serde(rename = "outTime")]
    pub out_time: String,
}

/// OKX API 端点常量
pub mod okx_endpoints {
    /// 生产环境 REST API 基础地址
    pub const PROD_REST_BASE_URL: &str = "https://www.okx.com";

    /// 生产环境 WebSocket 公共频道
    pub const PROD_WS_PUBLIC_URL: &str = "wss://ws.okx.com:8443/ws/v5/public";

    /// 生产环境 WebSocket 私有频道
    pub const PROD_WS_PRIVATE_URL: &str = "wss://ws.okx.com:8443/ws/v5/private";

    /// 生产环境 WebSocket 业务频道
    pub const PROD_WS_BUSINESS_URL: &str = "wss://ws.okx.com:8443/ws/v5/business";

    /// 模拟盘 REST API 基础地址
    pub const DEMO_REST_BASE_URL: &str = "https://www.okx.com";

    /// 模拟盘 WebSocket 公共频道
    pub const DEMO_WS_PUBLIC_URL: &str = "wss://wspap.okx.com:8443/ws/v5/public";

    /// 模拟盘 WebSocket 私有频道
    pub const DEMO_WS_PRIVATE_URL: &str = "wss://wspap.okx.com:8443/ws/v5/private";

    /// 模拟盘 WebSocket 业务频道
    pub const DEMO_WS_BUSINESS_URL: &str = "wss://wspap.okx.com:8443/ws/v5/business";
}

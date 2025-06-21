use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarkPriceChannel {
    // 常规时间频道
    #[serde(rename = "mark-price-candle3M")]
    MarkPriceCandle3M,
    #[serde(rename = "mark-price-candle1M")]
    MarkPriceCandle1M,
    #[serde(rename = "mark-price-candle1W")]
    MarkPriceCandle1W,
    #[serde(rename = "mark-price-candle1D")]
    MarkPriceCandle1D,
    #[serde(rename = "mark-price-candle2D")]
    MarkPriceCandle2D,
    #[serde(rename = "mark-price-candle3D")]
    MarkPriceCandle3D,
    #[serde(rename = "mark-price-candle5D")]
    MarkPriceCandle5D,
    #[serde(rename = "mark-price-candle12H")]
    MarkPriceCandle12H,
    #[serde(rename = "mark-price-candle6H")]
    MarkPriceCandle6H,
    #[serde(rename = "mark-price-candle4H")]
    MarkPriceCandle4H,
    #[serde(rename = "mark-price-candle2H")]
    MarkPriceCandle2H,
    #[serde(rename = "mark-price-candle1H")]
    MarkPriceCandle1H,
    #[serde(rename = "mark-price-candle30m")]
    MarkPriceCandle30m,
    #[serde(rename = "mark-price-candle15m")]
    MarkPriceCandle15m,
    #[serde(rename = "mark-price-candle5m")]
    MarkPriceCandle5m,
    #[serde(rename = "mark-price-candle3m")]
    MarkPriceCandle3m,
    #[serde(rename = "mark-price-candle1m")]
    MarkPriceCandle1m,

    // UTC时间频道
    #[serde(rename = "mark-price-candle3Mutc")]
    MarkPriceCandle3MUtc,
    #[serde(rename = "mark-price-candle1Mutc")]
    MarkPriceCandle1MUtc,
    #[serde(rename = "mark-price-candle1Wutc")]
    MarkPriceCandle1WUtc,
    #[serde(rename = "mark-price-candle1Dutc")]
    MarkPriceCandle1DUtc,
    #[serde(rename = "mark-price-candle2Dutc")]
    MarkPriceCandle2DUtc,
    #[serde(rename = "mark-price-candle3Dutc")]
    MarkPriceCandle3DUtc,
    #[serde(rename = "mark-price-candle5Dutc")]
    MarkPriceCandle5DUtc,
    #[serde(rename = "mark-price-candle12Hutc")]
    MarkPriceCandle12HUtc,
    #[serde(rename = "mark-price-candle6Hutc")]
    MarkPriceCandle6HUtc,
}

impl MarkPriceChannel {
    /// 获取频道名称字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MarkPriceCandle3M => "mark-price-candle3M",
            Self::MarkPriceCandle1M => "mark-price-candle1M",
            Self::MarkPriceCandle1W => "mark-price-candle1W",
            Self::MarkPriceCandle1D => "mark-price-candle1D",
            Self::MarkPriceCandle2D => "mark-price-candle2D",
            Self::MarkPriceCandle3D => "mark-price-candle3D",
            Self::MarkPriceCandle5D => "mark-price-candle5D",
            Self::MarkPriceCandle12H => "mark-price-candle12H",
            Self::MarkPriceCandle6H => "mark-price-candle6H",
            Self::MarkPriceCandle4H => "mark-price-candle4H",
            Self::MarkPriceCandle2H => "mark-price-candle2H",
            Self::MarkPriceCandle1H => "mark-price-candle1H",
            Self::MarkPriceCandle30m => "mark-price-candle30m",
            Self::MarkPriceCandle15m => "mark-price-candle15m",
            Self::MarkPriceCandle5m => "mark-price-candle5m",
            Self::MarkPriceCandle3m => "mark-price-candle3m",
            Self::MarkPriceCandle1m => "mark-price-candle1m",
            Self::MarkPriceCandle3MUtc => "mark-price-candle3Mutc",
            Self::MarkPriceCandle1MUtc => "mark-price-candle1Mutc",
            Self::MarkPriceCandle1WUtc => "mark-price-candle1Wutc",
            Self::MarkPriceCandle1DUtc => "mark-price-candle1Dutc",
            Self::MarkPriceCandle2DUtc => "mark-price-candle2Dutc",
            Self::MarkPriceCandle3DUtc => "mark-price-candle3Dutc",
            Self::MarkPriceCandle5DUtc => "mark-price-candle5Dutc",
            Self::MarkPriceCandle12HUtc => "mark-price-candle12Hutc",
            Self::MarkPriceCandle6HUtc => "mark-price-candle6Hutc",
        }
    }

    /// 获取时间间隔（秒）
    pub fn interval_seconds(&self) -> u64 {
        match self {
            Self::MarkPriceCandle1m => 60,
            Self::MarkPriceCandle3m => 180,
            Self::MarkPriceCandle5m => 300,
            Self::MarkPriceCandle15m => 900,
            Self::MarkPriceCandle30m => 1800,
            Self::MarkPriceCandle1H => 3600,
            Self::MarkPriceCandle2H => 7200,
            Self::MarkPriceCandle4H => 14400,
            Self::MarkPriceCandle6H => 21600,
            Self::MarkPriceCandle12H => 43200,
            Self::MarkPriceCandle1D => 86400,
            Self::MarkPriceCandle2D => 172800,
            Self::MarkPriceCandle3D => 259200,
            Self::MarkPriceCandle5D => 432000,
            Self::MarkPriceCandle1W => 604800,
            Self::MarkPriceCandle1M => 2592000, // 30天近似
            Self::MarkPriceCandle3M => 7776000, // 90天近似
            // UTC版本与对应的非UTC版本间隔相同
            Self::MarkPriceCandle3MUtc => 7776000,
            Self::MarkPriceCandle1MUtc => 2592000,
            Self::MarkPriceCandle1WUtc => 604800,
            Self::MarkPriceCandle1DUtc => 86400,
            Self::MarkPriceCandle2DUtc => 172800,
            Self::MarkPriceCandle3DUtc => 259200,
            Self::MarkPriceCandle5DUtc => 432000,
            Self::MarkPriceCandle12HUtc => 43200,
            Self::MarkPriceCandle6HUtc => 21600,
        }
    }

    /// 判断是否为UTC时间频道
    pub fn is_utc(&self) -> bool {
        matches!(
            self,
            Self::MarkPriceCandle3MUtc
                | Self::MarkPriceCandle1MUtc
                | Self::MarkPriceCandle1WUtc
                | Self::MarkPriceCandle1DUtc
                | Self::MarkPriceCandle2DUtc
                | Self::MarkPriceCandle3DUtc
                | Self::MarkPriceCandle5DUtc
                | Self::MarkPriceCandle12HUtc
                | Self::MarkPriceCandle6HUtc
        )
    }

    /// 获取所有可用的频道
    pub fn all_channels() -> Vec<Self> {
        vec![
            Self::MarkPriceCandle3M,
            Self::MarkPriceCandle1M,
            Self::MarkPriceCandle1W,
            Self::MarkPriceCandle1D,
            Self::MarkPriceCandle2D,
            Self::MarkPriceCandle3D,
            Self::MarkPriceCandle5D,
            Self::MarkPriceCandle12H,
            Self::MarkPriceCandle6H,
            Self::MarkPriceCandle4H,
            Self::MarkPriceCandle2H,
            Self::MarkPriceCandle1H,
            Self::MarkPriceCandle30m,
            Self::MarkPriceCandle15m,
            Self::MarkPriceCandle5m,
            Self::MarkPriceCandle3m,
            Self::MarkPriceCandle1m,
            Self::MarkPriceCandle3MUtc,
            Self::MarkPriceCandle1MUtc,
            Self::MarkPriceCandle1WUtc,
            Self::MarkPriceCandle1DUtc,
            Self::MarkPriceCandle2DUtc,
            Self::MarkPriceCandle3DUtc,
            Self::MarkPriceCandle5DUtc,
            Self::MarkPriceCandle12HUtc,
            Self::MarkPriceCandle6HUtc,
        ]
    }

    /// 获取常用的时间间隔
    pub fn common_intervals() -> Vec<Self> {
        vec![
            Self::MarkPriceCandle1m,
            Self::MarkPriceCandle5m,
            Self::MarkPriceCandle15m,
            Self::MarkPriceCandle1H,
            Self::MarkPriceCandle4H,
            Self::MarkPriceCandle1D,
            Self::MarkPriceCandle1W,
        ]
    }
}

impl std::fmt::Display for MarkPriceChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for MarkPriceChannel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mark-price-candle3M" => Ok(Self::MarkPriceCandle3M),
            "mark-price-candle1M" => Ok(Self::MarkPriceCandle1M),
            "mark-price-candle1W" => Ok(Self::MarkPriceCandle1W),
            "mark-price-candle1D" => Ok(Self::MarkPriceCandle1D),
            "mark-price-candle2D" => Ok(Self::MarkPriceCandle2D),
            "mark-price-candle3D" => Ok(Self::MarkPriceCandle3D),
            "mark-price-candle5D" => Ok(Self::MarkPriceCandle5D),
            "mark-price-candle12H" => Ok(Self::MarkPriceCandle12H),
            "mark-price-candle6H" => Ok(Self::MarkPriceCandle6H),
            "mark-price-candle4H" => Ok(Self::MarkPriceCandle4H),
            "mark-price-candle2H" => Ok(Self::MarkPriceCandle2H),
            "mark-price-candle1H" => Ok(Self::MarkPriceCandle1H),
            "mark-price-candle30m" => Ok(Self::MarkPriceCandle30m),
            "mark-price-candle15m" => Ok(Self::MarkPriceCandle15m),
            "mark-price-candle5m" => Ok(Self::MarkPriceCandle5m),
            "mark-price-candle3m" => Ok(Self::MarkPriceCandle3m),
            "mark-price-candle1m" => Ok(Self::MarkPriceCandle1m),
            "mark-price-candle3Mutc" => Ok(Self::MarkPriceCandle3MUtc),
            "mark-price-candle1Mutc" => Ok(Self::MarkPriceCandle1MUtc),
            "mark-price-candle1Wutc" => Ok(Self::MarkPriceCandle1WUtc),
            "mark-price-candle1Dutc" => Ok(Self::MarkPriceCandle1DUtc),
            "mark-price-candle2Dutc" => Ok(Self::MarkPriceCandle2DUtc),
            "mark-price-candle3Dutc" => Ok(Self::MarkPriceCandle3DUtc),
            "mark-price-candle5Dutc" => Ok(Self::MarkPriceCandle5DUtc),
            "mark-price-candle12Hutc" => Ok(Self::MarkPriceCandle12HUtc),
            "mark-price-candle6Hutc" => Ok(Self::MarkPriceCandle6HUtc),
            _ => Err(format!("Unknown mark price channel: {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IndexCandleChannel {
    // 常规时间频道
    #[serde(rename = "index-candle3M")]
    IndexCandle3M,
    #[serde(rename = "index-candle1M")]
    IndexCandle1M,
    #[serde(rename = "index-candle1W")]
    IndexCandle1W,
    #[serde(rename = "index-candle1D")]
    IndexCandle1D,
    #[serde(rename = "index-candle2D")]
    IndexCandle2D,
    #[serde(rename = "index-candle3D")]
    IndexCandle3D,
    #[serde(rename = "index-candle5D")]
    IndexCandle5D,
    #[serde(rename = "index-candle12H")]
    IndexCandle12H,
    #[serde(rename = "index-candle6H")]
    IndexCandle6H,
    #[serde(rename = "index-candle4H")]
    IndexCandle4H,
    #[serde(rename = "index-candle2H")]
    IndexCandle2H,
    #[serde(rename = "index-candle1H")]
    IndexCandle1H,
    #[serde(rename = "index-candle30m")]
    IndexCandle30m,
    #[serde(rename = "index-candle15m")]
    IndexCandle15m,
    #[serde(rename = "index-candle5m")]
    IndexCandle5m,
    #[serde(rename = "index-candle3m")]
    IndexCandle3m,
    #[serde(rename = "index-candle1m")]
    IndexCandle1m,

    // UTC时间频道
    #[serde(rename = "index-candle3Mutc")]
    IndexCandle3MUtc,
    #[serde(rename = "index-candle1Mutc")]
    IndexCandle1MUtc,
    #[serde(rename = "index-candle1Wutc")]
    IndexCandle1WUtc,
    #[serde(rename = "index-candle1Dutc")]
    IndexCandle1DUtc,
    #[serde(rename = "index-candle2Dutc")]
    IndexCandle2DUtc,
    #[serde(rename = "index-candle3Dutc")]
    IndexCandle3DUtc,
    #[serde(rename = "index-candle5Dutc")]
    IndexCandle5DUtc,
    #[serde(rename = "index-candle12Hutc")]
    IndexCandle12HUtc,
    #[serde(rename = "index-candle6Hutc")]
    IndexCandle6HUtc,
}

impl IndexCandleChannel {
    /// 获取频道名称字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IndexCandle3M => "index-candle3M",
            Self::IndexCandle1M => "index-candle1M",
            Self::IndexCandle1W => "index-candle1W",
            Self::IndexCandle1D => "index-candle1D",
            Self::IndexCandle2D => "index-candle2D",
            Self::IndexCandle3D => "index-candle3D",
            Self::IndexCandle5D => "index-candle5D",
            Self::IndexCandle12H => "index-candle12H",
            Self::IndexCandle6H => "index-candle6H",
            Self::IndexCandle4H => "index-candle4H",
            Self::IndexCandle2H => "index-candle2H",
            Self::IndexCandle1H => "index-candle1H",
            Self::IndexCandle30m => "index-candle30m",
            Self::IndexCandle15m => "index-candle15m",
            Self::IndexCandle5m => "index-candle5m",
            Self::IndexCandle3m => "index-candle3m",
            Self::IndexCandle1m => "index-candle1m",
            Self::IndexCandle3MUtc => "index-candle3Mutc",
            Self::IndexCandle1MUtc => "index-candle1Mutc",
            Self::IndexCandle1WUtc => "index-candle1Wutc",
            Self::IndexCandle1DUtc => "index-candle1Dutc",
            Self::IndexCandle2DUtc => "index-candle2Dutc",
            Self::IndexCandle3DUtc => "index-candle3Dutc",
            Self::IndexCandle5DUtc => "index-candle5Dutc",
            Self::IndexCandle12HUtc => "index-candle12Hutc",
            Self::IndexCandle6HUtc => "index-candle6Hutc",
        }
    }

    /// 获取时间间隔（秒）
    pub fn interval_seconds(&self) -> u64 {
        match self {
            Self::IndexCandle1m => 60,
            Self::IndexCandle3m => 180,
            Self::IndexCandle5m => 300,
            Self::IndexCandle15m => 900,
            Self::IndexCandle30m => 1800,
            Self::IndexCandle1H => 3600,
            Self::IndexCandle2H => 7200,
            Self::IndexCandle4H => 14400,
            Self::IndexCandle6H => 21600,
            Self::IndexCandle12H => 43200,
            Self::IndexCandle1D => 86400,
            Self::IndexCandle2D => 172800,
            Self::IndexCandle3D => 259200,
            Self::IndexCandle5D => 432000,
            Self::IndexCandle1W => 604800,
            Self::IndexCandle1M => 2592000, // 30天近似
            Self::IndexCandle3M => 7776000, // 90天近似
            // UTC版本与对应的非UTC版本间隔相同
            Self::IndexCandle3MUtc => 7776000,
            Self::IndexCandle1MUtc => 2592000,
            Self::IndexCandle1WUtc => 604800,
            Self::IndexCandle1DUtc => 86400,
            Self::IndexCandle2DUtc => 172800,
            Self::IndexCandle3DUtc => 259200,
            Self::IndexCandle5DUtc => 432000,
            Self::IndexCandle12HUtc => 43200,
            Self::IndexCandle6HUtc => 21600,
        }
    }

    /// 获取时间间隔的人类可读格式
    pub fn interval_display(&self) -> &'static str {
        match self {
            Self::IndexCandle1m => "1 minute",
            Self::IndexCandle3m => "3 minutes",
            Self::IndexCandle5m => "5 minutes",
            Self::IndexCandle15m => "15 minutes",
            Self::IndexCandle30m => "30 minutes",
            Self::IndexCandle1H => "1 hour",
            Self::IndexCandle2H => "2 hours",
            Self::IndexCandle4H => "4 hours",
            Self::IndexCandle6H => "6 hours",
            Self::IndexCandle12H => "12 hours",
            Self::IndexCandle1D => "1 day",
            Self::IndexCandle2D => "2 days",
            Self::IndexCandle3D => "3 days",
            Self::IndexCandle5D => "5 days",
            Self::IndexCandle1W => "1 week",
            Self::IndexCandle1M => "1 month",
            Self::IndexCandle3M => "3 months",
            Self::IndexCandle3MUtc => "3 months (UTC)",
            Self::IndexCandle1MUtc => "1 month (UTC)",
            Self::IndexCandle1WUtc => "1 week (UTC)",
            Self::IndexCandle1DUtc => "1 day (UTC)",
            Self::IndexCandle2DUtc => "2 days (UTC)",
            Self::IndexCandle3DUtc => "3 days (UTC)",
            Self::IndexCandle5DUtc => "5 days (UTC)",
            Self::IndexCandle12HUtc => "12 hours (UTC)",
            Self::IndexCandle6HUtc => "6 hours (UTC)",
        }
    }

    /// 判断是否为UTC时间频道
    pub fn is_utc(&self) -> bool {
        matches!(
            self,
            Self::IndexCandle3MUtc
                | Self::IndexCandle1MUtc
                | Self::IndexCandle1WUtc
                | Self::IndexCandle1DUtc
                | Self::IndexCandle2DUtc
                | Self::IndexCandle3DUtc
                | Self::IndexCandle5DUtc
                | Self::IndexCandle12HUtc
                | Self::IndexCandle6HUtc
        )
    }

    /// 获取时间间隔类型
    pub fn interval_type(&self) -> IntervalType {
        match self {
            Self::IndexCandle1m
            | Self::IndexCandle3m
            | Self::IndexCandle5m
            | Self::IndexCandle15m
            | Self::IndexCandle30m => IntervalType::Minute,

            Self::IndexCandle1H
            | Self::IndexCandle2H
            | Self::IndexCandle4H
            | Self::IndexCandle6H
            | Self::IndexCandle12H
            | Self::IndexCandle12HUtc
            | Self::IndexCandle6HUtc => IntervalType::Hour,

            Self::IndexCandle1D
            | Self::IndexCandle2D
            | Self::IndexCandle3D
            | Self::IndexCandle5D
            | Self::IndexCandle1DUtc
            | Self::IndexCandle2DUtc
            | Self::IndexCandle3DUtc
            | Self::IndexCandle5DUtc => IntervalType::Day,

            Self::IndexCandle1W | Self::IndexCandle1WUtc => IntervalType::Week,

            Self::IndexCandle1M
            | Self::IndexCandle3M
            | Self::IndexCandle1MUtc
            | Self::IndexCandle3MUtc => IntervalType::Month,
        }
    }

    /// 获取所有可用的频道
    pub fn all_channels() -> Vec<Self> {
        vec![
            Self::IndexCandle3M,
            Self::IndexCandle1M,
            Self::IndexCandle1W,
            Self::IndexCandle1D,
            Self::IndexCandle2D,
            Self::IndexCandle3D,
            Self::IndexCandle5D,
            Self::IndexCandle12H,
            Self::IndexCandle6H,
            Self::IndexCandle4H,
            Self::IndexCandle2H,
            Self::IndexCandle1H,
            Self::IndexCandle30m,
            Self::IndexCandle15m,
            Self::IndexCandle5m,
            Self::IndexCandle3m,
            Self::IndexCandle1m,
            Self::IndexCandle3MUtc,
            Self::IndexCandle1MUtc,
            Self::IndexCandle1WUtc,
            Self::IndexCandle1DUtc,
            Self::IndexCandle2DUtc,
            Self::IndexCandle3DUtc,
            Self::IndexCandle5DUtc,
            Self::IndexCandle12HUtc,
            Self::IndexCandle6HUtc,
        ]
    }

    /// 获取常用的时间间隔
    pub fn common_intervals() -> Vec<Self> {
        vec![
            Self::IndexCandle1m,
            Self::IndexCandle5m,
            Self::IndexCandle15m,
            Self::IndexCandle1H,
            Self::IndexCandle4H,
            Self::IndexCandle1D,
            Self::IndexCandle1W,
        ]
    }

    /// 获取高频交易间隔（短周期）
    pub fn high_frequency_intervals() -> Vec<Self> {
        vec![
            Self::IndexCandle1m,
            Self::IndexCandle3m,
            Self::IndexCandle5m,
            Self::IndexCandle15m,
            Self::IndexCandle30m,
        ]
    }

    /// 获取中等频率交易间隔
    pub fn medium_frequency_intervals() -> Vec<Self> {
        vec![
            Self::IndexCandle1H,
            Self::IndexCandle2H,
            Self::IndexCandle4H,
            Self::IndexCandle6H,
            Self::IndexCandle12H,
        ]
    }

    /// 获取低频交易间隔（长周期）
    pub fn low_frequency_intervals() -> Vec<Self> {
        vec![
            Self::IndexCandle1D,
            Self::IndexCandle2D,
            Self::IndexCandle3D,
            Self::IndexCandle5D,
            Self::IndexCandle1W,
            Self::IndexCandle1M,
            Self::IndexCandle3M,
        ]
    }

    /// 获取对应的UTC版本（如果存在）
    pub fn to_utc(&self) -> Option<Self> {
        match self {
            Self::IndexCandle3M => Some(Self::IndexCandle3MUtc),
            Self::IndexCandle1M => Some(Self::IndexCandle1MUtc),
            Self::IndexCandle1W => Some(Self::IndexCandle1WUtc),
            Self::IndexCandle1D => Some(Self::IndexCandle1DUtc),
            Self::IndexCandle2D => Some(Self::IndexCandle2DUtc),
            Self::IndexCandle3D => Some(Self::IndexCandle3DUtc),
            Self::IndexCandle5D => Some(Self::IndexCandle5DUtc),
            Self::IndexCandle12H => Some(Self::IndexCandle12HUtc),
            Self::IndexCandle6H => Some(Self::IndexCandle6HUtc),
            _ => None,
        }
    }

    /// 获取对应的非UTC版本
    pub fn to_local(&self) -> Option<Self> {
        match self {
            Self::IndexCandle3MUtc => Some(Self::IndexCandle3M),
            Self::IndexCandle1MUtc => Some(Self::IndexCandle1M),
            Self::IndexCandle1WUtc => Some(Self::IndexCandle1W),
            Self::IndexCandle1DUtc => Some(Self::IndexCandle1D),
            Self::IndexCandle2DUtc => Some(Self::IndexCandle2D),
            Self::IndexCandle3DUtc => Some(Self::IndexCandle3D),
            Self::IndexCandle5DUtc => Some(Self::IndexCandle5D),
            Self::IndexCandle12HUtc => Some(Self::IndexCandle12H),
            Self::IndexCandle6HUtc => Some(Self::IndexCandle6H),
            _ => None,
        }
    }
}

/// 时间间隔类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntervalType {
    Minute,
    Hour,
    Day,
    Week,
    Month,
}

impl std::fmt::Display for IndexCandleChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for IndexCandleChannel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "index-candle3M" => Ok(Self::IndexCandle3M),
            "index-candle1M" => Ok(Self::IndexCandle1M),
            "index-candle1W" => Ok(Self::IndexCandle1W),
            "index-candle1D" => Ok(Self::IndexCandle1D),
            "index-candle2D" => Ok(Self::IndexCandle2D),
            "index-candle3D" => Ok(Self::IndexCandle3D),
            "index-candle5D" => Ok(Self::IndexCandle5D),
            "index-candle12H" => Ok(Self::IndexCandle12H),
            "index-candle6H" => Ok(Self::IndexCandle6H),
            "index-candle4H" => Ok(Self::IndexCandle4H),
            "index-candle2H" => Ok(Self::IndexCandle2H),
            "index-candle1H" => Ok(Self::IndexCandle1H),
            "index-candle30m" => Ok(Self::IndexCandle30m),
            "index-candle15m" => Ok(Self::IndexCandle15m),
            "index-candle5m" => Ok(Self::IndexCandle5m),
            "index-candle3m" => Ok(Self::IndexCandle3m),
            "index-candle1m" => Ok(Self::IndexCandle1m),
            "index-candle3Mutc" => Ok(Self::IndexCandle3MUtc),
            "index-candle1Mutc" => Ok(Self::IndexCandle1MUtc),
            "index-candle1Wutc" => Ok(Self::IndexCandle1WUtc),
            "index-candle1Dutc" => Ok(Self::IndexCandle1DUtc),
            "index-candle2Dutc" => Ok(Self::IndexCandle2DUtc),
            "index-candle3Dutc" => Ok(Self::IndexCandle3DUtc),
            "index-candle5Dutc" => Ok(Self::IndexCandle5DUtc),
            "index-candle12Hutc" => Ok(Self::IndexCandle12HUtc),
            "index-candle6Hutc" => Ok(Self::IndexCandle6HUtc),
            _ => Err(format!("Unknown index candle channel: {}", s)),
        }
    }
}

use ratatui::style::Color;
use serde::Deserialize;
use std::fmt;

/// 지원하는 EVM 체인 목록
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChainId {
    Ethereum,
    Polygon,
    Arbitrum,
    Base,
    Optimism,
    Avalanche,
    Bsc,
}

impl ChainId {
    /// 체인 숫자 ID
    pub fn chain_id_num(&self) -> u64 {
        match self {
            Self::Ethereum => 1,
            Self::Polygon => 137,
            Self::Arbitrum => 42161,
            Self::Base => 8453,
            Self::Optimism => 10,
            Self::Avalanche => 43114,
            Self::Bsc => 56,
        }
    }

    /// 탭에 표시할 짧은 이름
    pub fn display_name(&self) -> &str {
        match self {
            Self::Ethereum => "ETH",
            Self::Polygon => "POL",
            Self::Arbitrum => "ARB",
            Self::Base => "BASE",
            Self::Optimism => "OP",
            Self::Avalanche => "AVAX",
            Self::Bsc => "BSC",
        }
    }

    /// 체인 브랜드 색상
    pub fn color(&self) -> Color {
        match self {
            Self::Ethereum => Color::Blue,
            Self::Polygon => Color::Magenta,
            Self::Arbitrum => Color::Cyan,
            Self::Base => Color::LightBlue,
            Self::Optimism => Color::Red,
            Self::Avalanche => Color::LightRed,
            Self::Bsc => Color::Yellow,
        }
    }

    /// 네이티브 토큰 심볼
    pub fn native_symbol(&self) -> &str {
        match self {
            Self::Ethereum => "ETH",
            Self::Polygon => "POL",
            Self::Arbitrum => "ETH",
            Self::Base => "ETH",
            Self::Optimism => "ETH",
            Self::Avalanche => "AVAX",
            Self::Bsc => "BNB",
        }
    }

    /// 기본 WebSocket RPC URL
    pub fn default_ws_url(&self) -> &str {
        match self {
            Self::Ethereum => "wss://eth.drpc.org",
            Self::Polygon => "wss://polygon-bor-rpc.publicnode.com",
            Self::Arbitrum => "wss://arbitrum-one-rpc.publicnode.com",
            Self::Base => "wss://base-rpc.publicnode.com",
            Self::Optimism => "wss://optimism-rpc.publicnode.com",
            Self::Avalanche => "wss://avalanche-c-chain-rpc.publicnode.com",
            Self::Bsc => "wss://bsc-rpc.publicnode.com",
        }
    }

    /// 쉼표 구분 문자열에서 ChainId 목록 파싱
    pub fn from_csv(s: &str) -> Vec<ChainId> {
        s.split(',')
            .filter_map(|name| {
                match name.trim().to_lowercase().as_str() {
                    "ethereum" | "eth" => Some(ChainId::Ethereum),
                    "polygon" | "pol" | "matic" => Some(ChainId::Polygon),
                    "arbitrum" | "arb" => Some(ChainId::Arbitrum),
                    "base" => Some(ChainId::Base),
                    "optimism" | "op" => Some(ChainId::Optimism),
                    "avalanche" | "avax" => Some(ChainId::Avalanche),
                    "bsc" | "bnb" => Some(ChainId::Bsc),
                    _ => None,
                }
            })
            .collect()
    }

    /// 전체 체인 목록
    pub fn all() -> Vec<ChainId> {
        vec![
            ChainId::Ethereum,
            ChainId::Polygon,
            ChainId::Arbitrum,
            ChainId::Base,
            ChainId::Optimism,
            ChainId::Avalanche,
            ChainId::Bsc,
        ]
    }
}

impl fmt::Display for ChainId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

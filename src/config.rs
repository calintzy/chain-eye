use anyhow::Result;
use serde::Deserialize;
use std::path::PathBuf;

/// RPC 설정
#[derive(Debug, Clone, Deserialize)]
pub struct RpcConfig {
    #[serde(default = "default_ws_url")]
    pub ws_url: String,
    pub fallback_ws_url: Option<String>,
    #[serde(default = "default_max_reconnect")]
    pub max_reconnect_attempts: u32,
    #[serde(default = "default_reconnect_delay")]
    pub reconnect_base_delay_ms: u64,
}

/// UI 설정
#[derive(Debug, Clone, Deserialize)]
pub struct UiConfig {
    #[serde(default = "default_max_tx")]
    pub max_transactions: usize,
    #[serde(default = "default_tick_rate")]
    pub tick_rate_ms: u64,
    #[serde(default = "default_addr_len")]
    pub address_display_len: usize,
}

/// 전체 설정
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub rpc: RpcConfig,
    #[serde(default)]
    pub ui: UiConfig,
}

// 기본값 함수들
fn default_ws_url() -> String {
    "wss://eth.drpc.org".to_string()
}
fn default_max_reconnect() -> u32 {
    10
}
fn default_reconnect_delay() -> u64 {
    1000
}
fn default_max_tx() -> usize {
    1000
}
fn default_tick_rate() -> u64 {
    16
}
fn default_addr_len() -> usize {
    4
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            ws_url: default_ws_url(),
            fallback_ws_url: Some("wss://ethereum-rpc.publicnode.com".to_string()),
            max_reconnect_attempts: default_max_reconnect(),
            reconnect_base_delay_ms: default_reconnect_delay(),
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            max_transactions: default_max_tx(),
            tick_rate_ms: default_tick_rate(),
            address_display_len: default_addr_len(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            rpc: RpcConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

impl AppConfig {
    /// 설정 파일 로딩 (사용자 설정 > 기본값)
    pub fn load(config_path: Option<&PathBuf>) -> Result<Self> {
        // 1. 기본 설정
        let mut config = AppConfig::default();

        // 2. 사용자 설정 파일 경로 결정
        let user_config_path = config_path
            .cloned()
            .or_else(|| {
                dirs::home_dir().map(|h| h.join(".chain-eye").join("config.toml"))
            });

        // 3. 사용자 설정 파일이 존재하면 로딩 후 병합
        if let Some(path) = user_config_path {
            if path.exists() {
                let content = std::fs::read_to_string(&path)?;
                let user_config: AppConfig = toml::from_str(&content)?;
                config.merge(user_config);
            }
        }

        Ok(config)
    }

    /// 사용자 설정을 기본 설정에 병합
    fn merge(&mut self, other: AppConfig) {
        // RPC: ws_url이 기본값이 아니면 오버라이드
        if other.rpc.ws_url != default_ws_url() {
            self.rpc.ws_url = other.rpc.ws_url;
        }
        if other.rpc.fallback_ws_url.is_some() {
            self.rpc.fallback_ws_url = other.rpc.fallback_ws_url;
        }

        // UI 설정 오버라이드
        if other.ui.max_transactions != default_max_tx() {
            self.ui.max_transactions = other.ui.max_transactions;
        }
    }

    /// CLI 인자로 설정 오버라이드
    pub fn apply_cli_overrides(&mut self, rpc_url: Option<&str>) {
        if let Some(url) = rpc_url {
            self.rpc.ws_url = url.to_string();
        }
    }
}

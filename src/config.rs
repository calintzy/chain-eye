use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;

use crate::chain::chains::ChainId;

/// 체인별 설정
#[derive(Debug, Clone, Deserialize)]
pub struct ChainConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub ws_url: Option<String>,
    pub fallback_ws_url: Option<String>,
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            ws_url: None,
            fallback_ws_url: None,
        }
    }
}

/// RPC 전역 설정 (재연결 파라미터)
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

/// 지갑 추적 설정
#[derive(Debug, Clone, Deserialize, Default)]
pub struct WatchConfig {
    #[serde(default)]
    pub addresses: Vec<String>,
}

/// 전체 설정
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub chains: HashMap<ChainId, ChainConfig>,
    #[serde(default)]
    pub rpc: RpcConfig,
    #[serde(default)]
    pub ui: UiConfig,
    #[serde(default)]
    pub watch: WatchConfig,
}

// 기본값 함수들
fn default_true() -> bool {
    true
}
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
            chains: HashMap::new(),
            rpc: RpcConfig::default(),
            ui: UiConfig::default(),
            watch: WatchConfig::default(),
        }
    }
}

impl AppConfig {
    /// 설정 파일 로딩
    pub fn load(config_path: Option<&PathBuf>) -> Result<Self> {
        let mut config = AppConfig::default();

        let user_config_path = config_path
            .cloned()
            .or_else(|| dirs::home_dir().map(|h| h.join(".chain-eye").join("config.toml")));

        if let Some(path) = user_config_path {
            if path.exists() {
                let content = std::fs::read_to_string(&path)?;
                let user_config: AppConfig = toml::from_str(&content)?;
                config.merge(user_config);
            }
        }

        Ok(config)
    }

    /// 사용자 설정 병합
    fn merge(&mut self, other: AppConfig) {
        if other.rpc.ws_url != default_ws_url() {
            self.rpc.ws_url = other.rpc.ws_url;
        }
        if other.rpc.fallback_ws_url.is_some() {
            self.rpc.fallback_ws_url = other.rpc.fallback_ws_url;
        }
        if other.ui.max_transactions != default_max_tx() {
            self.ui.max_transactions = other.ui.max_transactions;
        }
        // 체인 설정 병합
        for (chain_id, chain_config) in other.chains {
            self.chains.insert(chain_id, chain_config);
        }
        // Watch 설정 병합
        if !other.watch.addresses.is_empty() {
            self.watch = other.watch;
        }
    }

    /// CLI 인자로 설정 오버라이드
    pub fn apply_cli_overrides(&mut self, rpc_url: Option<&str>) {
        if let Some(url) = rpc_url {
            self.rpc.ws_url = url.to_string();
        }
    }

    /// 활성 체인 목록 반환 (CLI --chain 또는 config에서)
    pub fn get_active_chains(&self, cli_chains: &[ChainId]) -> Vec<(ChainId, ChainConfig)> {
        if cli_chains.is_empty() || (cli_chains.len() == 1 && cli_chains[0] == ChainId::Ethereum) {
            // 기본: config에 설정된 체인 또는 Ethereum만
            if self.chains.is_empty() {
                vec![(
                    ChainId::Ethereum,
                    ChainConfig {
                        enabled: true,
                        ws_url: Some(self.rpc.ws_url.clone()),
                        fallback_ws_url: self.rpc.fallback_ws_url.clone(),
                    },
                )]
            } else {
                self.chains
                    .iter()
                    .filter(|(_, c)| c.enabled)
                    .map(|(id, c)| (*id, c.clone()))
                    .collect()
            }
        } else {
            // CLI에서 지정된 체인들
            cli_chains
                .iter()
                .map(|id| {
                    let config = self.chains.get(id).cloned().unwrap_or_default();
                    (*id, config)
                })
                .collect()
        }
    }
}

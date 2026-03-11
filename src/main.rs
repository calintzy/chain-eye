mod app;
mod chain;
mod config;
mod error;
mod event;
mod filter;
mod ui;

use std::path::PathBuf;

use clap::Parser;
use tokio::sync::mpsc;

use crate::app::App;
use crate::chain::provider::chain_provider_task;
use crate::config::AppConfig;
use crate::filter::TxFilter;

#[derive(Parser)]
#[command(name = "chain-eye")]
#[command(version = "0.1.0")]
#[command(about = "Real-time EVM transaction monitoring TUI")]
struct Cli {
    /// WebSocket RPC URL (overrides config file)
    #[arg(long)]
    rpc: Option<String>,

    /// Minimum ETH value filter
    #[arg(long, value_name = "ETH")]
    min_value: Option<f64>,

    /// Filter by from address
    #[arg(long, value_name = "ADDRESS")]
    from: Option<String>,

    /// Filter by to address
    #[arg(long, value_name = "ADDRESS")]
    to: Option<String>,

    /// Config file path (default: ~/.chain-eye/config.toml)
    #[arg(long, short)]
    config: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // 1. 설정 로딩
    let mut config = AppConfig::load(cli.config.as_ref())?;
    config.apply_cli_overrides(cli.rpc.as_deref());

    // 2. 필터 설정
    let filter = TxFilter::from_cli(cli.min_value, cli.from, cli.to);

    // 3. 체인 이벤트 채널
    let (chain_tx, chain_rx) = mpsc::channel(256);

    // 4. 체인 프로바이더 태스크 (백그라운드)
    let provider_config = config.rpc.clone();
    let provider_filter = filter.clone();
    let addr_len = config.ui.address_display_len;
    tokio::spawn(async move {
        chain_provider_task(provider_config, provider_filter, addr_len, chain_tx).await;
    });

    // 5. TUI 앱 실행
    let mut app = App::new(config, filter, chain_rx);
    app.run().await?;

    Ok(())
}

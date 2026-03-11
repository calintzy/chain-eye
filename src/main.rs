mod app;
mod chain;
mod config;
mod error;
mod event;
mod filter;
mod token;
mod ui;

use std::path::PathBuf;

use clap::Parser;
use tokio::sync::mpsc;

use crate::app::App;
use crate::chain::chains::ChainId;
use crate::chain::manager::ChainManager;
use crate::config::AppConfig;
use crate::filter::TxFilter;

#[derive(Parser)]
#[command(name = "chain-eye")]
#[command(version = "0.2.0")]
#[command(about = "Real-time EVM transaction monitoring TUI")]
struct Cli {
    /// WebSocket RPC URL (Ethereum 오버라이드)
    #[arg(long)]
    rpc: Option<String>,

    /// 최소 ETH 값 필터
    #[arg(long, value_name = "ETH")]
    min_value: Option<f64>,

    /// from 주소 필터
    #[arg(long, value_name = "ADDRESS")]
    from: Option<String>,

    /// to 주소 필터
    #[arg(long, value_name = "ADDRESS")]
    to: Option<String>,

    /// 설정 파일 경로
    #[arg(long, short)]
    config: Option<PathBuf>,

    /// 모니터링할 체인 (쉼표 구분: ethereum,polygon,arbitrum)
    #[arg(long, value_name = "CHAINS", default_value = "ethereum")]
    chain: String,

    /// 추적할 지갑 주소 (ENS 지원)
    #[arg(long, value_name = "ADDRESS")]
    watch: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // 1. 설정 로딩
    let mut config = AppConfig::load(cli.config.as_ref())?;
    config.apply_cli_overrides(cli.rpc.as_deref());

    // 2. 체인 목록 결정
    let cli_chains = ChainId::from_csv(&cli.chain);
    let active_chains = config.get_active_chains(&cli_chains);
    let chain_ids: Vec<ChainId> = active_chains.iter().map(|(id, _)| *id).collect();

    // 3. 필터 설정
    let filter = TxFilter::from_cli(cli.min_value, cli.from, cli.to);

    // 4. 체인 이벤트 채널
    let (chain_tx, chain_rx) = mpsc::channel(256);

    // 5. 멀티체인 프로바이더 spawn
    let addr_len = config.ui.address_display_len;
    ChainManager::spawn_providers(&active_chains, &config.rpc, &filter, addr_len, chain_tx);

    // 6. ENS 해석 (--watch에 .eth 주소가 지정된 경우)
    let watch_address = if let Some(ref addr) = cli.watch {
        if chain::ens::is_ens_name(addr) {
            eprintln!("ENS 해석 중: {} ...", addr);
            match chain::ens::resolve_ens(addr).await {
                Some(resolved) => {
                    eprintln!("ENS 해석 완료: {} → {}", addr, resolved);
                    Some(resolved)
                }
                None => {
                    eprintln!("ENS 해석 실패: {} — 주소를 직접 사용합니다", addr);
                    Some(addr.clone())
                }
            }
        } else {
            Some(addr.clone())
        }
    } else {
        None
    };

    // 7. TUI 앱 실행
    let mut app = App::new(config, filter, chain_ids, watch_address, chain_rx);
    app.run().await?;

    Ok(())
}

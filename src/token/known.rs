use std::collections::HashMap;

use crate::chain::chains::ChainId;

/// 토큰 정보
pub struct TokenInfo {
    pub symbol: &'static str,
    pub decimals: u8,
}

/// 주요 토큰 목록 (체인별 하드코딩)
pub fn build_known_tokens() -> HashMap<(ChainId, String), TokenInfo> {
    let mut m = HashMap::new();

    // === Ethereum ===
    let eth_tokens = vec![
        ("0xdac17f958d2ee523a2206206994597c13d831ec7", "USDT", 6),
        ("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48", "USDC", 6),
        ("0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2", "WETH", 18),
        ("0x6b175474e89094c44da98b954eedeac495271d0f", "DAI", 18),
        ("0x2260fac5e5542a773aa44fbcfedf7c193bc2c599", "WBTC", 8),
        ("0x514910771af9ca656af840dff83e8264ecf986ca", "LINK", 18),
        ("0x1f9840a85d5af5bf1d1762f925bdaddc4201f984", "UNI", 18),
        ("0x7d1afa7b718fb893db30a3abc0cfc608aacfebb0", "MATIC", 18),
        ("0x95ad61b0a150d79219dcf64e1e6cc01f0b64c4ce", "SHIB", 18),
    ];
    for (addr, symbol, decimals) in eth_tokens {
        m.insert(
            (ChainId::Ethereum, addr.to_string()),
            TokenInfo { symbol, decimals },
        );
    }

    // === Polygon ===
    let pol_tokens = vec![
        ("0xc2132d05d31c914a87c6611c10748aeb04b58e8f", "USDT", 6),
        ("0x3c499c542cef5e3811e1192ce70d8cc03d5c3359", "USDC", 6),
        ("0x7ceb23fd6bc0add59e62ac25578270cff1b9f619", "WETH", 18),
        ("0x0d500b1d8e8ef31e21c99d1db9a6444d3adf1270", "WPOL", 18),
    ];
    for (addr, symbol, decimals) in pol_tokens {
        m.insert(
            (ChainId::Polygon, addr.to_string()),
            TokenInfo { symbol, decimals },
        );
    }

    // === Arbitrum ===
    let arb_tokens = vec![
        ("0xfd086bc7cd5c481dcc9c85ebe478a1c0b69fcbb9", "USDT", 6),
        ("0xaf88d065e77c8cc2239327c5edb3a432268e5831", "USDC", 6),
        ("0x82af49447d8a07e3bd95bd0d56f35241523fbab1", "WETH", 18),
        ("0x912ce59144191c1204e64559fe8253a0e49e6548", "ARB", 18),
    ];
    for (addr, symbol, decimals) in arb_tokens {
        m.insert(
            (ChainId::Arbitrum, addr.to_string()),
            TokenInfo { symbol, decimals },
        );
    }

    // === Base ===
    let base_tokens = vec![
        ("0x833589fcd6edb6e08f4c7c32d4f71b54bda02913", "USDC", 6),
        ("0x4200000000000000000000000000000000000006", "WETH", 18),
    ];
    for (addr, symbol, decimals) in base_tokens {
        m.insert(
            (ChainId::Base, addr.to_string()),
            TokenInfo { symbol, decimals },
        );
    }

    m
}

/// 토큰 주소로 심볼 조회
pub fn lookup_symbol<'a>(
    known: &'a HashMap<(ChainId, String), TokenInfo>,
    chain_id: ChainId,
    address: &str,
) -> Option<&'a TokenInfo> {
    known.get(&(chain_id, address.to_lowercase()))
}

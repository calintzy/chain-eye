# Chain-Eye

[English](README.md) | [한국어](README.ko.md) | [中文](README.zh.md)

**A terminal TUI for real-time multi-chain EVM transaction monitoring**

```
┌─ Chain-Eye v0.2 ─────────────────────────────────────────────┐
│  1 ETH  2 POL  3 ARB  4 BASE  5 OP  6 AVAX  7 BSC          │
├──────────────────────────────────────────────────────────────┤
│  ETH Mainnet    Block: 21,847,234    Gas: 12.3 Gwei  Connected│
├──────────────────────────────────────────────────────────────┤
│  Time      Hash          From          To        Value  Type  │
│▶ 12:45:03 0xab3f..8e2d 0x7c2a..1f4b 0xdef9.. 2.50 ETH      │
│  12:45:02 0x9b1e..5d8f 0x4a2c..9e1d 0xab3f.. 0     12.5K USDT│
│  12:45:01 0xdef9..3c7a 0x2b1d..4c3e 0x5e6f.. 1.25 ETH      │
│  12:44:59 0x4a2c..9e1d 0x9b1e..5d8f 0x2b1d.. 0     NFT #42  │
├──────────────────────────────────────────────────────────────┤
│  [ETH] TX: 1,234 received | Showing: 1,000 | Filter: OFF     │
├──────────────────────────────────────────────────────────────┤
│  [Tab/1-7] Chain [↑↓] Select [Enter] Detail [f] Filter [q] Quit│
└──────────────────────────────────────────────────────────────┘
```

---

## Why Chain-Eye?

### 1. Terminal ↔ Browser Context Switching
Every time you want to check a transaction while coding or analyzing data, you have to switch to a browser. This breaks your flow and kills productivity.

### 2. No Real-Time Data
Web explorers like Etherscan are static — they require a manual refresh. There is no way to watch a live transaction stream as it happens.

### 3. Single-Chain Limitation
Most tools only support one chain at a time. Switching between Ethereum, Polygon, and Arbitrum requires different tabs and tools.

### The Solution
**Chain-Eye** brings all of this into your terminal:
- **Multi-chain monitoring**: Monitor 7 EVM chains simultaneously with Tab switching
- **Real-time transaction stream**: Instant updates via WebSocket block subscriptions
- **Token transfer decoding**: ERC-20/721 transfers decoded with token symbols
- **Wallet tracking**: Watch specific addresses with `--watch` (ENS supported)
- **Powerful filtering**: Filter by minimum ETH value, sender, or recipient address
- **Auto-reconnect**: Exponential backoff keeps connections stable per chain
- **Terminal-native**: Monitor chain data without ever leaving your development environment

---

## Features

### v0.2 — Multi-Chain & Token Support
- **7 EVM chains**: Ethereum, Polygon, Arbitrum, Base, Optimism, Avalanche, BSC
- **Chain tab switching**: Tab/Shift+Tab to cycle, 1-7 for direct selection
- **Independent connections**: Each chain has its own WebSocket with auto-reconnect
- **ERC-20 token decoding**: Recognizes major tokens (USDT, USDC, WETH, DAI, etc.)
- **ERC-721 NFT detection**: Shows NFT transfer with token ID
- **Wallet tracking**: `--watch <address>` to track specific wallet activity
- **ENS resolution**: `--watch vitalik.eth` resolves ENS names automatically
- **Gas price display**: Shows current base fee in Gwei per chain
- **Chain-aware UI**: Native token symbol (ETH/POL/AVAX/BNB) per chain

### Core Features (v0.1+)
- **Real-time transaction stream**: Receive every transaction live via WebSocket
- **Transaction detail view**: Press Enter to see full details — Hash, From, To, Value, Gas, Token info
- **Filtering**: Filter transactions by minimum value, `from` address, or `to` address
- **Transaction classification**: Transfer, ContractCall, or ContractCreation
- **Block info**: Current block number and connection status
- **Vim-style navigation**: `j`/`k` for up/down, `g`/`G` for top/bottom
- **TOML config file**: `~/.chain-eye/config.toml`
- **Auto-reconnect**: Exponential backoff with configurable retry count

---

## Supported Chains

| Chain | Symbol | Key | Default RPC |
|-------|--------|:---:|-------------|
| Ethereum | ETH | 1 | `wss://eth.drpc.org` |
| Polygon | POL | 2 | `wss://polygon-bor-rpc.publicnode.com` |
| Arbitrum | ARB | 3 | `wss://arbitrum-one-rpc.publicnode.com` |
| Base | BASE | 4 | `wss://base-rpc.publicnode.com` |
| Optimism | OP | 5 | `wss://optimism-rpc.publicnode.com` |
| Avalanche | AVAX | 6 | `wss://avalanche-c-chain-rpc.publicnode.com` |
| BSC | BNB | 7 | `wss://bsc-rpc.publicnode.com` |

All chains use free public RPC endpoints by default. You can override them in the config file.

---

## Installation & Setup

### Prerequisites
- Rust 1.70 or later

### Build
```bash
git clone https://github.com/calintzy/chain-eye.git
cd chain-eye
cargo build --release
```

### Run
```bash
# Ethereum only (default)
./target/release/chain-eye

# Multi-chain monitoring
./target/release/chain-eye --chain ethereum,polygon,arbitrum

# All 7 chains
./target/release/chain-eye --chain ethereum,polygon,arbitrum,base,optimism,avalanche,bsc

# Watch a specific wallet
./target/release/chain-eye --watch 0x1234...5678

# Watch with ENS name
./target/release/chain-eye --watch vitalik.eth

# Combined: multi-chain + wallet + filter
./target/release/chain-eye --chain ethereum,polygon --watch 0x1234...5678 --min-value 1.0
```

---

## Usage

### CLI Options

| Option | Description | Example |
|--------|-------------|---------|
| `--chain` | Chains to monitor (comma-separated) | `--chain ethereum,polygon,arbitrum` |
| `--watch` | Wallet address to track (ENS supported) | `--watch vitalik.eth` |
| `--rpc` | Custom WebSocket RPC URL (Ethereum override) | `--rpc wss://your-rpc` |
| `--min-value` | Minimum native token value filter | `--min-value 1.0` |
| `--from` | Filter by sender address | `--from 0x1234...` |
| `--to` | Filter by recipient address | `--to 0x5678...` |
| `--config` | Custom config file path | `--config ./my-config.toml` |

---

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| **Tab** | Switch to next chain |
| **Shift+Tab** | Switch to previous chain |
| **1-7** | Select chain directly |
| **↑** or **k** | Select previous transaction |
| **↓** or **j** | Select next transaction |
| **Enter** | View transaction details |
| **Esc** | Close detail view |
| **w** | Toggle watch mode (requires `--watch`) |
| **f** | Toggle filter on/off |
| **g** | Jump to top of list |
| **G** | Jump to bottom of list |
| **q** | Quit |

---

## Configuration File

Save settings to `~/.chain-eye/config.toml`:

```toml
[rpc]
ws_url = "wss://eth.drpc.org"
fallback_ws_url = "wss://ethereum-rpc.publicnode.com"
max_reconnect_attempts = 10
reconnect_base_delay_ms = 1000

[ui]
max_transactions = 1000
tick_rate_ms = 16
address_display_len = 4

# Per-chain RPC overrides
[chains.ethereum]
enabled = true
ws_url = "wss://eth.drpc.org"
fallback_ws_url = "wss://ethereum-rpc.publicnode.com"

[chains.polygon]
enabled = true
ws_url = "wss://polygon-bor-rpc.publicnode.com"

[chains.arbitrum]
enabled = true

# Wallet tracking
[watch]
addresses = ["0x1234567890123456789012345678901234567890"]
```

---

## Tech Stack

- **Language**: Rust (2021 edition)
- **TUI framework**: ratatui 0.29
- **Terminal backend**: crossterm 0.28
- **EVM RPC client**: alloy 1.5+ (provider-ws, rpc-types, sol-types)
- **Async runtime**: tokio 1.x
- **CLI parser**: clap 4
- **Config**: serde + toml 0.8
- **Hex encoding**: hex 0.4
- **Time handling**: chrono 0.4
- **Error handling**: thiserror 2, anyhow 1
- **Async utilities**: futures-util 0.3

---

## Project Structure

```
chain-eye/
├── src/
│   ├── main.rs              # Entry point, CLI parsing, ENS resolution
│   ├── app.rs               # Main event loop, multi-chain state management
│   ├── config.rs            # TOML config with per-chain settings
│   ├── error.rs             # Custom error types
│   ├── event.rs             # Chain event definitions (with ChainId routing)
│   ├── filter.rs            # Transaction filtering logic
│   ├── chain/
│   │   ├── mod.rs           # Chain module entry
│   │   ├── chains.rs        # ChainId enum, metadata (7 EVM chains)
│   │   ├── manager.rs       # Multi-chain provider spawner
│   │   ├── provider.rs      # WebSocket provider, block subscription, token decoding
│   │   ├── ens.rs           # ENS name resolution (Ethereum Mainnet)
│   │   └── types.rs         # TxInfo, BlockInfo, TxType, TokenTransfer
│   ├── token/
│   │   ├── mod.rs           # Token module entry
│   │   ├── decode.rs        # ERC-20/721 transfer decoding (4-byte selectors)
│   │   └── known.rs         # Known token database (USDT, USDC, WETH, etc.)
│   └── ui/
│       ├── mod.rs           # UI module entry
│       ├── tabs.rs          # Chain tab bar widget
│       ├── header.rs        # Header (chain name, block, gas, connection)
│       ├── tx_list.rs       # Transaction list with token display
│       ├── tx_detail.rs     # Transaction detail with token/chain info
│       ├── watch.rs         # Wallet tracking panel
│       ├── status_bar.rs    # Status bar with chain name
│       ├── help.rs          # Help bar with Tab/watch shortcuts
│       └── layout.rs        # Layout with chain tabs area
├── Cargo.toml
├── Cargo.lock
└── README.md
```

---

## How It Works

### 1. Initialization
1. Parse CLI options (`--chain`, `--watch`, `--rpc`, filters)
2. Load `~/.chain-eye/config.toml` if it exists
3. Determine active chains from CLI or config
4. Resolve ENS names if `--watch` uses `.eth` address

### 2. Multi-Chain Connection
1. `ChainManager` spawns independent `tokio::spawn` tasks per chain
2. Each task connects via WebSocket (primary + fallback URL)
3. Connections are fully independent — one chain failing doesn't affect others
4. Auto-reconnect with exponential backoff per chain

### 3. Real-Time Streaming
1. Subscribe to block headers per chain
2. Fetch full block with transactions on each new block
3. Decode ERC-20/721 token transfers from transaction input data
4. Match known tokens (USDT, USDC, WETH, etc.) for symbol display
5. Apply filters and emit `ChainEvent` with `ChainId` routing

### 4. UI Rendering
1. Chain tabs show all active chains with connection status indicators
2. Only the selected chain's transactions are displayed
3. Token transfers show symbol and amount instead of raw type
4. Detail view includes chain name, token transfer info, and gas data

---

## Token Recognition

Chain-Eye recognizes major tokens across all supported chains:

| Token | Ethereum | Polygon | Arbitrum | Base |
|-------|:--------:|:-------:|:--------:|:----:|
| USDT | ✅ | ✅ | ✅ | - |
| USDC | ✅ | ✅ | ✅ | ✅ |
| WETH | ✅ | ✅ | ✅ | ✅ |
| DAI | ✅ | - | - | - |
| WBTC | ✅ | - | - | - |
| LINK | ✅ | - | - | - |
| UNI | ✅ | - | - | - |

Unknown tokens display as `???` with 18 decimals assumed.

---

## Reconnection Mechanism

Each chain reconnects independently using exponential backoff:

```
Attempt 1: wait 1s
Attempt 2: wait 2s
Attempt 3: wait 4s
...
Max delay: 30s
Max attempts: configurable (default: 10)
```

Connection status per chain is shown in the tab bar:
- `●` Connected
- `◌` Reconnecting
- `○` Disconnected

---

## Troubleshooting

### RPC Connection Failure
- Use `--rpc` to specify a different endpoint
- Check per-chain `ws_url` in config
- Verify your network connection

### No Transactions Appearing
- Check filters (toggle with `f`)
- Verify connection status in tab bar (● = connected)
- Some chains have lower throughput — wait for new blocks

### High CPU Usage
- Increase `tick_rate_ms` (16 → 50)
- Decrease `max_transactions` (1000 → 500)
- Monitor fewer chains

---

## License

MIT License

---

## Contributing

Contributions are welcome. Feel free to open a Pull Request.

---

## Roadmap

- **v0.1** ✅: Ethereum Mainnet real-time stream
- **v0.2** ✅: Multi-chain support, token decoding, wallet tracking, ENS
- **v0.3**: Watch mode UI, balance tracking, transaction direction indicators
- **v1.0**: Contract event decoding, real-time price integration, custom alerts

---

## Contact

For questions or issues, please open a GitHub Issue.

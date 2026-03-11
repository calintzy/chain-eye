# Chain-Eye

[English](README.md) | [한국어](README.ko.md) | [中文](README.zh.md)

**A terminal TUI for real-time EVM blockchain transaction monitoring**

```
┌─ Chain-Eye v0.1 ─────────────────────────────────────────────┐
│  ETH Mainnet  Block: 21,847,234  Connected  🟢               │
├──────────────────────────────────────────────────────────────┤
│  Time      Hash          From          To           Value    │
│▶ 12:45:03 0xab3f..8e2d 0x7c2a..1f4b 0xdef9..3c7a 2.50 ETH  │
│  12:45:02 0x9b1e..5d8f 0x4a2c..9e1d 0xab3f..8e2d 0.00 ETH  │
│  12:45:01 0xdef9..3c7a 0x2b1d..4c3e 0x5e6f..7a8b 1.25 ETH  │
│  12:44:59 0x4a2c..9e1d 0x9b1e..5d8f 0x2b1d..4c3e 0.50 ETH  │
├──────────────────────────────────────────────────────────────┤
│  TX: 1,234 received | Filter: OFF                            │
├──────────────────────────────────────────────────────────────┤
│  [↑↓] Select  [Enter] Detail  [F] Filter  [Q] Quit           │
└──────────────────────────────────────────────────────────────┘
```

---

## Why Chain-Eye?

Modern blockchain developers, traders, and security researchers face a common set of friction points:

### 1. Terminal ↔ Browser Context Switching
Every time you want to check a transaction while coding or analyzing data, you have to switch to a browser. This breaks your flow and kills productivity.

### 2. No Real-Time Data
Web explorers like Etherscan are static — they require a manual refresh. There is no way to watch a live transaction stream as it happens.

### 3. Poor Filtering and Monitoring
There are no tools that let you filter transactions in real time by specific wallets, addresses, or minimum transfer amounts.

### The Solution
**Chain-Eye** brings all of this into your terminal:
- **Real-time transaction stream**: Instant updates via WebSocket block subscriptions
- **Powerful filtering**: Filter by minimum ETH value, sender address, or recipient address
- **Quick detail view**: Press Enter to inspect any transaction immediately
- **Auto-reconnect**: Exponential backoff keeps the connection stable
- **Terminal-native**: Monitor chain data without ever leaving your development environment

---

## Features

### Core Features
- **Real-time transaction stream**: Receive every transaction on ETH Mainnet live via WebSocket
- **Transaction detail view**: Press Enter to see full details — Hash, From, To, Value, Gas, and more
- **Filtering**: Filter transactions by minimum ETH value, `from` address, or `to` address
- **Transaction classification**: Categorizes transactions as Transfer, ContractCall, or ContractCreation
- **Block info**: Displays the current block number and connection status
- **Keyboard navigation**:
  - Vim-style `j`/`k` for up/down movement
  - `g` to jump to the top, `G` to jump to the bottom
- **TOML config file**: Manage RPC endpoints and UI settings via `~/.chain-eye/config.toml`
- **CLI options**: Configure at runtime with `--rpc`, `--min-value`, `--from`, `--to`, and `--config`
- **Auto-reconnect**: Automatically recovers from dropped connections using exponential backoff

---

## Installation & Setup

### Prerequisites
- Rust 1.70 or later

### Build
```bash
git clone https://github.com/yourusername/chain-eye.git
cd chain-eye
cargo build --release
```

### Run
```bash
# Run with defaults (Ethereum Mainnet, free public RPC)
./target/release/chain-eye

# Or run directly with Cargo
cargo run --release
```

---

## Usage

### Basic Usage
```bash
chain-eye
```
Monitors Ethereum Mainnet transactions in real time using the free public RPC `wss://eth.drpc.org`.

### CLI Options

#### Custom RPC endpoint
```bash
chain-eye --rpc wss://your-custom-rpc-url
```

#### Minimum ETH value filter
```bash
chain-eye --min-value 1.0
```
Shows only transactions worth 1 ETH or more.

#### Filter by sender address
```bash
chain-eye --from 0x1234567890123456789012345678901234567890
```
Shows only transactions sent from the specified address.

#### Filter by recipient address
```bash
chain-eye --to 0xdac17f958d2ee523a2206206994597c13d831ec7
```
Shows only transactions sent to the specified address.

#### Combined filters
```bash
chain-eye --min-value 10.0 --to 0xdac17f958d2ee523a2206206994597c13d831ec7
```
Shows only transactions of 10 ETH or more going to a specific address.

#### Custom config file path
```bash
chain-eye --config /path/to/config.toml
```

---

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| **↑** or **k** | Select previous transaction |
| **↓** or **j** | Select next transaction |
| **Enter** | View transaction details |
| **Esc** | Close detail view |
| **f** | Toggle filter on/off |
| **g** | Jump to top of list |
| **G** | Jump to bottom of list |
| **q** | Quit the application |

---

## Configuration File

Saving settings to `~/.chain-eye/config.toml` makes them the default every time you run Chain-Eye.

### Example Config
```toml
[rpc]
# Primary RPC URL (default: wss://eth.drpc.org)
ws_url = "wss://eth.drpc.org"

# Fallback RPC URL (used if primary fails)
fallback_ws_url = "wss://ethereum-rpc.publicnode.com"

# Maximum number of reconnect attempts (default: 10)
max_reconnect_attempts = 10

# Base delay between reconnect attempts in milliseconds (default: 1000)
reconnect_base_delay_ms = 1000

[ui]
# Maximum number of transactions to keep in memory (default: 1000)
max_transactions = 1000

# UI rendering tick rate in milliseconds (default: 16 ≈ 60fps)
tick_rate_ms = 16

# Number of characters to display per address, excluding 0x prefix (default: 4 → 0xab3f)
address_display_len = 4
```

### Defaults
- **ws_url**: `wss://eth.drpc.org`
- **fallback_ws_url**: `wss://ethereum-rpc.publicnode.com`
- **max_reconnect_attempts**: 10
- **reconnect_base_delay_ms**: 1000
- **max_transactions**: 1000
- **tick_rate_ms**: 16
- **address_display_len**: 4

---

## Tech Stack

- **Language**: Rust (2021 edition)
- **TUI framework**: ratatui 0.29
- **Terminal backend**: crossterm 0.28
- **EVM RPC client**: alloy 1.5 (successor to ethers-rs)
- **Async runtime**: tokio 1.x
- **CLI parser**: clap 4
- **Config management**: serde + toml 0.8
- **Serialization**: serde_json 1
- **Time handling**: chrono 0.4
- **Home directory**: dirs 6
- **Error handling**: thiserror 2, anyhow 1
- **Async utilities**: futures-util 0.3

---

## Project Structure

```
chain-eye/
├── src/
│   ├── main.rs              # Entry point, CLI parsing, initialization
│   ├── app.rs               # Main application loop, event handling
│   ├── config.rs            # TOML config loading and defaults
│   ├── error.rs             # Custom error types
│   ├── event.rs             # Chain event definitions (blocks, transactions)
│   ├── filter.rs            # Transaction filtering logic
│   ├── chain/
│   │   ├── mod.rs           # Chain module entry point
│   │   ├── provider.rs      # WebSocket provider, block subscription
│   │   └── types.rs         # TxInfo, BlockInfo, TxType definitions
│   └── ui/
│       ├── mod.rs           # UI module entry point
│       ├── header.rs        # Header widget (block number, connection status)
│       ├── tx_list.rs       # Transaction list table
│       ├── tx_detail.rs     # Transaction detail popup
│       ├── status_bar.rs    # Status bar (stats, filter state)
│       ├── help.rs          # Help section
│       └── layout.rs        # Layout composition
├── config/
│   └── default.toml         # Example default config file
├── Cargo.toml               # Project metadata and dependencies
├── Cargo.lock               # Dependency version lock
└── README.md                # This file
```

---

## How It Works

### 1. Initialization
1. Parse CLI options
2. Load `~/.chain-eye/config.toml` if it exists
3. Override config values with any CLI flags provided
4. Construct `TxFilter`

### 2. WebSocket Connection
1. Attempt to connect to the primary RPC (`ws_url`)
2. If that fails, fall back to the fallback RPC
3. Maintain the WebSocket connection once established
4. On failure, retry with exponential backoff

### 3. Real-Time Streaming
1. Subscribe to block headers via WebSocket (`subscribe("newHeads")`)
2. For each new block, call `eth_getBlockByNumber` to fetch the transaction list
3. For each transaction:
   - Parse `from`, `to`, `value`, and `data` fields
   - Classify `TxType` (Transfer, ContractCall, ContractCreation)
   - Convert `value` to ETH
4. Emit `ChainEvent::NewTransaction`

### 4. UI Rendering
1. Re-render the screen every frame (default: every 16ms)
2. Display the transaction list
3. Highlight the currently selected item
4. Show active filter status

### 5. Event Handling
- **Key events**: Handle up/down navigation, Enter, Esc, f, g, G, q, etc.
- **Chain events**: Update transaction list and block info on new data

---

## Transaction Type Classification

| Type | Condition | Example |
|------|-----------|---------|
| **Transfer** | `to` is set and no `input` data | Simple ETH transfer |
| **ContractCall** | `to` is set and `input` data is present | Uniswap swap, token transfer |
| **ContractCreation** | `to` is null | Deploying a new contract |

---

## Reconnection Mechanism

If the WebSocket connection drops, Chain-Eye automatically reconnects using exponential backoff:

```
Attempt 1: wait 1s
Attempt 2: wait 2s
Attempt 3: wait 4s
Attempt 4: wait 8s
...
Up to: max_reconnect_attempts
```

Once reconnected, block subscriptions resume automatically.

---

## Troubleshooting

### RPC Connection Failure
- **Symptom**: `Disconnected: Connection failed` message
- **Fix**:
  - Use `--rpc` to specify a different RPC endpoint
  - Check `fallback_ws_url` in `~/.chain-eye/config.toml`
  - Verify your network connection

### No Transactions Appearing
- **Symptom**: The list is empty
- **Fix**:
  - Check whether filters are too restrictive (toggle with `f`)
  - Verify the connection status icon in the header (🟢 connected / 🔴 disconnected)
  - Confirm that blocks are being received

### High CPU Usage
- **Symptom**: Fan noise, high CPU utilization
- **Fix**:
  - Increase `tick_rate_ms` (default 16 → 50)
  - Decrease `max_transactions` (default 1000 → 500)

---

## License

MIT License

---

## Contributing

Contributions are welcome. Feel free to open a Pull Request.

---

## Roadmap

- **v0.1** (current): Ethereum Mainnet real-time stream
- **v0.2**: Multi-chain support (Polygon, Arbitrum, Base, etc.)
- **v0.3**: Advanced filtering, wallet tracking mode
- **v1.0**: Contract event decoding, real-time price integration

---

## Contact

For questions or issues, please open a GitHub Issue.

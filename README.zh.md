[English](README.md) | [한국어](README.ko.md) | [中文](README.zh.md)

# Chain-Eye

**在终端实时监控 EVM 区块链交易的 TUI 工具**

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

## 为什么需要 Chain-Eye？

现代区块链开发者、交易者和安全研究人员面临以下痛点：

### 1. 终端 ↔ 浏览器频繁切换
在编码或数据分析时，每次查看交易都需要打开浏览器，这会打断开发流程，降低工作效率。

### 2. 缺乏实时数据
Etherscan 等网页浏览器是静态的，需要手动刷新，无法查看实时的交易流。

### 3. 过滤与监控困难
目前没有工具能够按特定钱包地址、目标地址或最小交易额对交易进行实时过滤。

### 解决方案
**Chain-Eye** 在终端 TUI 中提供以下能力：
- **实时交易流**：通过 WebSocket 订阅区块，即时获取更新
- **强大的过滤**：按最小 ETH 金额、发送/接收地址过滤交易
- **快速详情查看**：按 Enter 键即可查看交易详细信息
- **自动重连**：使用指数退避（Exponential Backoff）保持稳定连接
- **终端优先**：无需离开开发环境即可监控链上数据

---

## 功能

### 核心功能
- **实时交易流**：通过 WebSocket 实时接收 ETH 主网的所有交易
- **交易详情查看**：按 Enter 键显示交易详细信息（Hash、From、To、Value、Gas 等）
- **过滤**：按最小 ETH 金额、from 地址、to 地址过滤交易
- **交易分类**：将交易区分为 Transfer、ContractCall、ContractCreation 三种类型
- **区块信息**：显示当前区块号与连接状态
- **键盘导航**：
  - 支持 Vim 风格的 j/k 键
  - g 跳到列表顶部，G 跳到列表底部
- **TOML 配置文件**：通过 `~/.chain-eye/config.toml` 管理 RPC 和 UI 设置
- **CLI 选项**：支持 --rpc、--min-value、--from、--to、--config 运行时配置
- **自动重连**：连接断开时使用指数退避自动恢复

---

## 安装与运行

### 前置要求
- Rust 1.70 及以上版本

### 构建
```bash
git clone https://github.com/yourusername/chain-eye.git
cd chain-eye
cargo build --release
```

### 运行
```bash
# 默认运行（Ethereum 主网，免费公共 RPC）
./target/release/chain-eye

# 或通过 cargo 直接运行
cargo run --release
```

---

## 使用方法

### 基本运行
```bash
chain-eye
```
使用免费公共 RPC `wss://eth.drpc.org` 监控 Ethereum 主网的实时交易。

### CLI 选项

#### 指定自定义 RPC
```bash
chain-eye --rpc wss://your-custom-rpc-url
```

#### 最小 ETH 金额过滤
```bash
chain-eye --min-value 1.0
```
仅显示 1 ETH 及以上的交易。

#### 按发送地址过滤
```bash
chain-eye --from 0x1234567890123456789012345678901234567890
```
仅显示从指定地址发出的交易。

#### 按接收地址过滤
```bash
chain-eye --to 0xdac17f958d2ee523a2206206994597c13d831ec7
```
仅显示发送到指定地址的交易。

#### 组合过滤
```bash
chain-eye --min-value 10.0 --to 0xdac17f958d2ee523a2206206994597c13d831ec7
```
仅显示 10 ETH 及以上且发送到指定地址的交易。

#### 指定配置文件
```bash
chain-eye --config /path/to/config.toml
```

---

## 键盘快捷键

| 按键 | 操作 |
|------|------|
| **↑** 或 **k** | 选择上一条交易 |
| **↓** 或 **j** | 选择下一条交易 |
| **Enter** | 查看选中交易的详细信息 |
| **Esc** | 关闭详情视图 |
| **f** | 切换过滤器开启/关闭 |
| **g** | 跳到列表顶部 |
| **G** | 跳到列表底部 |
| **q** | 退出程序 |

---

## 配置文件

将配置保存到 `~/.chain-eye/config.toml` 后，该文件将作为默认配置使用。

### 配置示例
```toml
[rpc]
# 主 RPC URL（默认值：wss://eth.drpc.org）
ws_url = "wss://eth.drpc.org"

# 备用 RPC URL（主 RPC 失败时使用）
fallback_ws_url = "wss://ethereum-rpc.publicnode.com"

# 最大重连尝试次数（默认值：10）
max_reconnect_attempts = 10

# 重连基础延迟时间（毫秒，默认值：1000）
reconnect_base_delay_ms = 1000

[ui]
# 内存中保留的最大交易数量（默认值：1000）
max_transactions = 1000

# UI 渲染帧率（毫秒，默认值：16 ≈ 60fps）
tick_rate_ms = 16

# 地址显示长度（含 0x 前缀，默认值：4 → 0xab3f）
address_display_len = 4
```

### 默认值
- **ws_url**: `wss://eth.drpc.org`
- **fallback_ws_url**: `wss://ethereum-rpc.publicnode.com`
- **max_reconnect_attempts**: 10
- **reconnect_base_delay_ms**: 1000
- **max_transactions**: 1000
- **tick_rate_ms**: 16
- **address_display_len**: 4

---

## 技术栈

- **语言**: Rust（2021 edition）
- **TUI 框架**: ratatui 0.29
- **终端后端**: crossterm 0.28
- **EVM RPC 客户端**: alloy 1.5（ethers-rs 的继任者）
- **异步运行时**: tokio 1.x
- **CLI 解析器**: clap 4
- **配置管理**: serde + toml 0.8
- **序列化**: serde_json 1
- **时间处理**: chrono 0.4
- **主目录**: dirs 6
- **错误处理**: thiserror 2、anyhow 1
- **异步工具**: futures-util 0.3

---

## 项目结构

```
chain-eye/
├── src/
│   ├── main.rs              # 入口点，CLI 解析，初始化
│   ├── app.rs               # 主应用循环，事件处理
│   ├── config.rs            # TOML 配置加载与默认值
│   ├── error.rs             # 自定义错误类型
│   ├── event.rs             # 链事件定义（区块、交易）
│   ├── filter.rs            # 交易过滤逻辑
│   ├── chain/
│   │   ├── mod.rs           # 链模块入口
│   │   ├── provider.rs      # WebSocket 提供者，区块订阅
│   │   └── types.rs         # TxInfo、BlockInfo、TxType 定义
│   └── ui/
│       ├── mod.rs           # UI 模块入口
│       ├── header.rs        # 头部组件（区块号、连接状态）
│       ├── tx_list.rs       # 交易列表表格
│       ├── tx_detail.rs     # 交易详情弹窗
│       ├── status_bar.rs    # 状态栏（统计信息、过滤状态）
│       ├── help.rs          # 帮助信息
│       └── layout.rs        # 布局组合
├── config/
│   └── default.toml         # 默认配置文件示例
├── Cargo.toml               # 项目元数据及依赖
├── Cargo.lock               # 依赖版本锁定
└── README.md                # 说明文档
```

---

## 工作原理

### 1. 初始化
1. 解析 CLI 选项
2. 加载 `~/.chain-eye/config.toml`（如存在）
3. 用 CLI 选项覆盖配置
4. 创建 TxFilter

### 2. WebSocket 连接
1. 尝试连接主 RPC URL（`ws_url`）
2. 失败时尝试备用 RPC
3. 成功后维持 WebSocket 连接
4. 失败时使用指数退避进行重连

### 3. 实时数据流
1. 通过 WebSocket 订阅区块头（`subscribe("newHeads")`）
2. 每个区块调用 `eth_getBlockByNumber` 获取交易列表
3. 对每笔交易：
   - 解析 `from`、`to`、`value`、`data`
   - 分类 `TxType`（Transfer、ContractCall、ContractCreation）
   - 将 `value` 转换为 ETH
4. 发出 `ChainEvent::NewTransaction` 事件

### 4. UI 渲染
1. 每帧（默认 16ms）重新渲染屏幕
2. 显示交易列表
3. 高亮选中项
4. 显示过滤状态

### 5. 事件处理
- **键盘事件**：处理上下移动、Enter、Esc、f、g、G、q 等按键
- **链事件**：新交易到达、区块信息更新

---

## 交易类型分类

| 类型 | 条件 | 示例 |
|------|------|------|
| **Transfer** | 有 `to` 地址且无 `input` 数据 | 普通 ETH 转账 |
| **ContractCall** | 有 `to` 地址且有 `input` 数据 | Uniswap 换币、代币转账 |
| **ContractCreation** | 无 `to` 地址（null） | 部署新合约 |

---

## 重连机制

WebSocket 连接断开时，将使用指数退避自动重连。

```
第 1 次尝试：等待 1 秒
第 2 次尝试：等待 2 秒
第 3 次尝试：等待 4 秒
第 4 次尝试：等待 8 秒
...
最大尝试次数：max_reconnect_attempts
```

重连成功后，将自动重新开始订阅区块。

---

## 故障排除

### RPC 连接失败
- **症状**：显示 `Disconnected: Connection failed` 消息
- **解决方案**：
  - 使用 `--rpc` 选项指定其他 RPC 服务器
  - 检查 `~/.chain-eye/config.toml` 中的 `fallback_ws_url`
  - 确认网络连接正常

### 交易列表为空
- **症状**：列表没有任何内容
- **解决方案**：
  - 检查过滤条件是否过于严格（按 f 键切换过滤器）
  - 确认 RPC 连接状态（查看头部的连接状态图标 🟢/🔴）
  - 确认是否正在接收区块

### CPU 占用过高
- **症状**：风扇噪音大，CPU 使用率高
- **解决方案**：
  - 增大 `tick_rate_ms`（默认 16 → 50）
  - 减小 `max_transactions`（默认 1000 → 500）

---

## 许可证

MIT License

---

## 贡献

欢迎贡献代码，请发送 Pull Request。

---

## 路线图

- **v0.1**（当前）：Ethereum 主网实时数据流
- **v0.2**：多链支持（Polygon、Arbitrum、Base 等）
- **v0.3**：高级过滤、钱包追踪模式
- **v1.0**：合约事件解码、实时价格集成

---

## 联系方式

如有问题或疑问，请在 GitHub Issues 中提交。

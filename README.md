# Solana Price Monitor & Arbitrage Detector (v1)

[![Rust Version](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![Solana](https://img.shields.io/badge/solana-blockchain-purple.svg)](https://solana.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A real-time, lightweight Rust CLI tool that monitors decentralized exchanges (DEXes) on the Solana blockchain and detects risk-free arbitrage opportunities. Using the official Jupiter Routing API, **Version 1 (v1)** monitors prices on **Raydium** and **Orca**, computes precise net profitability after accounting for transaction fees, priority fees, and price impact penalties, and logs opportunities for algorithmic analysis.

---

## 🚀 Overview

Arbitrage on high-frequency chains like Solana requires low-latency data and realistic profit modeling. This tool polls price quotes for specified token pairs (default: `SOL`/`USDC`) across two of Solana's largest DEXes via the Jupiter Swap API.

### Key Features
* **Real-Time Price Polling**: Automatically fetches concurrent quotes every 10 seconds.
* **Granular Route Isolation**: Targets and isolates liquidity routes from Raydium vs. Orca+V2 to compute exact price differentials.
* **Realistic Net Profit Formula**: Subtracts price impact penalties, base transaction fees (for 2 swap legs), and custom priority fees to prevent false positives.
* **Persistent Data Logging**: Appends and persists historical trading data to `opportunities.csv` for backend backtesting and analysis.

---

## 📊 Core Architecture & Profit Logic

### 1. Data Flow & Execution Loop

```
   +---------------------------------------------+
   |                Tokio Loop                   |
   |              (Every 10 Sec)                 |
   +----------------------+----------------------+
                          |
                          v
   +----------------------+----------------------+
   |             Jupiter Swap API                |
   |   (Query Quote for Raydium & Orca+V2)       |
   +----------------------+----------------------+
                          |
                          v
   +----------------------+----------------------+
   |        Price Parsing & Normalization        |
   |        (Normalize Out-Amounts & Impact)     |
   +----------------------+----------------------+
                          |
                          v
   +----------------------+----------------------+
   |           Net Profit Calculation            |
   | (Accounting for Slippage, Tx & Priority Fees)|
   +----------------------+----------------------+
               /                           \
              /                             \
             v                               v
   +--------------------+          +--------------------+
   | Terminal Console   |          |  opportunities.csv |
   |  (Live Monitor)    |          |    (Data Logger)   |
   +--------------------+          +--------------------+
```

### 2. Profit Calculation Formula

Many arbitrage bots suffer from "ghost profits"—theoretical gains that vanish when subjected to on-chain realities. Our calculator in `src/price/calculate_profit.rs` ensures realism:

$$\text{Net Profit (USDC)} = \text{Gross Difference} - \text{Price Impact Cost} - \text{Base Tx Fees} - \text{Priority Fee}$$

Where:
* **Gross Difference**: The absolute difference in output amounts received between Raydium and Orca.
  $$\text{Gross Difference} = | \text{Raydium Out} - \text{Orca Out} |$$
* **Price Impact Cost**: Penalty based on the combined liquidity pool slippage:
  $$\text{Price Impact Cost} = (|\text{Raydium Impact \%}| + |\text{Orca Impact \%}|) \times \text{Raydium Out}$$
* **Base Tx Fees**: Represents the network base fee for executing both legs of the swap.
  $$\text{Base Tx Fees} = 2 \times 0.000005 \text{ SOL} \times \text{SOL Price (USDC)}$$
* **Priority Fee**: Estimated priority fee (default: `0.0005 SOL`) converted to USDC to ensure swift, competitive block inclusion.

---

## 🛠️ Installation & Setup

### Prerequisites
* **Rust**: Ensure you have the latest Rust toolchain installed (edition 2024). [Install Rust](https://www.rust-lang.org/tools/install).
* **Jupiter API Key**: You will need a valid Jupiter API Key. You can obtain one from the [Jupiter Developer Portal](https://station.jup.ag/docs/apis/swap-api).

### Step-by-Step Setup

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/yourusername/solana-priceMonitor-arbitragedetect.git
   cd solana-priceMonitor-arbitragedetect
   ```

2. **Configure Environment Variables**:
   Copy the example environment file:
   ```bash
   cp .env.example .env
   ```
   Open the `.env` file and insert your Jupiter API Key:
   ```env
   JUP_API_KEY=your_actual_jupiter_api_key
   ```

3. **Build the Application**:
   Compile the release version for maximum performance:
   ```bash
   cargo build --release
   ```

---

## 💻 Usage

To run the price monitor:
```bash
cargo run --release
```

### Console Output Example
The CLI prints real-time opportunities directly to your stdout:
```text
[2026-08-13T14:32:05.123Z] Raydium: 145.2300 | Orca: 145.1800 | Net profit: -0.024500 | Profitable: false
[2026-08-13T14:32:15.456Z] Raydium: 145.2200 | Orca: 145.2400 | Net profit: -0.012300 | Profitable: false
```

### Historical Data Logging (`opportunities.csv`)
All polled opportunities are written to `opportunities.csv` in the root directory. If the file does not exist, it is initialized with the following structure:

| Header | Description |
| :--- | :--- |
| `timestamp` | ISO 8601 Timestamp of the poll event |
| `raydium_price` | Normalized USDC output amount from Raydium |
| `orca_price` | Normalized USDC output amount from Orca |
| `raydium_impact` | Price impact percentage on Raydium |
| `orca_impact` | Price impact percentage on Orca |
| `net_profit` | Computed profit/loss in USDC after fees & slippage |
| `is_profitable` | Boolean indicating if the opportunity was net-profitable (`> 0.0`) |

---

## 🗺️ v2 Roadmap

This v1 release provides a strong framework for monitoring and simulation. Future releases will aim to support the following:
- [ ] **Dynamic Token Monitoring**: Support passing tokens via CLI arguments or configuration files (e.g., `SOL/USDC`, `BONK/SOL`, `JUP/USDC`).
- [ ] **On-Chain Arbitrage Execution**: Direct execution of profitable routes using Anchor / Solana SDK with custom smart contracts.
- [ ] **Flash Loans Integration**: Support for flash loans to eliminate capital constraints.
- [ ] **Dynamic Priority Fee Optimization**: Integrating Helius or Jupiter Priority Fee API to adjust fees dynamically based on current cluster congestion.
- [ ] **Multi-DEX Extension**: Expanding beyond Raydium and Orca to include Phoenix, Meteora, and Lifinity.

---

## ⚖️ Disclaimer

*This software is for educational and research purposes only. Cryptocurrency trading, especially high-frequency arbitrage, carries significant financial risks including but not limited to network congestion, execution slippage, smart contract vulnerabilities, and capital loss. Do not run this software with real capital without exhaustive testing and auditing.*

---

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information (if applicable).

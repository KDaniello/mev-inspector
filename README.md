# 🥪 MEV Inspector: Real-Time Sandwich Detector

A full-stack Maximal Extractable Value (MEV) monitoring system. It detects **Sandwich Attacks** on Uniswap V2/V3 in real-time by analyzing transaction ordering in Ethereum blocks.

## 🏗️ Architecture

The project consists of two high-performance components:

1.  **Rust Collector (Backend):**
    *   Connects to Ethereum via WebSocket (Alloy).
    *   Downloads full blocks (including transactions).
    *   Applies heuristic algorithms to detect `Buy(Bot) -> Buy(Victim) -> Sell(Bot)` patterns.
    *   Logs detected attacks to a JSONL stream.

2.  **Python Dashboard (Frontend):**
    *   Reads the data stream in real-time.
    *   Visualizes attack statistics, top bot addresses, and victim losses using **Streamlit**.

## 🚀 Key Features

*   **Deep Block Analysis:** This tool inspects the *internal ordering* of transactions within a block to find MEV opportunities.
*   **Hybrid Stack:** Leverages Rust for low-latency ingestion and Python for data science/visualization.
*   **Multi-Router Support:** Tracks attacks on both Uniswap V2 Router and Universal Router (V3).

## 🛠️ Tech Stack

*   **Rust:** Alloy (RPC), Tokio (Async), Serde (JSON), Tracing.
*   **Python:** Streamlit, Pandas.

## 📦 Setup & Run

### 1. Configure
Create a `.env` file:
```ini
RPC_URL=wss://mainnet.infura.io/ws/v3/YOUR_API_KEY
OUTPUT_FILE=mev_data.jsonl
```

### 2. Run Backend (Rust)
This will start scanning blocks and writing to mev_data.jsonl.

```powershell
cargo run --release
```

### 3. Run Frontend (Python)
```powershell
pip install streamlit pandas
streamlit run dashboard.py
```

## 📜 License
MIT License.
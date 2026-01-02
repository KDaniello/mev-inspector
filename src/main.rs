use alloy::{
    primitives::Address, 
    providers::{Provider, ProviderBuilder, WsConnect}, 
    rpc::types::{Block, BlockNumberOrTag, BlockTransactions},
    consensus::Transaction,
};
use eyre::Result;
use futures_util::StreamExt;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::str::FromStr;
use tracing::{warn, error, info};

const UNISWAP_V2_ROUTER: &str = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D";
const UNIVERSAL_ROUTER: &str = "0x3fC91A3afd70395Cd496C647d5a6CC9D4B2b7FAD";

#[derive(serde::Serialize)]
struct MevEvent {
    block_number: u64,
    bot_address: String,
    victim_address: String,
    tx_front: String,
    tx_victim: String,
    tx_back: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
    dotenv::dotenv().ok();

    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
    let json_path = env::var("OUTPUT_FILE").unwrap_or_else(|_| "mev_data.jsonl".to_string());

    info!("🕵️‍♂️ MEV Inspector Started");
    info!("📂 Output: {}", json_path);

    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().connect_ws(ws).await?;

    info!("✅ Connected! Waiting for new blocks...");

    let sub = provider.subscribe_blocks().await?;
    let mut stream = sub.into_stream();

    while let Some(header) = stream.next().await {
        let block_number = header.number;
        let tag = BlockNumberOrTag::Number(block_number);

        match provider.get_block_by_number(tag).full().await {
            Ok(Some(block)) => {
                info!("📦 Analyzing Block #{} ({} txs)", block_number, block.transactions.len());
                process_block(block, &json_path)?;
            }
            Ok(None) => warn!("⚠️ Block #{} not found yet", block_number),
            Err(e) => error!("❌ Failed to get block: {:?}", e),
        }
    }

    Ok(())
}

fn process_block(block: Block, file_path: &str) -> Result<()> {
    let txs = match block.transactions {
        BlockTransactions::Full(t) => t,
        _ => return Ok(()),
    };

    let v2_router = Address::from_str(UNISWAP_V2_ROUTER).unwrap();
    let universal_router = Address::from_str(UNIVERSAL_ROUTER).unwrap();

    let mut swap_txs: Vec<(usize, String, String)> = Vec::new();

    for (i, tx) in txs.iter().enumerate() {

        let to_addr = tx.inner.to();

        if let Some(to) = to_addr {
            if to == v2_router || to == universal_router {
                let from_addr = tx.inner.signer();
                let tx_hash = tx.inner.hash();

                swap_txs.push((i, from_addr.to_string(), tx_hash.to_string()));
            }
        }
    }

    if swap_txs.len() < 3 {
        return Ok(());
    }

    for i in 0..swap_txs.len() - 2 {
        let (idx1, bot1, hash1) = &swap_txs[i];
        let (idx2, victim, hash2) = &swap_txs[i+1];
        let (idx3, bot2, hash3) = &swap_txs[i+2];

        if bot1 == bot2 {
            if bot1 != victim {
                if (idx3 - idx1) <= 3 {

                    info!("🥪 SANDWICH DETECTED in Block {}!", block.header.number);
                    info!("   🤖 Bot: {}", bot1);
                    info!("   💀 Victim: {}", victim);

                    let event = MevEvent {
                        block_number: block.header.number,
                        bot_address: bot1.clone(),
                        victim_address: victim.clone(),
                        tx_front: hash1.clone(),
                        tx_victim: hash2.clone(),
                        tx_back: hash3.clone(),
                    };

                    let mut file = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(file_path)?;

                    writeln!(file, "{}", serde_json::to_string(&event)?)?;
                }
            }
        }
    }

    Ok(())
}
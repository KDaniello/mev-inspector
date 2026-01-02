import streamlit as st
import pandas as pd
import json
import time

st.set_page_config(
    page_title="MEV Inspector",
    page_icon="🥪",
    layout="wide"
)

st.title("🥪 Real-Time MEV Sandwich Detector")
st.markdown("Monitoring Ethereum Mainnet for Sandwich Attacks...")

FILE_PATH = "mev_data.jsonl"

def load_data():
    data = []
    try:
        with open(FILE_PATH,"r") as f:
            for line in f:
                try:
                    data.append(json.loads(line))
                except json.JSONDecodeError:
                    continue
    except FileNotFoundError:
        return pd.DataFrame()
    
    if not data:
        return pd.DataFrame()
    
    df = pd.DataFrame(data)
    df = df.sort_values(by="block_number", ascending=False)
    return df

placeholder = st.empty()

while True:
    df = load_data()
    
    with placeholder.container():
        col1, col2, col3 = st.columns(3)
        
        total_attacks = len(df) if not df.empty else 0
        unique_bots = df['bot_address'].nunique() if not df.empty else 0
        last_block = df['block_number'].max() if not df.empty else "Waiting..."
        
        col1.metric("Total Attacks Detected", total_attacks)
        col2.metric("Unique Bots Active", unique_bots)
        col3.metric("Last Attack Block", last_block)
        
        if not df.empty:
            st.subheader("🕵️‍♂️ Latest Attacks")
            
            display_df = df[['block_number', 'bot_address', 'victim_address', 'tx_front']]
            st.dataframe(display_df, use_container_width=True)
            
            st.subheader("🤖 Top Sandwich Bots")
            bot_counts = df['bot_address'].value_counts().head(5)
            st.bar_chart(bot_counts)
        
        else:
            st.info("📡 Scanning blocks... No sandwiches detected yet. (Wait a few minutes)")
    
    time.sleep(2)
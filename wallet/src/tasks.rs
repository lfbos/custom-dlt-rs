use crate::core::Core;
use crate::ui::run_ui;
use crate::util::big_mode_btc;
use btclib::config::BlockchainConfig;
use btclib::types::Transaction;
use cursive::views::TextContent;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tokio::time::{self, Duration};
use tracing::*;

pub async fn update_utxos(core: Arc<Core>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let config = BlockchainConfig::global();
        let mut interval =
            time::interval(Duration::from_secs(config.wallet.utxo_update_interval_secs));
        loop {
            interval.tick().await;
            if let Err(e) = core.fetch_utxos().await {
                error!("Failed to update UTXOs: {}", e);
            }
        }
    })
}

pub async fn handle_transactions(
    rx: kanal::AsyncReceiver<Transaction>,
    core: Arc<Core>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        while let Ok(transaction) = rx.recv().await {
            if let Err(e) = core.send_transaction(transaction).await {
                error!("Failed to send transaction: {}", e);
            }
        }
    })
}

pub async fn ui_task(core: Arc<Core>, balance_content: TextContent) -> JoinHandle<()> {
    tokio::task::spawn_blocking(move || {
        info!("Running UI");
        if let Err(e) = run_ui(core, balance_content) {
            error!("UI ended with error: {e}");
        };
    })
}

pub async fn update_balance(core: Arc<Core>, balance_content: TextContent) -> JoinHandle<()> {
    tokio::spawn(async move {
        let config = BlockchainConfig::global();
        loop {
            tokio::time::sleep(Duration::from_millis(
                config.wallet.balance_display_update_interval_ms,
            ))
            .await;
            info!("updating balance string");
            balance_content.set_content(big_mode_btc(&core));
        }
    })
}

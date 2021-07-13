use std::time::Duration;
use ethers::prelude::{Ws, Provider, Address, StreamExt};
use std::str::FromStr;
use std::sync::Arc;

pub mod token_contract;

type AnyError = std::result::Result<(), Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> AnyError {
    tokio::task::spawn(async move {
        let ws = Ws::connect("wss://mainnet.infura.io/ws/v3/[API-KEY]").await.unwrap();
        let provider = Arc::new(Provider::new(ws).interval(Duration::from_millis(500)));
        let address = Address::from_str("0xdac17f958d2ee523a2206206994597c13d831ec7").unwrap();
        let contract = token_contract::TokenContract::new(address, provider);

        let filter = contract.transfer_filter();
        let mut stream = filter.stream().await.unwrap();

        while let Some(block) = stream.next().await {
            dbg!(block);
        }
    }).await;

    Ok(())
}

use crate::{nft_721, nft_market, EthersResult};
use ethers::contract::Contract;
use ethers::prelude::*;
use prelude::*;
use std::env;
use std::sync::Arc;

pub async fn watch_nft_transfer_event() -> EthersResult<()> {
    let nft = nft_721::client::Client::new(Network::Polygon);
    let market = nft_market::client::Client::new(Network::Polygon);

    let provider = Arc::new(
        Provider::<Ws>::connect(
            env::var("POLYGON_WS_URL")
                .expect("POLYGON_WS_URL must be set")
                .as_str(),
        )
        .await?,
    );
    let contract = Contract::new(nft.address, nft.abi, provider);

    let event = contract.event_for_name::<TransferEvent>("Transfer")?;
    let mut stream = event.stream().await?;
    println!("{}", "waiting transfer event");
    while let Some(event) = stream.next().await {
        match event {
            Ok(event) => {
                println!(
                    "transfer: from: {}, to: {}, token_id: {}",
                    event.from, event.to, event.token_id
                );

                let key = format!("{:?}#{}", nft.address, event.token_id);
                println!("target: {}", key);

                let order_keys = market.get_sell_order_keys().await?;
                for key in order_keys.clone() {
                    println!("key: {}", key);
                }

                if order_keys.contains(&key) {
                    market
                        .cancel_order_by_admin(
                            format!("{:?}", nft.address),
                            event.token_id.as_u128(),
                        )
                        .await?;
                }
            }
            Err(e) => {
                eprintln!("error while processing event: {:?}", e);
            }
        }
    }

    Ok(())
}

#[derive(EthEvent)]
#[ethevent(abi = "Transfer(address,address,uint256)")]
pub struct TransferEvent {
    #[ethevent(indexed, name = "from")]
    pub from: Address,
    #[ethevent(indexed, name = "to")]
    pub to: Address,
    #[ethevent(indexed, name = "tokenId")]
    pub token_id: U256,
}

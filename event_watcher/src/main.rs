use dotenv::dotenv;

#[tokio::main]
pub async fn main() {
    dotenv().ok();

    if let Err(e) = impl_ethers_rs::event::watch_nft_transfer_event().await {
        println!("error: {:?}", e);
        return;
    }
}

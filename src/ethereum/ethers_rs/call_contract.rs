use std::env;

#[derive(Clone, Debug)]
pub struct Client {
    wallet_address: String,
    wallet_secret: String,
    contract_address: String,
}

impl Client {
    pub fn new(contract_address: String) -> Self {
        let base_url = env::var("ETHEREUM_URL").expect("ETHEREUM_URL must be set");

        let wallet_address = env::var("WALLET_ADDRESS").expect("WALLET_ADDRESS must be set");
        let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");

        Client {
            wallet_address,
            wallet_secret,
            contract_address,
        }
    }
}

use crate::{query_contract, Error, EthersResult};
use ethers::abi::{ethabi, Abi};
use ethers::prelude::transaction::eip712::{EIP712Domain, Eip712DomainType, TypedData, Types};
use ethers::prelude::*;
use ethers::utils::hex;
use prelude::*;
use serde_json::Number;
use std::collections::BTreeMap;
use std::env;

#[derive(Clone, Debug)]
pub struct Client {
    user_wallet_address: Address,
    user_wallet_secret: String,
    relayer_wallet_secret: String,
    mtw_address: Address,
    nft_address: Address,
    mtw_abi: Abi,
    nft_abi: Abi,
    network: Network,
}

impl Client {
    pub fn new(network: Network) -> Self {
        let user_wallet_address = env::var("WALLET_ADDRESS").expect("WALLET_ADDRESS must be set");
        let user_wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");
        let relayer_wallet_secret = env::var("RELAYER_SECRET").expect("RELAYER_SECRET must be set");

        Client {
            user_wallet_address: user_wallet_address.parse::<Address>().unwrap(),
            user_wallet_secret,
            relayer_wallet_secret,
            mtw_address: network
                .meta_transaction_wallet_address()
                .parse::<Address>()
                .unwrap(),
            nft_address: network
                .meta_transactional_nft_721_address()
                .parse::<Address>()
                .unwrap(),
            mtw_abi: serde_json::from_str(include_str!("mtw_abi.json").trim()).unwrap(),
            nft_abi: serde_json::from_str(include_str!("nft_abi.json").trim()).unwrap(),
            network,
        }
    }

    pub async fn get_nonce(&self) -> EthersResult<U256> {
        let res = query_contract(
            self.mtw_address.to_owned(),
            self.mtw_abi.to_owned(),
            self.network.to_owned(),
        )
        .method::<_, U256>("getNonce", self.user_wallet_address.clone())?
        .call()
        .await?;
        Ok(res)
    }

    pub async fn mint(&self, to: Address, hash: String) -> EthersResult<()> {
        let provider = Provider::<Http>::try_from(self.network.chain_url().clone()).unwrap();

        let user_wallet = self
            .user_wallet_secret
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(self.network.chain_id());

        let relayer_wallet = self
            .relayer_wallet_secret
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(self.network.chain_id());

        let nonce = self.get_nonce().await?;

        let nft_function = self.nft_abi.function("mint").unwrap().clone();
        let encoded_nft_function = nft_function
            .encode_input(&[
                ethabi::Token::Address(to.into()),
                ethabi::Token::String(hash),
            ])
            .unwrap();
        let data = hex::encode(encoded_nft_function);

        let mut types: Types = BTreeMap::new();
        types.insert(
            "EIP712Domain".to_string(),
            vec![
                Eip712DomainType {
                    name: "name".to_string(),
                    r#type: "string".to_string(),
                },
                Eip712DomainType {
                    name: "version".to_string(),
                    r#type: "string".to_string(),
                },
                Eip712DomainType {
                    name: "chainId".to_string(),
                    r#type: "uint256".to_string(),
                },
                Eip712DomainType {
                    name: "verifyingContract".to_string(),
                    r#type: "address".to_string(),
                },
            ],
        );

        types.insert(
            "ForwardRequest".to_string(),
            vec![
                Eip712DomainType {
                    name: "from".to_string(),
                    r#type: "address".to_string(),
                },
                Eip712DomainType {
                    name: "to".to_string(),
                    r#type: "address".to_string(),
                },
                Eip712DomainType {
                    name: "value".to_string(),
                    r#type: "uint256".to_string(),
                },
                Eip712DomainType {
                    name: "gas".to_string(),
                    r#type: "uint256".to_string(),
                },
                Eip712DomainType {
                    name: "nonce".to_string(),
                    r#type: "uint256".to_string(),
                },
                Eip712DomainType {
                    name: "data".to_string(),
                    r#type: "bytes".to_string(),
                },
            ],
        );

        let mut message: BTreeMap<String, serde_json::Value> = BTreeMap::new();
        message.insert(
            "from".to_string(),
            serde_json::Value::String(format!("{:?}", self.user_wallet_address.clone())),
        );
        message.insert(
            "to".to_string(),
            serde_json::Value::String(format!("{:?}", self.nft_address.clone())),
        );
        message.insert(
            "value".to_string(),
            serde_json::Value::Number(Number::from(0)),
        );
        message.insert(
            "gas".to_string(),
            serde_json::Value::Number(Number::from(GAS_LIMIT)),
        );
        message.insert(
            "nonce".to_string(),
            serde_json::Value::Number(Number::from(nonce.low_u128())),
        );
        message.insert("data".to_string(), serde_json::Value::String(data));

        let typed_data: TypedData = TypedData {
            domain: EIP712Domain {
                name: Some("MetaTransactionWallet".to_string()),
                version: Some("0.0.1".to_string()),
                chain_id: Some(U256::from(self.network.chain_id().clone())),
                verifying_contract: Some(self.mtw_address),
                salt: None,
            },
            types,
            primary_type: "ForwardRequest".to_string(),
            message: message.clone(),
        };

        let signature = user_wallet
            .sign_typed_data::<TypedData>(&typed_data)
            .await?;

        let forward_request = vec![
            ethabi::Token::Address(
                message
                    .get("from")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .parse()
                    .unwrap(),
            ),
            ethabi::Token::Address(
                message
                    .get("to")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .parse()
                    .unwrap(),
            ),
            ethabi::Token::Uint(message.get("value").unwrap().as_u64().unwrap().into()),
            ethabi::Token::Uint(message.get("gas").unwrap().as_u64().unwrap().into()),
            ethabi::Token::Uint(message.get("nonce").unwrap().as_u64().unwrap().into()),
            ethabi::Token::Bytes(
                hex::decode(message.get("data").unwrap().as_str().unwrap()).unwrap(),
            ),
        ];

        let mtw_function = self.mtw_abi.function("execute").unwrap().clone();
        let encoded_mtw_function = mtw_function
            .encode_input(&[
                ethabi::Token::Tuple(forward_request),
                ethabi::Token::Bytes(signature.to_vec()),
            ])
            .unwrap();

        let tx = TransactionRequest::new()
            .to(self.mtw_address.to_owned())
            .data(encoded_mtw_function);
        let client = SignerMiddleware::new_with_provider_chain(provider, relayer_wallet)
            .await
            .unwrap();
        let pending_tx = client
            .send_transaction(tx, None)
            .await
            .map_err(|_e| Error::Internal("error".to_string()))?;

        let receipt = pending_tx.await?.unwrap();
        client
            .get_transaction(receipt.transaction_hash.clone())
            .await
            .map_err(|_e| Error::Internal("error".to_string()))?;

        println!("{:?}", receipt);

        Ok(())
    }
}

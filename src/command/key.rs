use crate::error::CliResult;
use ethers::prelude::Signature;
use ethers_signers::{LocalWallet, Signer};
use secp256k1::rand::thread_rng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use std::env;
use std::str::FromStr;

pub async fn generate() -> CliResult<()> {
    let secp = Secp256k1::new();
    let mut rng = thread_rng();
    let seckey: SecretKey = SecretKey::new(&mut rng);
    let pubkey = PublicKey::from_secret_key(&secp, &seckey);
    let seckey = seckey.display_secret().to_string();
    let wallet = seckey.parse::<LocalWallet>()?;

    println!("secret: {}", seckey);
    println!("pubkey: {}", pubkey.to_string());
    println!("address {:?}", wallet.address());

    Ok(())
}

pub async fn sign(message: String) -> CliResult<()> {
    let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");
    let wallet = wallet_secret.parse::<LocalWallet>()?;

    let signature = wallet.sign_message(message).await?;

    println!("result: {}", signature.to_string());

    Ok(())
}

pub async fn verify(signature: String, message: String) -> CliResult<()> {
    let wallet_secret = env::var("WALLET_SECRET").expect("WALLET_SECRET must be set");
    let wallet = wallet_secret.parse::<LocalWallet>()?;

    let sig = Signature::from_str(&signature).unwrap();

    match sig.verify(message, wallet.address()) {
        Ok(_) => println!("verified by {:?}!", wallet.address()),
        Err(_) => println!("cannot verified by {:?}!", wallet.address()),
    }

    Ok(())
}

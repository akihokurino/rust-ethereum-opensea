use crate::error::CliResult;
use ethers::core::k256::elliptic_curve::sec1::ToEncodedPoint;
use ethers::prelude::Signature;
use ethers_signers::{LocalWallet, Signer};
use std::env;
use std::str::FromStr;

#[allow(unused)]
pub async fn generate_by_ethers_rs() -> CliResult<()> {
    let seckey =
        ethers::core::k256::elliptic_curve::SecretKey::<ethers::core::k256::Secp256k1>::random(
            &mut rand::thread_rng(),
        );
    let seckey_str = ethers::utils::hex::encode(seckey.to_be_bytes().as_slice());
    let pubkey = seckey.public_key();
    let pubkey_encoded = pubkey.to_encoded_point(false);
    let pubkey_str = ethers::utils::hex::encode(pubkey_encoded.as_bytes());
    let address = ethers::core::types::Address::from_slice(
        &ethers::utils::keccak256(&pubkey_encoded.as_bytes()[1..])[12..],
    );
    let address_str = format!("{:?}", address);

    println!("secret: {}", seckey_str);
    println!("pubkey: {}", pubkey_str);
    println!("address {}", address_str);

    Ok(())
}

#[allow(unused)]
pub async fn generate_by_secp256k1() -> CliResult<()> {
    let secp = secp256k1::Secp256k1::new();
    let seckey = secp256k1::SecretKey::new(&mut secp256k1::rand::thread_rng());
    let seckey_str = seckey.display_secret().to_string();
    let pubkey = secp256k1::PublicKey::from_secret_key(&secp, &seckey);
    let pubkey_str = pubkey.to_string();
    let wallet = seckey_str.parse::<LocalWallet>()?;
    let address = wallet.address();
    let address_str = format!("{:?}", address);

    println!("secret: {}", seckey_str);
    println!("pubkey: {}", pubkey_str);
    println!("address {}", address_str);

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

use crate::error::CliResult;
use secp256k1::rand::thread_rng;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

pub async fn generate() -> CliResult<()> {
    let secp = Secp256k1::new();
    let mut rng = thread_rng();
    let seckey: SecretKey = SecretKey::new(&mut rng);
    let pubkey = PublicKey::from_secret_key(&secp, &seckey);

    println!("secret: {}", seckey.display_secret().to_string());
    println!("pubkey: {}", pubkey.to_string());

    Ok(())
}

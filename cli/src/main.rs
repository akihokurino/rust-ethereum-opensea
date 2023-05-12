extern crate core;

use clap::{arg, Parser, ValueEnum};
use dotenv::dotenv;
use prelude::*;
use std::env;
use std::str::FromStr;

#[derive(ValueEnum, Clone, Debug)]
enum Command {
    Balance,
    SendEth,
    CreateMetadata,
    Mint,
    Transfer,
    Info,
    KeyGen,
    Sign,
    Verify,
    Deploy,
    UpdateTime,
    NftMarketSell,
    NftMarketCancel,
    NftMarketBuy,
    ApproveForSell,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Package {
    EthersRs,
    RustWeb3,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Contract {
    Nft721,
    Nft1155,
    Sbt721,
    RevealNft721,
    NftMarket,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    #[clap(value_enum)]
    command: Command,

    #[arg(long, default_value = "nft")]
    name: String,

    #[arg(long, default_value = "nft market sample")]
    description: String,

    #[arg(long, default_value = "sample.png")]
    image_filename: String,

    #[arg(
        long,
        default_value = "https://placehold.jp/3d4070/ffffff/500x500.png?text=Reveal"
    )]
    image_url: String,

    #[arg(long, default_value_t = 10)]
    amount: u128,

    #[arg(long, default_value = "QmPDE4pXnFvNtqJ2889HgEQUEft8KCdyMaKKt5zzw3NuMS")]
    content_hash: String,

    #[arg(long, default_value = "ethers-rs")]
    #[clap(value_enum)]
    package: Package,

    #[arg(long, default_value = "Polygon")]
    network: String,

    #[arg(long, default_value = "nft721")]
    #[clap(value_enum)]
    contract: Contract,

    #[arg(long, default_value_t = 0.1)]
    ether: f64,

    #[arg(long, default_value = "0x0E91D6613a84d7C8b72a289D8b275AF7717C3d2E")]
    to_address: String,

    #[arg(long, default_value_t = 1)]
    token_id: u128,

    #[arg(long, default_value = "world")]
    message: String,

    #[arg(
        long,
        default_value = "2a30afb5d5b476a505422d931c5b98a10d6ac6b6fb3a56a27c658a9fa36911f10b079fe392893e684881813e7d07a3fd14048ba902c20eb56eb9f0e7f8c2a1131b"
    )]
    signature: String,
}

#[tokio::main]
pub async fn main() {
    dotenv().ok();

    if let Err(e) = execute(Args::parse()).await {
        println!("error: {:?}", e);
        return;
    }
}

async fn execute(args: Args) -> CliResult<()> {
    let network = Network::from_str(&args.network).unwrap();
    let to_address = impl_ethers_rs::to_address(args.to_address.clone());

    match args.command {
        Command::Balance => match args.package {
            Package::EthersRs => impl_ethers_rs::get_balance(network)
                .await
                .map_err(Error::from),
            Package::RustWeb3 => impl_rust_web3::get_balance(network)
                .await
                .map_err(Error::from),
        },
        Command::SendEth => match args.package {
            Package::EthersRs => impl_ethers_rs::send_eth(network, args.ether, args.to_address)
                .await
                .map_err(Error::from),
            Package::RustWeb3 => impl_rust_web3::send_eth(network, args.ether, args.to_address)
                .await
                .map_err(Error::from),
        },
        Command::CreateMetadata => {
            if !args.image_url.is_empty() {
                ipfs::create_metadata_from_url(args.name, args.description, args.image_url)
                    .await
                    .map_err(Error::from)
            } else {
                ipfs::create_metadata_from_file(args.name, args.description, args.image_filename)
                    .await
                    .map_err(Error::from)
            }
        }
        Command::Mint => match args.package {
            Package::EthersRs => match args.contract {
                Contract::Nft721 => {
                    let cli = impl_ethers_rs::nft_721::client::Client::new(network);
                    cli.mint(args.content_hash.clone())
                        .await
                        .map_err(Error::from)
                }
                Contract::Nft1155 => {
                    let cli = impl_ethers_rs::nft_1155::client::Client::new(network);
                    cli.mint(args.content_hash.clone(), args.amount)
                        .await
                        .map_err(Error::from)
                }
                Contract::Sbt721 => {
                    let cli = impl_ethers_rs::sbt_721::client::Client::new(network);
                    cli.mint(args.content_hash.clone())
                        .await
                        .map_err(Error::from)
                }
                Contract::RevealNft721 => {
                    let cli = impl_ethers_rs::reveal_nft_721::client::Client::new(network);
                    cli.mint(args.content_hash.clone())
                        .await
                        .map_err(Error::from)
                }
                _ => return Err(Error::Internal("invalid params".to_string())),
            },
            Package::RustWeb3 => match args.contract {
                Contract::Nft721 => {
                    let cli = impl_rust_web3::nft_721::client::Client::new(network);
                    cli.mint(args.content_hash.clone())
                        .await
                        .map_err(Error::from)
                }
                Contract::Nft1155 => {
                    let cli = impl_rust_web3::nft_1155::client::Client::new(network);
                    cli.mint(args.content_hash.clone(), args.amount)
                        .await
                        .map_err(Error::from)
                }
                _ => return Err(Error::Internal("invalid params".to_string())),
            },
        },
        Command::Transfer => match args.package {
            Package::EthersRs => match args.contract {
                Contract::Nft721 => {
                    let cli = impl_ethers_rs::nft_721::client::Client::new(network);
                    cli.transfer(to_address, args.token_id)
                        .await
                        .map_err(Error::from)
                }
                Contract::Nft1155 => {
                    let cli = impl_ethers_rs::nft_1155::client::Client::new(network);
                    cli.transfer(to_address, args.token_id)
                        .await
                        .map_err(Error::from)
                }
                Contract::RevealNft721 => {
                    let cli = impl_ethers_rs::reveal_nft_721::client::Client::new(network);
                    cli.transfer(to_address, args.token_id)
                        .await
                        .map_err(Error::from)
                }
                _ => return Err(Error::Internal("invalid params".to_string())),
            },
            Package::RustWeb3 => match args.contract {
                Contract::Nft721 => {
                    let cli = impl_rust_web3::nft_721::client::Client::new(network);
                    cli.transfer(
                        impl_rust_web3::parse_address(args.to_address).unwrap(),
                        args.token_id,
                    )
                    .await
                    .map_err(Error::from)
                }
                Contract::Nft1155 => {
                    let cli = impl_rust_web3::nft_1155::client::Client::new(network);
                    cli.transfer(
                        impl_rust_web3::parse_address(args.to_address).unwrap(),
                        args.token_id,
                    )
                    .await
                    .map_err(Error::from)
                }
                _ => return Err(Error::Internal("invalid params".to_string())),
            },
        },
        Command::Info => match args.package {
            Package::EthersRs => match args.contract {
                Contract::Nft721 => {
                    let cli = impl_ethers_rs::nft_721::client::Client::new(network);
                    println!("------------------------------------------------------------");
                    println!("Nft721 info: {}", network.nft_721_address());
                    println!("name = {}", cli.name().await?);
                    println!("latestTokenId = {}", cli.latest_token_id().await?);
                    println!("totalSupply = {:?}", cli.total_supply().await?);
                    println!("totalOwned = {:?}", cli.total_owned().await?);
                    println!("------------------------------------------------------------");
                    Ok(())
                }
                Contract::Nft1155 => {
                    let cli = impl_ethers_rs::nft_1155::client::Client::new(network);
                    println!("------------------------------------------------------------");
                    println!("Nft1155 info: {}", network.nft_1155_address());
                    println!("name = {}", cli.name().await?);
                    println!("latestTokenId = {}", cli.latest_token_id().await?);
                    println!("totalSupply = {:?}", cli.total_supply().await?);
                    println!("totalOwned = {:?}", cli.total_owned().await?);
                    println!("------------------------------------------------------------");
                    Ok(())
                }
                Contract::Sbt721 => {
                    let cli = impl_ethers_rs::sbt_721::client::Client::new(network);
                    println!("------------------------------------------------------------");
                    println!("Sbt721 info: {}", network.sbt_721_address());
                    println!("name = {}", cli.name().await?);
                    println!("totalSupply = {:?}", cli.total_supply().await?);
                    println!("------------------------------------------------------------");
                    Ok(())
                }
                Contract::RevealNft721 => {
                    let cli = impl_ethers_rs::reveal_nft_721::client::Client::new(network);
                    println!("------------------------------------------------------------");
                    println!("RevealNft721 info: {}", network.reveal_nft_address());
                    println!("name = {}", cli.name().await?);
                    println!("totalSupply = {:?}", cli.total_supply().await?);
                    println!("getCurrentHour = {}", cli.get_current_hour().await?);
                    println!("------------------------------------------------------------");
                    Ok(())
                }
                Contract::NftMarket => {
                    let market = impl_ethers_rs::nft_market::client::Client::new(Network::Polygon);
                    let keys = market.get_sell_order_keys().await.map_err(Error::from)?;
                    for key in keys {
                        println!("key: {:?}", key);
                    }
                    let items = market.get_all_sell_order().await.map_err(Error::from)?;
                    for item in items {
                        println!("{:?}", item);
                    }
                    Ok(())
                }
            },
            Package::RustWeb3 => match args.contract {
                Contract::Nft721 => {
                    let cli = impl_rust_web3::nft_721::client::Client::new(network);
                    println!("------------------------------------------------------------");
                    println!("Nft721 info: {}", network.nft_721_address());
                    println!("name = {}", cli.name().await?);
                    println!("latestTokenId = {}", cli.latest_token_id().await?);
                    println!("totalSupply = {:?}", cli.total_supply().await?);
                    println!("totalOwned = {:?}", cli.total_owned().await?);
                    println!("------------------------------------------------------------");
                    Ok(())
                }
                Contract::Nft1155 => {
                    let cli = impl_rust_web3::nft_1155::client::Client::new(network);
                    println!("------------------------------------------------------------");
                    println!("Nft1155 info: {}", network.nft_1155_address());
                    println!("name = {}", cli.name().await?);
                    println!("latestTokenId = {}", cli.latest_token_id().await?);
                    println!("totalSupply = {:?}", cli.total_supply().await?);
                    println!("totalOwned = {:?}", cli.total_owned().await?);
                    println!("------------------------------------------------------------");
                    Ok(())
                }
                _ => return Err(Error::Internal("invalid params".to_string())),
            },
        },
        Command::KeyGen => impl_ethers_rs::generate_keys().await.map_err(Error::from),
        Command::Sign => impl_ethers_rs::sign(args.message)
            .await
            .map_err(Error::from),
        Command::Verify => impl_ethers_rs::verify(args.signature, args.message)
            .await
            .map_err(Error::from),
        Command::Deploy => match args.package {
            Package::EthersRs => match args.contract {
                Contract::Nft721 => {
                    let cli = impl_ethers_rs::nft_721::client::Client::new(network);
                    cli.deploy().await.map_err(Error::from)
                }
                Contract::Nft1155 => {
                    let cli = impl_ethers_rs::nft_1155::client::Client::new(network);
                    cli.deploy().await.map_err(Error::from)
                }
                Contract::Sbt721 => {
                    let cli = impl_ethers_rs::sbt_721::client::Client::new(network);
                    cli.deploy().await.map_err(Error::from)
                }
                Contract::RevealNft721 => {
                    let cli = impl_ethers_rs::reveal_nft_721::client::Client::new(network);
                    cli.deploy().await.map_err(Error::from)
                }
                _ => return Err(Error::Internal("invalid params".to_string())),
            },
            Package::RustWeb3 => match args.contract {
                Contract::Nft721 => {
                    let cli = impl_rust_web3::nft_721::client::Client::new(network);
                    cli.deploy().await.map_err(Error::from)
                }
                Contract::Nft1155 => {
                    let cli = impl_rust_web3::nft_1155::client::Client::new(network);
                    cli.deploy().await.map_err(Error::from)
                }
                _ => return Err(Error::Internal("invalid params".to_string())),
            },
        },
        Command::UpdateTime => {
            if network == Network::Ethereum {
                let cli = impl_ethers_rs::reveal_nft_721::client::Client::new(network);
                cli.update_time().await?;
            }

            Ok(())
        }
        Command::NftMarketSell => {
            let market = impl_ethers_rs::nft_market::client::Client::new(Network::Polygon);
            market
                .sell_order(
                    env::var("SELLER_SECRET").expect("SELLER_SECRET must be set"),
                    Network::Polygon.nft_721_address(),
                    args.token_id,
                    args.ether,
                )
                .await
                .map_err(Error::from)
        }
        Command::NftMarketCancel => {
            let market = impl_ethers_rs::nft_market::client::Client::new(Network::Polygon);
            market
                .cancel_order(
                    env::var("SELLER_SECRET").expect("SELLER_SECRET must be set"),
                    Network::Polygon.nft_721_address(),
                    args.token_id,
                )
                .await
                .map_err(Error::from)
        }
        Command::NftMarketBuy => {
            let market = impl_ethers_rs::nft_market::client::Client::new(Network::Polygon);
            market
                .buy_order(
                    env::var("BUYER_SECRET").expect("BUYER_SECRET must be set"),
                    Network::Polygon.nft_721_address(),
                    args.token_id,
                    args.ether,
                )
                .await
                .map_err(Error::from)
        }
        Command::ApproveForSell => match args.contract {
            Contract::Nft721 => {
                let cli = impl_ethers_rs::nft_721::client::Client::new(network);
                cli.set_approval_for_all().await.map_err(Error::from)
            }
            Contract::Nft1155 => {
                let cli = impl_ethers_rs::nft_1155::client::Client::new(network);
                cli.set_approval_for_all().await.map_err(Error::from)
            }
            Contract::RevealNft721 => {
                let cli = impl_ethers_rs::reveal_nft_721::client::Client::new(network);
                cli.set_approval_for_all().await.map_err(Error::from)
            }
            _ => return Err(Error::Internal("invalid params".to_string())),
        },
    }
}

pub type CliResult<T> = Result<T, Error>;

#[derive(thiserror::Error, Debug, PartialOrd, PartialEq, Clone)]
pub enum Error {
    #[error("internal error: {0}")]
    Internal(String),
}

impl From<impl_ethers_rs::Error> for Error {
    fn from(e: impl_ethers_rs::Error) -> Self {
        let msg = format!("ethers-rs error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<impl_rust_web3::Error> for Error {
    fn from(e: impl_rust_web3::Error) -> Self {
        let msg = format!("rust-web3 error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<ipfs::Error> for Error {
    fn from(e: ipfs::Error) -> Self {
        let msg = format!("ipfs error: {:?}", e);
        Self::Internal(msg)
    }
}

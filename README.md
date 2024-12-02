# Rust Ethereum Solidity

## Used, Dependency

- rust v1.68.2
- ethers-rs ( https://github.com/gakonst/ethers-rs )
- rust-secp256k1 ( https://github.com/rust-bitcoin/rust-secp256k1 )
- infura ( https://infura.io/ )
- solidity v0.8.17
- hardhat ( https://hardhat.org/ )
- open zeppelin ( https://openzeppelin.com/ )
- chainlink ( https://docs.chain.link/ )
- oracle

## Support Network
- Ethereum ( Goerli )
- Polygon ( Mumbai )
- Avalanche ( Fuji )

## Contracts

### Nft721

Sample nft of erc721.

### Nft1155

Sample nft of erc1155.

### Sbt721

Sample sbt of erc721.

### UpgradeableNft721

Sample upgradeable nft of erc721.

### RevealNft721 + Oracle

Sample reveal nft. 
Use chainlink and oracle to update reveal time.

### NftMarket

Sample nft marketplace like open sea.
You can deploy an ERC721 contract to mint NFTs and register the contract with the market contract. On the market side, by using the setApprovalForAll function, it receives permission to transfer the NFTs, and automatically transfers the NFTs according to the purchase process. The Ether sent by the buyer will be transferred to the seller.

By detecting the Transfer event, we automatically cancel the sale order when a token with an active sell order is transferred outside the market.

### MultiSigWallet

Sample multi sig as contract wallet.

### MetaTransactionWallet

Sample meta transaction as contract wallet.

## ChainLink Notes

- https://zenn.dev/aki030402/articles/546c38c8b437f1

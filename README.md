# Rust Ethereum CLI

## Specs

- rust v1.68.2
- ethers-rs ( https://github.com/gakonst/ethers-rs )
- rust-web3 ( https://github.com/tomusdrw/rust-web3 )
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

## ChainLink Notes

- https://zenn.dev/aki030402/articles/546c38c8b437f1

## Command

### get balance

```
make balance NETWORK=mumbai PACKAGE=EthersRs
```

### send ether

```
make send-eth ETHER=10 TO_ADDRESS=0x00 NETWORK=Polygon PACKAGE=EthersRs
```

### show token info

```
make token-info NETWORK=Polygon PACKAGE=EthersRs CONTRACT=RustToken721
```

### create metadata in ipfs

```
make create-metadata NAME="Rust Sample" DESCRIPTION="Generate by Rust" IMAGE_FILENAME=sample.png
```

### mint token

```
make mint CONTRACT=RustToken721 NETWORK=Polygon CONTENT_HASH=QmPDE AMOUNT=1 PACKAGE=EthersRs
```

### deploy token

```
make deploy-token CONTRACT=RustToken721 NETWORK=Polygon PACKAGE=EthersRs
```

### generate private key and public key, address

```
make key-gen
```

### generate signature by private key

```
make sign MESSAGE=hello
```

### verify signature by public key

```
make verify MESSAGE=hello SIGNATURE=2a30...
```
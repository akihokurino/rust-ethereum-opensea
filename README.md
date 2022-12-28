# Rust Ethereum OpenSea CLI

## Specs

- rust latest
- rust-web3 ( https://github.com/tomusdrw/rust-web3 )
- ethers-rs ( https://github.com/gakonst/ethers-rs )
- rust-secp256k1 ( https://github.com/rust-bitcoin/rust-secp256k1 )
- aws-sdk-rust ( https://github.com/awslabs/aws-sdk-rust )
- infura ( https://infura.io/ )
- hardhat ( https://hardhat.org/ )
- solidity v0.8.17
- open zeppelin ( https://openzeppelin.com/ )
- chainlink ( https://docs.chain.link/ )
- oracle

## Related

- https://github.com/akihokurino/lambda-opensea

## ChainLink Notes

- https://zenn.dev/aki030402/articles/546c38c8b437f1

## Command

### get balance（rust-web3 + ethers-rs）

```
make balance
```

### send ether（rust-web3 + ethers-rs）

```
make send-eth ETHER=10 TO_ADDRESS=0x00
```

### mint erc721（rust-web3 + ethers-rs）

```
make mint NAME=name DESCRIPTION=desc IMAGE_FILENAME=sample.png SCHEMA=ERC721
```

### mint erc1155（rust-web3 + ethers-rs）

```
make mint NAME=name DESCRIPTION=desc IMAGE_FILENAME=sample.png AMOUNT=10 SCHEMA=ERC1155
```

### show token info（rust-web3 + ethers-rs）

```
make token-info
```

### generate private key and public key

```
make key-gen
```

### generate signature by private key

```
make sign
```

### verify signature by public key

```
make verify SIGNATURE=2a30...
```

### deploy token（rust-web3 + ethers-rs）

```
make deploy-token SCHEMA=ERC721
make deploy-token SCHEMA=ERC1155
```

### show opensea asset info

```
make opensea-asset-info CONTRACT_ADDRESS=0x00 TOKEN_ID=1
```

### show opensea order info

```
make opensea-sell-order-info CONTRACT_ADDRESS=0x00 TOKEN_ID=1
make opensea-buy-order-info CONTRACT_ADDRESS=0x00 TOKEN_ID=1
```

### sell in opensea

```
make opensea-sell TOKEN_ID=1 ETHER=1 SCHEMA=ERC721
```

### transfer in opensea

```
make opensea-transfer TOKEN_ID=1 TO_ADDRESS=0x00 SCHEMA=ERC721
```


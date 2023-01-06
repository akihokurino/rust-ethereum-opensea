# Rust Ethereum OpenSea CLI

## Specs

- rust latest
- rust-web3 ( https://github.com/tomusdrw/rust-web3 )
- ethers-rs ( https://github.com/gakonst/ethers-rs )
- rust-secp256k1 ( https://github.com/rust-bitcoin/rust-secp256k1 )
- aws-sdk-rust ( https://github.com/awslabs/aws-sdk-rust )
- infura ( https://infura.io/ )
- alchemy ( https://www.alchemy.com/ )
- solidity v0.8.17
- hardhat ( https://hardhat.org/ )
- open zeppelin ( https://openzeppelin.com/ )
- chainlink ( https://docs.chain.link/ )
- oracle

## Support Network
- Ethereum ( Goerli )
- Polygon ( Mumbai )
- Avalanche ( Fuji )

## Related

- https://github.com/akihokurino/lambda-opensea

## ChainLink Notes

- https://zenn.dev/aki030402/articles/546c38c8b437f1

## Command

### get balance

```
make balance
```

### send ether

```
make send-eth ETHER=10 TO_ADDRESS=0x00
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

### mint

```
make mint NAME=name DESCRIPTION=desc IMAGE_FILENAME=sample.png SCHEMA=ERC721
make mint NAME=name DESCRIPTION=desc IMAGE_FILENAME=sample.png AMOUNT=10 SCHEMA=ERC1155
```

### show token info

```
make token-info
```

### deploy token

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
make opensea-sell TOKEN_ID=1 ETHER=1 SCHEMA=ERC1155
```

### transfer in opensea

```
make opensea-transfer TOKEN_ID=1 TO_ADDRESS=0x00 SCHEMA=ERC721
make opensea-transfer TOKEN_ID=1 TO_ADDRESS=0x00 SCHEMA=ERC1155
```


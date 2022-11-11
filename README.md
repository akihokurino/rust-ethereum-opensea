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

## Related

- https://github.com/akihokurino/lambda-opensea

## Command

### mint erc721

```
make mint NAME=name DESCRIPTION=desc IMAGE_FILENAME=test.png SCHEMA=ERC721
```

### mint erc1155

```
make mint NAME=name DESCRIPTION=desc IMAGE_FILENAME=test.png AMOUNT=10 SCHEMA=ERC1155
```

### show contract info

```
make contract-info
```

### show asset info

```
make asset-info CONTRACT_ADDRESS=0x00 TOKEN_ID=1
```

### show order info

```
make sell-order-info CONTRACT_ADDRESS=0x00 TOKEN_ID=1
make buy-order-info CONTRACT_ADDRESS=0x00 TOKEN_ID=1
```

### sell

```
make sell TOKEN_ID=1 ETHER=1 SCHEMA=ERC721
```

### transfer

```
make transfer TOKEN_ID=1 TO_ADDRESS=0x00 SCHEMA=ERC721
```

### generate private key and public key

```
make key-gen
```

### deploy contract

```
make deploy-contract SCHEMA=ERC721
```

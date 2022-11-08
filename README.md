# Rust CLI

## Specs

- rust
- rust-web3 ( https://github.com/tomusdrw/rust-web3 )
- aws-sdk-rust ( https://github.com/awslabs/aws-sdk-rust )
- infura ( https://infura.io/ )
- goerli
- hardhat ( https://hardhat.org/ )
- solidity
- open zeppelin ( https://openzeppelin.com/ )

## Command

### init contract

```
make init
```

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

## Deployed Contract Addresses（Goerli）

- ERC721
  `0x0ccCDB50B9EF2e6FA7133a54533640175E4BcDD1`
- ERC1155
  `0x8287087a8861d466dC3f30cbDBfD51CbC26b3AEC`

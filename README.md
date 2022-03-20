# Rust CLI

## Specs

- rust
- rust-web3 ( https://github.com/tomusdrw/rust-web3 )
- aws-sdk-rust ( https://github.com/awslabs/aws-sdk-rust )
- infura ( https://infura.io/ )
- metamask
- geth
- rinkeby

## Command

### init contract
```
make init
```

### mint erc721
```
make mint NAME=name DESCRIPTION=desc IMAGE_URL=https://test.com/test.png SCHEMA=erc721
make mint NAME=name DESCRIPTION=desc IMAGE_FILENAME=test.png SCHEMA=erc721
```

### mint erc1155
```
make mint NAME=name DESCRIPTION=desc IMAGE_URL=https://test.com/test.png AMOUNT=10 SCHEMA=erc1155
make mint NAME=name DESCRIPTION=desc IMAGE_FILENAME=test.png AMOUNT=10 SCHEMA=erc1155
```

### show contract info
```
make contract-info
```

### show asset info
```
make asset-info CONTRACT_ADDRESS=0xab4d975cc0075e7eaf5eee17f652fe5d4c0ca180 TOKEN_ID=4
```

### show sell order info
```
make sell-order-info CONTRACT_ADDRESS=0xab4d975cc0075e7eaf5eee17f652fe5d4c0ca180 TOKEN_ID=4
```

# Ethereum

## Specs

- truffle ( https://trufflesuite.com/ )
- solidity
- open zeppelin ( https://openzeppelin.com/ )
- infura ( https://infura.io/ )
- metamask
- geth
- rinkeby

## Deployed Contract Addresses（Rinkeby）

- ERC721
  `0x0ccCDB50B9EF2e6FA7133a54533640175E4BcDD1`
- ERC1155
  `0x8287087a8861d466dC3f30cbDBfD51CbC26b3AEC`

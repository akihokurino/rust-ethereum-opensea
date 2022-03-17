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
```

### mint erc1155
```
make mint NAME=name DESCRIPTION=desc IMAGE_URL=https://test.com/test.png AMOUNT=10 SCHEMA=erc1155
```

### show contract info
```
make info
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
  `0x00c1123392F7546f27048Bd36995A2a41196cf7B`
- ERC1155
  `0x6BB17243cB3b2debee345dB5Cdff16f254b7dDcE`

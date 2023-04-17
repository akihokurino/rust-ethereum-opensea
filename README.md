# Rust Ethereum CLI

## Specs

- rust latest
- rust-web3 ( https://github.com/tomusdrw/rust-web3 )
- ethers-rs ( https://github.com/gakonst/ethers-rs )
- rust-secp256k1 ( https://github.com/rust-bitcoin/rust-secp256k1 )
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
make mint
```

### show token info

```
make token-info
```

### deploy token

```
make deploy-token
```
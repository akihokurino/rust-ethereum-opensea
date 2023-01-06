MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

NAME := "RustToken Sample"
DESCRIPTION := "RustToken Sample Generate"
IMAGE_FILENAME := "sample.png"
IMAGE_URL := "https://placehold.jp/3d4070/ffffff/500x500.png?text=Reveal"
AMOUNT := "1"
NETWORK := "Ethereum"
SCHEMA := "ERC721"
CONTRACT_ADDRESS := ""
TOKEN_ID := ""
ETHER := ""
TO_ADDRESS := ""
QUERY := "getLatestPrice"
MESSAGE := "world"
SIGNATURE := ""

build:
	cargo build

balance: build
	./target/debug/rust-ethereum \
	--command balance \
	--network $(NETWORK)

send-eth: build
	./target/debug/rust-ethereum \
	--command send-eth \
	--ether $(ETHER) \
	--to-address $(TO_ADDRESS) \
	--network $(NETWORK)

make-metadata: build
	./target/debug/rust-ethereum \
    --command make-metadata \
  	--name $(NAME) \
    --description $(DESCRIPTION) \
    --image-url $(IMAGE_URL)

mint: build
	./target/debug/rust-ethereum \
	--command mint \
	--name $(NAME) \
	--description $(DESCRIPTION) \
	--image-filename $(IMAGE_FILENAME) \
	--amount $(AMOUNT) \
	--schema $(SCHEMA) \
	--network $(NETWORK)

token-info: build
	./target/debug/rust-ethereum \
	--command token-info \
	--network $(NETWORK)

opensea-asset-info: build
	./target/debug/rust-ethereum \
	--command opensea-asset-info --contract-address $(CONTRACT_ADDRESS) --token-id $(TOKEN_ID)

opensea-sell-order-info: build
	./target/debug/rust-ethereum \
	--command opensea-sell-order-info --contract-address $(CONTRACT_ADDRESS) --token-id $(TOKEN_ID)

opensea-buy-order-info: build
	./target/debug/rust-ethereum \
	--command opensea-buy-order-info --contract-address $(CONTRACT_ADDRESS) --token-id $(TOKEN_ID)

opensea-sell: build
	./target/debug/rust-ethereum \
	--command opensea-sell \
	--token-id $(TOKEN_ID) \
	--schema $(SCHEMA) \
	--ether $(ETHER) \
	--network $(NETWORK)

opensea-transfer: build
	./target/debug/rust-ethereum \
	--command opensea-transfer \
	--token-id $(TOKEN_ID) \
	--schema $(SCHEMA) \
	--to-address $(TO_ADDRESS) \
	--network $(NETWORK)

key-gen: build
	./target/debug/rust-ethereum \
	--command key-gen

sign: build
	./target/debug/rust-ethereum \
	--command sign \
	--message $(MESSAGE)

verify: build
	./target/debug/rust-ethereum \
	--command verify \
	--message $(MESSAGE) \
	--signature $(SIGNATURE)

deploy-token: build
	./target/debug/rust-ethereum \
	--command deploy-token \
	--schema $(SCHEMA) \
	--network $(NETWORK)

extract-abi:
	cat ethereum/artifacts/contracts/RustToken721.sol/RustToken721.json | jq '.abi' > src/ethereum/rust_web3/rust-token721.abi.json
	cat ethereum/artifacts/contracts/RustToken1155.sol/RustToken1155.json | jq '.abi' > src/ethereum/rust_web3/rust-token1155.abi.json
	cat ethereum/artifacts/contracts/RustToken721.sol/RustToken721.json | jq -r '.bytecode' > src/ethereum/rust_web3/rust-token721.bin
	cat ethereum/artifacts/contracts/RustToken1155.sol/RustToken1155.json | jq -r '.bytecode' > src/ethereum/rust_web3/rust-token1155.bin
	cat ethereum/artifacts/contracts/RustToken721.sol/RustToken721.json | jq '.abi' > src/ethereum/ethers_rs/rust-token721.abi.json
	cat ethereum/artifacts/contracts/RustToken1155.sol/RustToken1155.json | jq '.abi' > src/ethereum/ethers_rs/rust-token1155.abi.json
	cat ethereum/artifacts/contracts/RustToken721.sol/RustToken721.json | jq -r '.bytecode' > src/ethereum/ethers_rs/rust-token721.bin
	cat ethereum/artifacts/contracts/RustToken1155.sol/RustToken1155.json | jq -r '.bytecode' > src/ethereum/ethers_rs/rust-token1155.bin
	cat ethereum/artifacts/contracts/RevealToken721.sol/RevealToken721.json | jq '.abi' > src/ethereum/ethers_rs/reveal-token721.abi.json
	cat ethereum/artifacts/contracts/RevealToken721.sol/RevealToken721.json | jq '.bytecode' > src/ethereum/ethers_rs/reveal-token721.bin
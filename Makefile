MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

NAME := "RustToken Sample"
DESCRIPTION := "RustToken Sample Generate"
IMAGE_FILENAME := "sample.png"
AMOUNT := "1"
SCHEMA := "ERC721"
CONTRACT_ADDRESS := ""
TOKEN_ID := ""
ETHER := ""
TO_ADDRESS := ""
QUERY := "getLatestPrice"

build:
	cargo build

mint: build
	./target/debug/rust-ethereum \
	--command mint \
	--name $(NAME) \
	--description $(DESCRIPTION) \
	--image-filename $(IMAGE_FILENAME) \
	--amount $(AMOUNT) \
	--schema $(SCHEMA) \

contract-info: build
	./target/debug/rust-ethereum \
	--command contract-info

asset-info: build
	./target/debug/rust-ethereum \
	--command asset-info --contract-address $(CONTRACT_ADDRESS) --token-id $(TOKEN_ID)

sell-order-info: build
	./target/debug/rust-ethereum \
	--command sell-order-info --contract-address $(CONTRACT_ADDRESS) --token-id $(TOKEN_ID)

buy-order-info: build
	./target/debug/rust-ethereum \
	--command buy-order-info --contract-address $(CONTRACT_ADDRESS) --token-id $(TOKEN_ID)

sell: build
	./target/debug/rust-ethereum \
	--command sell \
	--token-id $(TOKEN_ID) \
	--schema $(SCHEMA) \
	--sell-ether $(ETHER)

transfer: build
	./target/debug/rust-ethereum \
	--command transfer \
	--token-id $(TOKEN_ID) \
	--schema $(SCHEMA) \
	--to-address $(TO_ADDRESS)

key-gen: build
	./target/debug/rust-ethereum \
	--command key-gen

deploy-contract: build
	./target/debug/rust-ethereum \
	--command deploy-contract \
	--schema $(SCHEMA)

sample-oracle-info: build
	./target/debug/rust-ethereum \
    --command sample-oracle-info

sample-oracle-get-time-request: build
	./target/debug/rust-ethereum \
    --command sample-oracle-get-time-request

extract-token-abi:
	cat ethereum/artifacts/contracts/RustToken721.sol/RustToken721.json | jq '.abi' > src/ethereum/rust_web3/rust-token721.abi.json
	cat ethereum/artifacts/contracts/RustToken1155.sol/RustToken1155.json | jq '.abi' > src/ethereum/rust_web3/rust-token1155.abi.json
	cat ethereum/artifacts/contracts/RustToken721.sol/RustToken721.json | jq -r '.bytecode' > src/ethereum/rust_web3/rust-token721.bin
	cat ethereum/artifacts/contracts/RustToken1155.sol/RustToken1155.json | jq -r '.bytecode' > src/ethereum/rust_web3/rust-token1155.bin

extract-sample-abi:
	cat ethereum/artifacts/contracts/SampleOracle.sol/SampleOracle.json | jq '.abi' > src/ethereum/ethers_rs/sample-oracle.abi.json
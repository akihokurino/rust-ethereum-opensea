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

build:
	cargo build

mint: build
	./target/debug/rust-opensea \
	--command mint \
	--name $(NAME) \
	--description $(DESCRIPTION) \
	--image-filename $(IMAGE_FILENAME) \
	--amount $(AMOUNT) \
	--schema $(SCHEMA) \

contract-info: build
	./target/debug/rust-opensea \
	--command contract-info

asset-info: build
	./target/debug/rust-opensea \
	--command asset-info --contract-address $(CONTRACT_ADDRESS) --token-id $(TOKEN_ID)

sell-order-info: build
	./target/debug/rust-opensea \
	--command sell-order-info --contract-address $(CONTRACT_ADDRESS) --token-id $(TOKEN_ID)

buy-order-info: build
	./target/debug/rust-opensea \
	--command buy-order-info --contract-address $(CONTRACT_ADDRESS) --token-id $(TOKEN_ID)

sell: build
	./target/debug/rust-opensea \
	--command sell \
	--token-id $(TOKEN_ID) \
	--schema $(SCHEMA) \
	--sell-ether $(ETHER)

transfer: build
	./target/debug/rust-opensea \
	--command transfer \
	--token-id $(TOKEN_ID) \
	--schema $(SCHEMA) \
	--to-address $(TO_ADDRESS)

key-gen: build
	./target/debug/rust-opensea \
	--command key-gen

deploy-contract: build
	./target/debug/rust-opensea \
	--command deploy-contract

extract-abi:
	cat ethereum/artifacts/contracts/RustToken721.sol/RustToken721.json | jq '.abi' > src/open_sea/rust-token721.abi.json
	cat ethereum/artifacts/contracts/RustToken1155.sol/RustToken1155.json | jq '.abi' > src/open_sea/rust-token1155.abi.json
	cat ethereum/artifacts/contracts/RustToken721.sol/RustToken721.json | jq -r '.bytecode' > src/open_sea/rust-token721.bin
	cat ethereum/artifacts/contracts/RustToken1155.sol/RustToken1155.json | jq -r '.bytecode' > src/open_sea/rust-token1155.bin
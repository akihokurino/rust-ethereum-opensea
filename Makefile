MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

NAME := ""
DESCRIPTION := ""
IMAGE_URL := ""
IMAGE_FILENAME := ""
AMOUNT := "1"
SCHEMA := "erc721"
CONTRACT_ADDRESS := ""
TOKEN_ID := ""

build:
	cargo build

init: build
	./target/debug/rust-opensea \
	--command init

mint: build
	./target/debug/rust-opensea \
	--command mint \
	--nft-name $(NAME) \
	--nft-description $(DESCRIPTION) \
	--nft-image-url $(IMAGE_URL) \
	--nft-image-filename $(IMAGE_FILENAME) \
	--nft-amount $(AMOUNT) \
	--nft-stats level=10 rank=3 \
	--nft-schema $(SCHEMA) \

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

extract-abi:
	cat ethereum/build/contracts/RustToken721.json | jq '.abi' > src/open_sea/rust-token721.abi.json
	cat ethereum/build/contracts/RustToken1155.json | jq '.abi' > src/open_sea/rust-token1155.abi.json
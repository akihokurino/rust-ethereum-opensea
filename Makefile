MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

TITLE := ""
DESCRIPTION := ""
IMAGE_URL := ""

build:
	cargo build

initialize: build
	./target/debug/rust-opensea \
	--command initialize

create-nft: build
	./target/debug/rust-opensea \
	--command create-nft \
	--nft-name $(TITLE) \
	--nft-description $(DESCRIPTION) \
	--nft-image-url $(IMAGE_URL) \
	--nft-stats level=10 rank=3

info: build
	./target/debug/rust-opensea \
	--command info

extract-abi:
	cat ethereum/build/contracts/RustToken721.json | jq '.abi' > src/open_sea/rust-token721.abi.json
	cat ethereum/build/contracts/RustToken1155.json | jq '.abi' > src/open_sea/rust-token1155.abi.json
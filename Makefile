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
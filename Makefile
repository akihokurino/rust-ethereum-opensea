MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

build:
	cargo build

initialize: build
	./target/debug/rust-opensea --command initialize

create-nft: build
	./target/debug/rust-opensea --command create-nft --stats hoge=1 huga=2
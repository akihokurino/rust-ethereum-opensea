MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

NAME := "Rust Sample"
DESCRIPTION := "Generate token by rust"
IMAGE_FILENAME := "sample.png"
IMAGE_URL := "https://placehold.jp/3d4070/ffffff/500x500.png?text=Reveal"
AMOUNT := "10"
NETWORK := "Polygon"
ETHER := "0.01"
TO_ADDRESS := "0x0E91D6613a84d7C8b72a289D8b275AF7717C3d2E"
TOKEN_ID := "1"
MESSAGE := "world"
SIGNATURE := "2a30afb5d5b476a505422d931c5b98a10d6ac6b6fb3a56a27c658a9fa36911f10b079fe392893e684881813e7d07a3fd14048ba902c20eb56eb9f0e7f8c2a1131b"
PACKAGE := "EthersRs"
CONTRACT := "Nft721"
CONTENT_HASH := "QmPDE4pXnFvNtqJ2889HgEQUEft8KCdyMaKKt5zzw3NuMS"

build:
	cargo build

build-impl-ethers-rs:
	cargo build --lib --package impl-ethers-rs

build-impl-rust-web3:
	cargo build --lib --package impl-rust-web3

build-ipfs:
	cargo build --lib --package ipfs

balance: build
	./target/debug/rust-ethereum \
	--command balance \
	--network $(NETWORK) \
	--package $(PACKAGE)

send-eth: build
	./target/debug/rust-ethereum \
	--command send-eth \
	--ether $(ETHER) \
	--to-address $(TO_ADDRESS) \
	--network $(NETWORK) \
	--package $(PACKAGE)

token-info: build
	./target/debug/rust-ethereum \
	--command token-info \
	--network $(NETWORK) \
	--package $(PACKAGE) \
	--contract $(CONTRACT)

create-metadata: build
	./target/debug/rust-ethereum \
    --command create-metadata \
  	--name $(NAME) \
    --description $(DESCRIPTION) \
    --image-filename $(IMAGE_FILENAME)

mint: build
	./target/debug/rust-ethereum \
	--command mint \
	--contract $(CONTRACT) \
	--network $(NETWORK) \
	--content-hash $(CONTENT_HASH) \
	--amount $(AMOUNT) \
	--package $(PACKAGE)

transfer: build
	./target/debug/rust-ethereum \
	--command transfer \
	--contract $(CONTRACT) \
	--network $(NETWORK) \
	--to-address $(TO_ADDRESS) \
	--token-id $(TOKEN_ID) \
	--package $(PACKAGE)

deploy-contract: build
	./target/debug/rust-ethereum \
	--command deploy-contract \
	--contract $(CONTRACT) \
	--network $(NETWORK) \
	--package $(PACKAGE)

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

update-time: build
	./target/debug/rust-ethereum \
    --command update-time \
    --network $(NETWORK)

extract-abi:
	cat ethereum/artifacts/contracts/Nft721.sol/Nft721.json | jq '.abi' > impl_ethers_rs/src/nft_721/abi.json
	cat ethereum/artifacts/contracts/Nft721.sol/Nft721.json | jq -r '.bytecode' > impl_ethers_rs/src/nft_721/bin
	cat ethereum/artifacts/contracts/Nft721.sol/Nft721.json | jq '.abi' > impl_rust_web3/src/nft_721/abi.json
	cat ethereum/artifacts/contracts/Nft721.sol/Nft721.json | jq -r '.bytecode' > impl_rust_web3/src/nft_721/bin

	cat ethereum/artifacts/contracts/Nft1155.sol/Nft1155.json | jq '.abi' > impl_ethers_rs/src/nft_1155/abi.json
	cat ethereum/artifacts/contracts/Nft1155.sol/Nft1155.json | jq -r '.bytecode' > impl_ethers_rs/src/nft_1155/bin
	cat ethereum/artifacts/contracts/Nft1155.sol/Nft1155.json | jq '.abi' > impl_rust_web3/src/nft_1155/abi.json
	cat ethereum/artifacts/contracts/Nft1155.sol/Nft1155.json | jq -r '.bytecode' > impl_rust_web3/src/nft_1155/bin

	cat ethereum/artifacts/contracts/RevealNft721.sol/RevealNft721.json | jq '.abi' > impl_ethers_rs/src/reveal_nft_721/abi.json
	cat ethereum/artifacts/contracts/RevealNft721.sol/RevealNft721.json | jq '.bytecode' > impl_ethers_rs/src/reveal_nft_721/bin

	cat ethereum/artifacts/contracts/Sbt721.sol/Sbt721.json | jq '.abi' > impl_ethers_rs/src/sbt_721/abi.json
	cat ethereum/artifacts/contracts/Sbt721.sol/Sbt721.json | jq '.bytecode' > impl_ethers_rs/src/sbt_721/bin
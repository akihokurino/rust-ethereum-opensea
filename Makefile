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
MESSAGE := "world"
SIGNATURE := ""

build:
	cargo build

balance: build
	./target/debug/rust-ethereum \
	--command balance

send-eth: build
	./target/debug/rust-ethereum \
	--command send-eth \
	--ether $(ETHER) \
	--to-address $(TO_ADDRESS)

mint: build
	./target/debug/rust-ethereum \
	--command mint \
	--name $(NAME) \
	--description $(DESCRIPTION) \
	--image-filename $(IMAGE_FILENAME) \
	--amount $(AMOUNT) \
	--schema $(SCHEMA) \

token-info: build
	./target/debug/rust-ethereum \
	--command token-info

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
	--ether $(ETHER)

opensea-transfer: build
	./target/debug/rust-ethereum \
	--command opensea-transfer \
	--token-id $(TOKEN_ID) \
	--schema $(SCHEMA) \
	--to-address $(TO_ADDRESS)

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
	--schema $(SCHEMA)

sample-oracle-info: build
	./target/debug/rust-ethereum \
    --command sample-oracle-info

sample-oracle-get-time-request: build
	./target/debug/rust-ethereum \
    --command sample-oracle-get-time-request

hello-info: build
	./target/debug/rust-ethereum \
    --command hello-info

hello-set-message: build
	./target/debug/rust-ethereum \
    --command hello-set-message \
    --message $(MESSAGE)

deploy-hello: build
	./target/debug/rust-ethereum \
	--command deploy-hello

extract-abi:
	cat ethereum/artifacts/contracts/RustToken721.sol/RustToken721.json | jq '.abi' > src/ethereum/rust_web3/rust-token721.abi.json
	cat ethereum/artifacts/contracts/RustToken1155.sol/RustToken1155.json | jq '.abi' > src/ethereum/rust_web3/rust-token1155.abi.json
	cat ethereum/artifacts/contracts/RustToken721.sol/RustToken721.json | jq -r '.bytecode' > src/ethereum/rust_web3/rust-token721.bin
	cat ethereum/artifacts/contracts/RustToken1155.sol/RustToken1155.json | jq -r '.bytecode' > src/ethereum/rust_web3/rust-token1155.bin
	cat ethereum/artifacts/contracts/SampleOracle.sol/SampleOracle.json | jq '.abi' > src/ethereum/ethers_rs/sample-oracle.abi.json
	cat ethereum/artifacts/contracts/Hello.sol/Hello.json | jq '.abi' > src/ethereum/ethers_rs/hello.abi.json
	cat ethereum/artifacts/contracts/Hello.sol/Hello.json | jq -r '.bytecode' > src/ethereum/ethers_rs/hello.bin
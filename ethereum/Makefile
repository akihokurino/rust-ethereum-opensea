vendor:
	npm install

compile:
	npx hardhat compile

.PHONY: test
test:
	npx hardhat test

node:
	npx hardhat node

deploy-hardhat:
	npx hardhat run scripts/deploy-nft.ts --network hardhat
	npx hardhat run scripts/deploy-oracle.ts --network hardhat
	npx hardhat run scripts/deploy-reveal-token.ts --network hardhat
	npx hardhat run scripts/deploy-sbt.ts --network hardhat

deploy-geth:
	npx hardhat run scripts/deploy-nft.ts --network geth
	npx hardhat run scripts/deploy-oracle.ts --network geth
	npx hardhat run scripts/deploy-reveal-token.ts --network geth
	npx hardhat run scripts/deploy-sbt.ts --network geth

deploy-nft-goerli:
	npx hardhat run scripts/deploy-nft.ts --network goerli

deploy-oracle-goerli:
	npx hardhat run scripts/deploy-oracle.ts --network goerli

deploy-reveal-token-goerli:
	npx hardhat run scripts/deploy-reveal-token.ts --network goerli

deploy-sbt-goerli:
	npx hardhat run scripts/deploy-sbt.ts --network goerli

deploy-nft-mumbai:
	npx hardhat run scripts/deploy-nft.ts --network mumbai

deploy-nft-fuji:
	npx hardhat run scripts/deploy-nft.ts --network fuji

init-private-net:
	geth --datadir ./private-net/ init ./private-net/genesis.json

console-private-net:
	geth --networkid "15" --port 8545 --http --nodiscover --datadir ./private-net/ --allow-insecure-unlock --unlock 0 --password ./private-net/password console

reset-private-net:
	rm -rf ./private-net/geth/*
	rm -rf ./private-net/history
	touch ./private-net/history
	geth --datadir ./private-net/ init ./private-net/genesis.json

import-private-net:
	geth account import --datadir ./private-net/ ./private-net/key.prv
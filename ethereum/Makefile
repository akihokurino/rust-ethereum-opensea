NETWORK := mumbai

vendor:
	npm install

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

compile:
	npx hardhat compile

.PHONY: test
test:
	npx hardhat test

node:
	npx hardhat node

deploy-nft-721:
	npx hardhat run scripts/deploy-nft-721.ts --network $(NETWORK)

deploy-nft-1155:
	npx hardhat run scripts/deploy-nft-1155.ts --network $(NETWORK)

deploy-sbt-721:
	npx hardhat run scripts/deploy-sbt-721.ts --network $(NETWORK)

deploy-reveal-nft-721:
	npx hardhat run scripts/deploy-reveal-nft-721.ts --network $(NETWORK)

deploy-upgradeable-nft-721-v1:
	npx hardhat run scripts/deploy-upgradeable-nft-721-v1.ts --network $(NETWORK)

deploy-upgradeable-nft-721-v2:
	npx hardhat run scripts/deploy-upgradeable-nft-721-v2.ts --network $(NETWORK)

deploy-nft-market:
	npx hardhat run scripts/deploy-nft-market.ts --network $(NETWORK)
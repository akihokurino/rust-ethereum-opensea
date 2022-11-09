vendor:
	npm install

compile:
	npx hardhat compile

.PHONY: test
test:
	npx hardhat test

node:
	npx hardhat node

deploy-local:
	npx hardhat run scripts/deploy.ts --network hardhat

deploy-goerli:
	npx hardhat run scripts/deploy.ts --network goerli

init-geth-net:
	geth --datadir ./geth-net/ init ./geth-net/genesis.json

console-geth-net:
	geth --networkid "31337" --port 8545 --http --nodiscover --datadir ./geth-net/ --allow-insecure-unlock --unlock 0 --password ./geth-net/password console

reset-geth-net:
	rm -rf ./geth-net/geth/*
	rm -rf ./geth-net/history
	touch ./geth-net/history
	geth --datadir ./geth-net/ init ./geth-net/genesis.json

import-geth-net:
	geth account import --datadir ./geth-net/ ./geth-net/key.prv
MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

compile:
	./node_modules/.bin/truffle compile

develop:
	./node_modules/.bin/truffle develop

develop-private-net:
	./node_modules/.bin/truffle console --network development

deploy-rinkeby:
	./node_modules/.bin/truffle migrate --reset --network rinkeby

init-private-net:
	geth --datadir ./private-net/ init ./private-net/genesis.json
	
console-private-net:
	geth --networkid "10" --port 8545 --http --nodiscover --datadir ./private-net/ --allow-insecure-unlock --unlock 0 --password ./private-net/password console

import-private-net:
	geth account import --datadir ./private-net/ ./private-net/secret

reset-private-net:
	rm -rf ./private-net/geth/*
	rm -rf ./private-net/keystore/*
	rm -rf ./private-net/history
	touch ./private-net/history
	geth --datadir ./private-net/ init ./private-net/genesis.json
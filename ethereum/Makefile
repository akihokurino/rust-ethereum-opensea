MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

compile:
	./node_modules/.bin/truffle compile

console:
	./node_modules/.bin/truffle develop

deploy:
	./node_modules/.bin/truffle migrate --reset --network rinkeby
#!/bin/bash
set -xeu

sleep 60
curl -v http://millau-node-bob:9933/health
curl -v http://rialto-node-bob:9933/health

MESSAGE_LANE=${MSG_EXCHANGE_GEN_LANE:-00000000}

/home/user/parity-relay relay-messages millau-to-rialto \
	--lane $MESSAGE_LANE \
	--source-host millau-node-bob \
	--source-port 9944 \
	--source-signer //Eve \
	--target-host rialto-node-bob \
	--target-port 9944 \
	--target-signer //Eve \
	--prometheus-host=0.0.0.0

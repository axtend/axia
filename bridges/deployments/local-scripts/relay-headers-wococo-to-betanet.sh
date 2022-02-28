#!/bin/bash
#
# Run an instance of the Wococo -> Betanet header sync.
#
# Right now this relies on local Wococo and Betanet networks
# running (which include `pallet-bridge-grandpa` in their
# runtimes), but in the future it could use use public RPC nodes.

set -xeu

RUST_LOG=rpc=trace,bridge=trace ./target/debug/axlib-relay init-bridge wococo-to-betanet \
	--source-host 127.0.0.1 \
	--source-port 9944 \
	--target-host 127.0.0.1 \
	--target-port 9955 \
	--target-signer //Alice

RUST_LOG=rpc=trace,bridge=trace ./target/debug/axlib-relay relay-headers wococo-to-betanet \
	--source-host 127.0.0.1 \
	--source-port 9944 \
	--target-host 127.0.0.1 \
	--target-port 9955 \
	--target-signer //Charlie \
	--prometheus-host=0.0.0.0 \

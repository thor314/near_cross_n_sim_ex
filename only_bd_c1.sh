#!/bin/bash
set -e

./build.sh

pushd c1
near deploy --wasmFile res/c1.wasm --initFunction "new" --initArgs '{"name": "Todd Chavez", "number": 1}' --accountId $1.testnet
popd

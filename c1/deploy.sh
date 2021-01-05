#!/bin/bash

near deploy --wasmFile res/c1.wasm --initFunction "new" --initArgs '{"name": "Todd Chavez", "number": 1}' --accountId c1.$1.testnet

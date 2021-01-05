#!/bin/bash

near deploy --wasmFile res/c2.wasm --initFunction "new" --initArgs '{"friend": "Bojack", "foe": "None to speak of", "i_dunno": true}' --accountId c2.$1.testnet

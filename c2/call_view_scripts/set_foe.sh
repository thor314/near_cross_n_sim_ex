#!/bin/bash

near call c2.$1.testnet set_foe "{\"foe\": \"$2\"}" --accountId c2.$1.testnet

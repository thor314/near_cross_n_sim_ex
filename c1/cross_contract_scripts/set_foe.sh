#!/bin/bash
near call c1.$1.testnet set_foe "{\"foe\": \"$2\"}" --accountId c1.$1.testnet

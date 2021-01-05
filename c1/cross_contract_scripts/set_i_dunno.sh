#!/bin/bash
near call c1.$1.testnet set_i_dunno "{\"i_dunno\": \"$2\"}" --accountId c1.$1.testnet

#!/bin/bash

near call c2.$1.testnet set_i_dunno "{\"i_dunno\": $2}" --accountId c2.$1.testnet

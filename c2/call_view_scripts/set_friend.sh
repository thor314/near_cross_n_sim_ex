#!/bin/bash

near call c2.$1.testnet set_friend "{\"friend\": \"$2\"}" --accountId c2.$1.testnet

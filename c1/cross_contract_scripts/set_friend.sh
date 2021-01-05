#!/bin/bash
near call c1.$1.testnet set_friend "{\"friend\": \"$2\"}" --accountId c1.$1.testnet

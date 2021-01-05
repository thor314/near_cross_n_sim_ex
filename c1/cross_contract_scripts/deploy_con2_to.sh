#!/bin/bash
near call c1.$1.testnet deploy_con2_to '{"subaddress": "c3"}' --accountId $1.testnet
# Failure [c1.dingu.testnet]: Error: The new account_id c3.dingu.testnet can't be created by c1.dingu.testnet

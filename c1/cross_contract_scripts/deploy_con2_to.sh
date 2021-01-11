#!/bin/bash
near call "$1.testnet" deploy_con2_to "{\"subaddress\": \"$2\"}" --accountId "$1.testnet"

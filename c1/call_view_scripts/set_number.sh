#!/bin/bash

near call c1.$1.testnet set_number "{\"number\": $2}" --accountId c1.$1.testnet

#!/bin/bash
near call c1.$1.testnet cb_get_friend_then_set_name "{\"my_address\": \"Tigger\"}" --accountId $1.testnet --gas 435057293590

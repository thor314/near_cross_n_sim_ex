#!/bin/bash
near call c1.$1.testnet cb_get_friend_then_set_name "{\"my_address\": \"$1\"}" --accountId $1.testnet --gas 3428032796700

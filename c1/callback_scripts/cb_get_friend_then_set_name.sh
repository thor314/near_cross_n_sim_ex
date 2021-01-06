#!/bin/bash
near call c1.$1.testnet cb_get_friend_then_set_name '' --accountId c1.$1.testnet --gas 300000000000000

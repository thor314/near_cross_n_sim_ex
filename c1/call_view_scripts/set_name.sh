#!/bin/bash

near call c1.$1.testnet set_name "{\"name\": \"$2\"}" --accountId c1.$1.testnet

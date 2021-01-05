#!/bin/bash
set -e

pushd c1
./deploy.sh $1
popd

#!/bin/bash
set -e

pushd c1
./deploy.sh $1
popd

pushd c2
./deploy.sh $1
popd

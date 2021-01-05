#!/bin/bash
set -e

pushd c1
./build.sh
cp res/c1.wasm ./tests/
popd

pushd c2
./build.sh
cp res/c2.wasm ../c1/tests/
popd

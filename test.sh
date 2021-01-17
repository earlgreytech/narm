#!/bin/bash

cd tests
./build.sh
cd ..
cargo test

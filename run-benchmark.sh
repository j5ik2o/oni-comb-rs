#!/usr/bin/env sh

# cargo bench -p oni-comb-parser-rs

pushd parser
cargo bench -- --profile-time 60
popd

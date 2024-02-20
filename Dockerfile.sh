#!/bin/bash
set -e

apt-get update
apt-get -qq -y install moreutils curl libssl-dev libz-dev clang g++-x86-64-linux-gnu g++-aarch64-linux-gnu gcc-x86-64-linux-gnu gcc-aarch64-linux-gnu lld build-essential

curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.75.0

rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu

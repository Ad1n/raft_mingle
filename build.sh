#!/bin/bash
set -e
set -x

if [[ -z "$TARGET" ]]; then
    TARGET=aarch64-unknown-linux-gnu
fi

case $TARGET in
    x86_64-unknown-linux-gnu)
        VERSION="${CI_COMMIT_SHORT_SHA}_x86_64"
        export CC=x86_64-linux-gnu-gcc
        export CXX=x86_64-linux-gnu-g++
        ;;
    aarch64-unknown-linux-gnu)
        VERSION="${CI_COMMIT_SHORT_SHA}_aarch64"
        export CC=aarch64-linux-gnu-gcc
        export CXX=aarch64-linux-gnu-g++
        export CFLAGS="-mcpu=generic"
        ;;
    *)
        echo "Unknown target $TARGET"
        exit 1
        ;;
esac

echo "Building version $VERSION ($TARGET)"

cargo build --release --target=$TARGET

BIN=$(find "$CARGO_TARGET_DIR/$TARGET/release/" -maxdepth 1 -executable -type f)
DEST_DIR=/usr/local/bin

echo "Copying $BIN to $DEST_DIR..."
cp "$CARGO_TARGET_DIR"/$TARGET/release/server $DEST_DIR

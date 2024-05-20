#!/bin/bash
export TARGET_NAME="ralertsinua-x86_64-unknown-linux-musl"
export DIST="target/distrib/$TARGET_NAME"

# docker run \
#     -v cargo-cache:/root/.cargo/registry \
#     -v "$PWD:/volume" \
#     --rm -it clux/muslrust cargo build --release

# chown after docker changes permission to target folder
chown -R $USER:$USER target

# Create a directory for the files to be archived
mkdir -p $DIST

# Copy the files into the directory
cp target/x86_64-unknown-linux-musl/release/ralertsinua $DIST
cp CHANGELOG.md LICENSE README.md $DIST

# Create tzr.xz archive with name TARGET_NAME containing folder DIST
tar -C target/distrib -cJf target/distrib/$TARGET_NAME.tar.xz $TARGET_NAME

#!/bin/bash
export TARGET_NAME="ralertsinua-x86_64-unknown-linux-musl"
export DIST="target/distrib/$TARGET_NAME"
# INFO: https://github.com/clux/muslrust?tab=readme-ov-file#filesystem-permissions-on-local-builds
# Filesystem permissions on local builds
# When building locally, the permissions of the musl parts of the ./target artifacts dir will be owned by root and requires sudo rm -rf target/ to clear. This is an intended complexity tradeoff with user builds.
docker run \
    -v cargo-cache:/root/.cargo/registry \
    -v "$PWD:/volume" \
    --rm -it clux/muslrust \
    cargo build --release --no-default-features --features cache,reqwest-rustls-tls -vv
    # -e CARGO_FEATURE_REQWEST_RUSTLS_TLS=1 \
    # -e RUSTFLAGS=-Ctarget-feature=-crt-static \

# chown after docker changes permission to target folder
sudo chown -R $USER:$USER target

# Create a directory for the files to be archived
mkdir -p $DIST

# Copy the files into the directory
cp target/x86_64-unknown-linux-musl/release/ralertsinua $DIST
cp CHANGELOG.md LICENSE README.md $DIST

# Create tzr.xz archive with name TARGET_NAME containing folder DIST
tar -C target/distrib -cJf target/distrib/$TARGET_NAME.tar.xz $TARGET_NAME

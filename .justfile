set export

export MUSL_TARGET_NAME := "ralertsinua-x86_64-unknown-linux-musl"
export MUSL_DIST := "target/distrib/'${MUSL_TARGET_NAME}'"
export RUST_LOG := "verbose"
export RUST_BACKTRACE := "1"

# foo := if env_var("RELEASE") == "true" { `get-something-from-release-database` } else { "dummy-value" }

clean:
    #!/bin/sh
    find target -mindepth 1 -maxdepth 1 ! -name "debug" ! -name "tmp" -exec rm -rf {} +

docs:
    #!/bin/sh
    curl --proto '=https' --tlsv1.2 -LsSf https://github.com/axodotdev/oranda/releases/latest/download/oranda-installer.sh | sh
    oranda build --config-path docs/oranda.render.json

[confirm]
publish:
    #!/bin/sh
    # If crate A depends on crate B, B must come before A in this list
    crates=(
        ralertsinua-models
        ralertsinua-http
        ralertsinua-geo
    )
    for crate in "${crates[@]}"; do
        echo "Publishing ${crate}"
        (
            cd "$crate"
            cargo publish --no-verify
        )
        sleep 20
    done

[group('build')]
build target_env="gnu":
    #!/bin/sh
    echo "Building for target_env: ${target_env}"
    if [ "{{target_env}}" = "musl" ]; then
        # INFO: https://github.com/clux/muslrust?tab=readme-ov-file#filesystem-permissions-on-local-builds
        # Filesystem permissions on local builds
        # When building locally, the permissions of the musl parts of the ./target artifacts dir will be owned by root and requires sudo rm -rf target/ to clear. This is an intended complexity tradeoff with user builds.
        docker run \
            -v cargo-cache:/root/.cargo/registry \
            -v "$PWD:/volume" \
            -e OPENSSL_LIB_DIR=/usr/lib \
            -e OPENSSL_INCLUDE_DIR=/usr/include \
            --rm -it clux/muslrust \
            cargo build --release --no-default-features --features cache,reqwest-rustls-tls

            # Create a directory for the files to be archived
            mkdir -p $MUSL_DIST
            cp target/x86_64-unknown-linux-musl/release/ralertsinua $MUSL_DIST
            cp CHANGELOG.md LICENSE README.md $MUSL_DIST
            tar -C target/distrib -cJf target/distrib/$MUSL_TARGET_NAME.tar.xz $MUSL_TARGET_NAME
    else
        cargo dist build
    fi

    find ./target -type f -name 'ralertsinua' -exec du -sh {} \;


[group('run')]
run mode="dev":
    #!/bin/sh
    echo "Running with mode: ${mode}"
    if [ "{{mode}}" = "prod" ]; then
        if test -f ./target/release/ralertsinua; then
            ./target/release/ralertsinua
        else
            echo "Binary not found. Run 'cargo build --release' first."
        fi
    elif [ "{{mode}}" = "ttyd" ]; then
        docker build -f Dockerfile -t ralertsinua-ttyd .
        docker run --env-file .env -p 7681:7681 --rm -it ralertsinua-ttyd:latest
    else
        cargo run
    fi

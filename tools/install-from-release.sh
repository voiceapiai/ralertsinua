#!/bin/sh

set -euxo pipefail

cd "$(mktemp -d)"

base_url="https://github.com/voiceapiai/ralertsinua/releases/latest/download/ralertsinua-"

os="$(uname -s)"
if [ "$os" == "Darwin" ]; then
    url="${base_url}universal-apple-darwin.zip"
    curl -LO --proto '=https' --tlsv1.2 -sSf "$url"
    unzip ralertsinua-universal-apple-darwin.zip
elif [ "$os" == "Linux" ]; then
    machine="$(uname -m)"
    target="${machine}-unknown-linux-musl"
    if [ "$machine" == "armv7" ]; then
        target="${target}eabihf"
    fi

    url="${base_url}${target}.tar.xz"
    curl -L --proto '=https' --tlsv1.2 -sSf "$url" | tar -xvJf - --strip-components=1
elif [ "${OS-}" = "Windows_NT" ]; then
    machine="$(uname -m)"
    target="${machine}-pc-windows-msvc"
    url="${base_url}${target}.zip"
    curl -LO --proto '=https' --tlsv1.2 -sSf "$url"
    unzip "ralertsinua-${target}.zip"
else
    echo "Unsupported OS ${os}"
    exit 1
fi

# Find the 'ralertsinua' binary and move it to a directory in your PATH
find . -type f -name 'ralertsinua' -exec mv {} /usr/local/bin/ \;
# Install
# cargo binstall -y --force ./ralertsinua

CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"

if ! [[ ":$PATH:" == *":$CARGO_HOME/bin:"* ]]; then
    echo
    printf "\033[0;31mYour path is missing %s, you might want to add it.\033[0m\n" "$CARGO_HOME/bin"
    echo
fi

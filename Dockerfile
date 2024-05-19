# TODO: consider https://github.com/clux/muslrust
# TODO: consider https://github.com/raonigabriel/web-terminal

# Start from the Rust Alpine image for the builder stage
FROM rust:alpine

# Set the ALPINE_BUILD environment variable
ENV ALPINE_BUILD=1
ENV RUST_BACKTRACE=1
ENV RUSTFLAGS=-Ctarget-feature=-crt-static

# Install OpenSSL and musl-dev
RUN apk add --no-cache curl musl-dev valgrind openssl-dev

# Set the OpenSSL lib directory
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include

# Install cargo-binstall
# RUN cargo install cargo-binstall
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | sh
RUN cargo binstall --help

# Install the latest version of ralertsinua
# RUN cargo binstall --pkg-url="https://github.com/voiceapiai/ralertsinua/releases/latest/download/ralertsinua-x86_64-unknown-linux-musl.tar.xz" --pkg-fmt="txz" ralertsinua -v --no-discover-github-token # FIXME
COPY install-from-release.sh /
RUN chmod +x /install-from-release.sh
RUN /install-from-release.sh

# CMD ["valgrind --tool=memcheck --leak-check=full  ./target/x86_64-unknown-linux-musl/release/ralertsinua"]
CMD ["ralertsinua"]

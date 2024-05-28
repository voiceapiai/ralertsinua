FROM raonigabriel/web-terminal

# Set the OpenSSL lib directory
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include

ENV RELEASE_URL=https://github.com/voiceapiai/ralertsinua/releases/latest/download/ralertsinua-x86_64-unknown-linux-musl.tar.xz
ENV RUST_BACKTRACE=1

WORKDIR /home/ralertsinua

# COPY /target/x86_64-unknown-linux-musl/release                    # This is for local testing
# RUN mv ralertsinua /usr/local/bin                                 # This is for local testing

# Install the latest musl version of ralertsinua
ADD $RELEASE_URL ralertsinua.tar.xz
RUN tar -xvJf ralertsinua.tar.xz && \
    find . -type f -name 'ralertsinua' -exec mv {} /usr/local/bin/ \; && \
    rm ralertsinua.tar.xz

HEALTHCHECK CMD ["ralertsinua", "--help"]

# Comment this line if you don't need ttyd, and then just run ralertsina
# CMD [ "ttyd", "-s", "3", "-t", "titleFixed=Rust alerts.in.ua TUI - ralertsinua", "-t", "rendererType=webgl", "-t", "disableLeaveAlert=true", "ralertsinua" ]
CMD [ "ralertsinua" ]

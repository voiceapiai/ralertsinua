FROM raonigabriel/web-terminal

ENV RELEASE_URL=https://github.com/voiceapiai/ralertsinua/releases/latest/download/ralertsinua-x86_64-unknown-linux-musl.tar.xz

WORKDIR /home/ralertsinua

# Install the latest musl version of ralertsinua
ADD $RELEASE_URL ralertsinua.tar.xz
RUN tar -xvJf ralertsinua.tar.xz && \
    find . -type f -name 'ralertsinua' -exec mv {} /usr/local/bin/ \; && \
    rm ralertsinua.tar.xz

HEALTHCHECK CMD ["ralertsinua", "--help"]

# Comment this line if you don't need ttyd, and then just run ralertsina
CMD [ "ttyd", "-s", "3", "-t", "titleFixed=Rust alerts.in.ua TUI - ralertsinua", "-t", "rendererType=webgl", "-t", "disableLeaveAlert=true", "ralertsinua" ]
# CMD [ "ralertsinua" ]

# TODO: https://github.com/colinmurphy1/docker-ttyd/blob/main/entrypoint.sh

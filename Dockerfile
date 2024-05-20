FROM raonigabriel/web-terminal
RUN apk add --no-cache curl

ENV RELEASE_URL=https://github.com/voiceapiai/ralertsinua/releases/latest/download/ralertsinua-x86_64-unknown-linux-musl.tar.xz

WORKDIR /home/ralertsinua


# Install the latest musl version of ralertsinua
RUN curl -L --proto '=https' --tlsv1.2 -sSf "$RELEASE_URL" | tar -xvJf -
RUN find . -type f -name 'ralertsinua' -exec mv {} /usr/local/bin/ \;

HEALTHCHECK CMD ["ralertsinua", "--help"]

CMD [ "ttyd", "-s", "3", "-t", "titleFixed=Rust alerts.in.ua TUI - ralertsinua", "-t", "rendererType=webgl", "-t", "disableLeaveAlert=true", "ralertsinua" ]

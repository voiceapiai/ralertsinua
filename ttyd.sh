#!/bin/sh

# Build the Docker image
docker build -f Dockerfile -t ralertsinua-ttyd .

# Run the Docker container with the environment variables
docker run --env-file .env -p 7681:7681 --rm -it ralertsinua-ttyd:latest

#!/bin/bash

curl --proto '=https' --tlsv1.2 -LsSf https://github.com/axodotdev/oranda/releases/latest/download/oranda-installer.sh | sh
oranda build --config-path docs/oranda.render.json

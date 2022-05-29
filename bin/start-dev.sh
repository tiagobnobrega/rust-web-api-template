#!/usr/bin/env sh
sh ./src/bin/init-docker.sh
echo "Initializing cargo watcher..."
cargo watch -w "./src" -x 'run'
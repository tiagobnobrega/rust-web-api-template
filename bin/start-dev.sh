#!/usr/bin/env sh
# Absolute path to this script, e.g. /home/user/bin/foo.sh
SCRIPT=$(readlink -f "$0")
# Absolute path this script is in, thus /home/user/bin
SCRIPTPATH=$(dirname "$SCRIPT")

sh "$SCRIPTPATH"/init-docker.sh

echo "Setup diesel..."
diesel setup;
echo "Initializing cargo watcher..."
cargo watch -w "./src" -x 'run'
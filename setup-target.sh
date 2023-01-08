#!/bin/bash

set -e

source="target"

initial=$(pwd)
cd "$(git rev-parse --show-toplevel)"

if [ -d "/var/tmp" ]; then
	target_base="/var/tmp"
else
	target_base="/tmp"
fi

if [[ ! -L "$source" || ! -d "$source" ]]; then
	echo "Removing $source."
	rm -rf "$source"

	target=$(mktemp -p "$target_base" -d -t "$USER.XXXXXXXXX")
	echo "Creating $source to $target."
	ln -s "$target" "$source"
else
	echo "Target already created and is already a symlink."
fi

echo "Running initial build."
cargo build

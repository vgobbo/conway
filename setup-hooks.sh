#!/bin/bash

set -e

source="hooks"

cd "$(git rev-parse --show-toplevel)"

for hook in "$source"/* ; do
	echo "Configuring hook $(basename "$hook")."
	ln -srf "$hook" ".git/hooks/$(basename "$hook")"
done

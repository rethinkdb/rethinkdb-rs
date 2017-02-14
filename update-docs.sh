#!/bin/sh

# Run in a subshell to maintain the current directory
(
	# Go to top level directory
	cd "$(dirname "$0")"

	# Make sure the `docs-origin` remote is defined
	git remote add docs-origin https://github.com/rust-rethinkdb/docs.git &>/dev/null

	# Pull the latest docs
	git subtree pull --prefix build/docs docs-origin 1.0.x
)

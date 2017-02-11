#!/bin/sh

# Run in a subshell to maintain the current directory
(
	# Go to top level directory
	pushd $(dirname $0) > /dev/null

	# Pull the latest docs
	git subtree pull --prefix build/docs docs-origin master
)

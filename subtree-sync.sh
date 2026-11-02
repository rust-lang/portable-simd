#!/bin/bash

# Script for performing pulls (pull changes from rust-lang/rust) and pushes
# (push changes to rust-lang/rust) using Josh.
# Install the https://github.com/rust-lang/josh-sync to use this script.
# Usage:
# 1. Pull
# ./subtree-sync.sh pull
#
# 2. Push
# ./subtree-sync.sh push <github-username>

set -eou pipefail

git fetch origin

if [ "$(git rev-parse --show-prefix)" != "" ]; then
    echo "Run this script from the git root" >&2
    exit 1
fi

if [ "$(git rev-parse HEAD)" != "$(git rev-parse origin/main)" ]; then
    echo "$(pwd) is not at origin/main" >&2
    exit 1
fi

today=$(date +%Y-%m-%d)

case $1 in
    "push")
        username=${2}
        branch=sync-to-rust-${today}
        rustc-josh-sync push "${branch}" "${username}"
        ;;
    "pull")
        branch=sync-from-rust-${today}
        git checkout -B "${branch}"
        rustc-josh-sync pull
        ;;
esac

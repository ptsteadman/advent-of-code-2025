#!/usr/bin/env bash

# Fetch Rust toolchains if they don't exist yet
bazel --output_base=~/ide_bazel fetch //... 2>/dev/null

bazel \
    --output_base=~/ide_bazel \
    run \
    @rules_rust//tools/rust_analyzer:discover_bazel_rust_project -- \
    --bazel_arg=--watchfs \
    ${1:+"$1"} 2> /tmp/discover_bazel_rust_project.log

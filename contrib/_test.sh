#!/usr/bin/env bash

set -ex

REPO_DIR=$(git rev-parse --show-toplevel)
FEATURES="std"                  # Note we don't currently test with "external-secp". 

cargo --version
rustc --version

# Work out if we are using a nightly toolchain.
NIGHTLY=false
if cargo --version | grep nightly; then
    NIGHTLY=true
fi

# Make all cargo invocations verbose
export CARGO_TERM_VERBOSE=true

# Defaults / "std" feature
cargo build --locked
cargo test --locked

# No features.
cargo build --locked --no-default-features
cargo test --locked --no-default-features

if [ "$DO_LINT" = true ]
then
    cargo clippy --locked --all-features --all-targets -- -D warnings
fi

# Build the docs if told to (this only works with the nightly toolchain)
if [ "$DO_DOCSRS" = true ]; then
    RUSTDOCFLAGS="--cfg docsrs -D warnings -D rustdoc::broken-intra-doc-links" cargo +nightly doc --all-features
fi

# Build the docs with a stable toolchain, in unison with the DO_DOCSRS command
# above this checks that we feature guarded docs imports correctly.
if [ "$DO_DOCS" = true ]; then
    RUSTDOCFLAGS="-D warnings" cargo +stable doc --all-features
fi

# Run formatter if told to.
if [ "$DO_FMT" = true ]; then
    if [ "$NIGHTLY" = false ]; then
        echo "DO_FMT requires a nightly toolchain (consider using RUSTUP_TOOLCHAIN)"
        exit 1
    fi
    rustup component add rustfmt
    cargo fmt --check || exit 1
fi

exit 0

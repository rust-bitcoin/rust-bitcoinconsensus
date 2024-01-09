default:
  @just --list

# Cargo build everything.
build:
  cargo build --all-targets

# Cargo check everything.
check:
  cargo check --all-targets

# Lint everything.
lint:
  cargo clippy --all-targets -- --deny warnings

# Check the formatting
format:
  cargo +nightly fmt --all --check

# Update the recent and minimal lock files.
update-lock-files:
  contrib/update-lock-files.sh

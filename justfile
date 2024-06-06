default:
  @just --list

# Cargo build everything.
build:
  cargo build --workspace --all-targets --all-features

# Cargo check everything.
check:
  cargo check --workspace --all-targets --all-features

# Lint everything.
lint:
  cargo +$(cat ./nightly-version) clippy --workspace --all-targets --all-features -- --deny warnings

# Run cargo fmt
fmt:
  cargo +$(cat ./nightly-version) fmt --all

# Check the formatting
format:
  cargo +$(cat ./nightly-version) fmt --all --check

# Update the recent and minimal lock files.
update-lock-files:
  contrib/update-lock-files.sh

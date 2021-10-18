#!/bin/bash
RUST_BACKTRACE=1 cargo test --all-features -- --nocapture
# RUST_BACKTRACE=1 cargo test --release --all-features -- --nocapture
# RUST_BACKTRACE=1 cargo test --features techblox -- --nocapture
# RUST_BACKTRACE=1 cargo test --features robocraft -- --nocapture
exit $?

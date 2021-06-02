#!/bin/bash
RUST_BACKTRACE=1 cargo test --all-features -- --nocapture
# RUST_BACKTRACE=1 cargo test --features techblox -- --nocapture
exit $?

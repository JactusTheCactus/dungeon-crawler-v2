#!/usr/bin/env bash
set -uo pipefail
cargo +nightly fmt
cargo check
cargo clippy
cargo build
./target/debug/dungeon-crawler-v2

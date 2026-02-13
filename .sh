#!/usr/bin/env bash
set -uo pipefail
game() {
	cargo +nightly fmt
	cargo check
	cargo clippy
	cargo build
	./target/debug/dungeon-crawler-v2
}
page() {
	find docs -name '*.html' -o -name '*.css' -delete
	npx pug3 docs
	npx sass docs \
		--no-source-map \
		--style=compressed
}
page
game

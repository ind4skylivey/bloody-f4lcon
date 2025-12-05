fmt:
	cargo fmt

lint:
	cargo clippy --all-targets -- -D warnings

test:
	cargo test

build:
	cargo build

release:
	cargo build --release

ci: fmt lint test



all: fmt clippy unit-tests

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all --all-targets --all-features

unit-tests:
	cargo test


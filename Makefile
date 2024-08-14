

all: fmt clippy unit-tests

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all --all-targets --all-features

unit-tests:
	cargo test

mol-gen:
	moleculec --schema-file tests/schemas/test1.mol --language rust | rustfmt > tests/src/old/test1.rs



all: fmt clippy unit-tests example-tests

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all --all-targets --all-features

unit-tests:
	cargo test

example-tests:
	cargo run --bin serde_molecule_customized_union_id
	cd examples/serde_molecule_nostd && make build && cd ../..

mol-gen:
	moleculec --schema-file tests/schemas/test1.mol --language rust | rustfmt > tests/src/old/test1.rs

dry-publish:
	cargo publish -p serde_molecule --dry-run --allow-dirty
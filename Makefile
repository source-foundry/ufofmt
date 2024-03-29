all: build-release

build-test:
	cargo build

build-release:
	cargo build --release

install:
	cargo install --path .

fmt:
	cargo fmt --all

check-fmt:
	cargo fmt --all -- --check

check-clippy:
	cargo clippy -- -D warnings

.PHONY: all build-test build-release install fmt check-fmt check-clippy
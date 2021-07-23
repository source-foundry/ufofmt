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

.PHONY: all build-test build-release install fmt check-fmt
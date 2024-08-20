SHELL := /bin/bash

.PHONY: all lib bin

all: lib bin

lib:
	cargo build --no-default-features && \
	cargo clippy --no-default-features -- -D warnings && \
	cargo fmt --all -- --check && \
	cargo test --no-default-features

bin:
	cargo build && \
	cargo clippy -- -D warnings && \
	cargo fmt --all -- --check && \
	cargo test

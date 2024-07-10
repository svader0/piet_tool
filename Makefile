.PHONY: build build-release install
install: build-release
	cargo install --locked --path .

build-release:
	cargo build --release

build:
	cargo build
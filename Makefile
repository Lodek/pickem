.PHONY: build install


build:
	cargo build --release

install:
	mkdir bin
	cp target/release/pickem bin
	cp shell/core.zsh bin

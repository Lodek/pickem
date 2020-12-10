.PHONY: install

install:
	cargo build --release
	cp target/release/pickem /usr/bin/pickem
	mkdir -p /usr/share/pickem
	cp shell/pickem.zsh /usr/share/pickem/pickem.zsh

uninstall:
	rm /usr/bin/pickem
	rm -r /usr/share/pickem

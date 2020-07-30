mailpreview-cli:
	cargo build --release

install:
	cp ./target/release/mailpreview-cli /usr/local/bin/
.PHONY: install

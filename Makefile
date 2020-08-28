mailpreview-cli:
	cargo build --release

mailpreview-cli-static:
	cargo build --release --target x86_64-unknown-linux-musl
.PHONY: mailpreview-cli-static

install:
	cp ./target/release/mailpreview-cli /usr/local/bin/
.PHONY: install

static_install:
	cp ./target/x86_64-unknown-linux-musl/release/mailpreview-cli /usr/local/bin/
.PHONY: static_install

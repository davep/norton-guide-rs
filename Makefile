.PHONY: all
all:
	cargo build

.PHONY: test
test:
	cargo test

.PHONY: rtfm
rtfm:
	cargo doc --open

.PHONY: lint
lint:
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: tidy
tidy:
	cargo fmt --all -- --check

### Makefile ends here

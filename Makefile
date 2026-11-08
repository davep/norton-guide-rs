.PHONY: all
all:
	cargo build

.PHONY: test
test:
	cargo test

.PHONY: rtfm
rtfm:
	cargo doc --open

### Makefile ends here

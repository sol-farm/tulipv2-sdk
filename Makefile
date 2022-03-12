.PHONY: lint
lint:
	cargo +nightly clippy --fix -Z unstable-options --all

.PHONY: fmt
fmt:
	find -type f -name "*.rs" -not -path "*target*" -exec rustfmt --edition 2021 {} \;

.DEFAULT_GOAL := help

SHELL=bash
BASE ?= main

.PHONY: fix
fix:
	@$(MAKE) -s -C .. $@

.PHONY: fmt
fmt:  ## Run rustfmt and dprint
	cargo fmt --all
	dprint fmt

.PHONY: check
check:  ## Run cargo check with all features
	cargo check -p apollo --all-features

.PHONY: clippy
clippy:  ## Run clippy with all features
	cargo clippy --all-targets --all-features -- -W clippy::dbg_macro

.PHONY: clippy-default
clippy-default:  ## Run clippy with default features
	cargo clippy --all-targets -- -W clippy::dbg_macro

.PHONY: pre-commit
pre-commit: fmt clippy clippy-default  ## Run autoformatting and linting

.PHONY: check-features
check-features:  ## Run cargo check for feature flag combinations (warning: slow)
	cargo hack check -p apollo --each-feature --no-dev-deps

.PHONY: miri
miri:  ## Run miri
	# not tested on all features because miri does not support SIMD
	# some tests are also filtered, because miri cannot deal with the rayon threadpool
	# we ignore leaks because the thread pool of rayon is never killed.
	MIRIFLAGS="-Zmiri-disable-isolation -Zmiri-ignore-leaks -Zmiri-disable-stacked-borrows" \
	APOLLO_ALLOW_EXTENSION=1 \
	cargo miri test \
	    --features object \
	    -p apollo-core \
#	    -p apollo-arrow

.PHONY: test
test:  ## Run tests
	cargo test --all-features \
		-p apollo-compute \
		-p apollo-core \
		-p apollo-io \
		-p apollo-lazy \
		-p apollo-ops \
		-p apollo-plan \
		-p apollo-row \
		-p apollo-sql \
		-p apollo-testing \
		-p apollo-time \
		-p apollo-utils \
		-- \
		--test-threads=2

.PHONY: nextest
nextest:  ## Run tests with nextest
	cargo nextest run --all-features \
		-p apollo-compute \
		-p apollo-core \
		-p apollo-io \
		-p apollo-lazy \
		-p apollo-ops \
		-p apollo-plan \
		-p apollo-row \
		-p apollo-sql \
		-p apollo-testing \
		-p apollo-time \
		-p apollo-utils \

.PHONY: integration-tests
integration-tests:  ## Run integration tests
	cargo test --all-features --test it -p apollo

.PHONY: test-doc
test-doc:  ## Run doc examples
	cargo test --doc \
	    -p apollo-lazy \
	    -p apollo-io \
	    -p apollo-core \
		-p apollo-testing \
		-p apollo-sql

.PHONY: bench-save
bench-save:  ## Run benchmark and save
	cargo bench --features=random --bench $(BENCH) -- --save-baseline $(SAVE)

.PHONY: bench-cmp
bench-cmp:  ## Run benchmark and compare
	cargo bench --features=random --bench $(BENCH) -- --load-baseline $(FEAT) --baseline $(BASE)

.PHONY: doctest
doctest:  ## Check that documentation builds
	cargo doc --no-deps --all-features -p apollo-utils
	cargo doc --no-deps --features=docs-selection -p apollo-core
	cargo doc --no-deps -p apollo-time
	cargo doc --no-deps -p apollo-ops
	cargo doc --no-deps --all-features -p apollo-io
	cargo doc --no-deps --all-features -p apollo-lazy
	cargo doc --no-deps --features=docs-selection -p apollo
	cargo doc --no-deps --all-features -p apollo-sql

.PHONY: publish
publish:  ## Publish Apollo crates
	cargo publish --allow-dirty -p apollo-error
	cargo publish --allow-dirty -p apollo-utils
	cargo publish --allow-dirty -p apollo-schema
	cargo publish --allow-dirty -p apollo-arrow
	cargo publish --allow-dirty -p apollo-compute
	cargo publish --allow-dirty -p apollo-dtype
	cargo publish --allow-dirty -p apollo-row
	cargo publish --allow-dirty -p apollo-json
	cargo publish --allow-dirty -p apollo-core
	cargo publish --allow-dirty -p apollo-ffi
	cargo publish --allow-dirty -p apollo-ops
	cargo publish --allow-dirty -p apollo-testing
	cargo publish --allow-dirty -p apollo-time
	cargo publish --allow-dirty -p apollo-parquet
	cargo publish --allow-dirty -p apollo-io
	cargo publish --allow-dirty -p apollo-plan
	cargo publish --allow-dirty -p apollo-expr
	cargo publish --allow-dirty -p apollo-mem-engine
	cargo publish --allow-dirty -p apollo-stream
	cargo publish --allow-dirty -p apollo-lazy
	cargo publish --allow-dirty -p apollo-sql
	cargo publish --allow-dirty -p apollo
	# This is independent
	cargo publish --allow-dirty -p apollo-python

.PHONY: help
help:  ## Display this help screen
	@echo -e "\033[1mAvailable commands:\033[0m"
	@grep -E '^[a-z.A-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-18s\033[0m %s\n", $$1, $$2}' | sort

.PHONY: check-wasm
check-wasm:  ## Check wasm build without supported features
	RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo hack check --target wasm32-unknown-unknown -p apollo --no-dev-deps  \
		--each-feature                        \
		--exclude-features async              \
		--exclude-features aws                \
		--exclude-features azure              \
		--exclude-features cloud              \
		--exclude-features decompress         \
		--exclude-features default            \
		--exclude-features docs-selection     \
		--exclude-features extract_jsonpath   \
		--exclude-features fmt                \
		--exclude-features gcp                \
		--exclude-features csv                \
		--exclude-features ipc                \
		--exclude-features ipc_streaming      \
		--exclude-features json               \
		--exclude-features nightly            \
		--exclude-features parquet            \
		--exclude-features performant         \
		--exclude-features streaming          \
		--exclude-features http               \
		--exclude-features full               \
		--exclude-features test

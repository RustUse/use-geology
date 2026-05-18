.PHONY: help fmt check lint test test-minimal build doc examples publish-dry-run-focused publish-dry-run-facade release-readiness verify

FOCUSED_CRATES := use-mineral use-rock use-stratum use-formation use-fault use-tectonic-plate use-geologic-time use-fossil use-geologic-process use-sediment

help:
	@printf "%s\n" \
		"help                    Show available repository tasks" \
		"fmt                     Check formatting with rustfmt" \
		"check                   Run cargo check for the workspace" \
		"lint                    Run clippy with warnings denied" \
		"test                    Run workspace tests with all features" \
		"test-minimal            Run workspace tests with no default features" \
		"build                   Build the workspace with all features" \
		"doc                     Build workspace docs without dependencies" \
		"examples                Check all examples" \
		"publish-dry-run-focused Dry-run publish focused crates" \
		"publish-dry-run-facade  Dry-run publish use-geology after registry propagation" \
		"release-readiness       Run the focused release validation path" \
		"verify                  Run the main workspace validation path"

fmt:
	cargo fmt --all -- --check

check:
	cargo check --workspace --all-features

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-features

test-minimal:
	cargo test --workspace --no-default-features

build:
	cargo build --workspace --all-features

doc:
	cargo doc --workspace --all-features --no-deps

examples:
	cargo check --workspace --all-features --examples

publish-dry-run-focused:
	@for crate in $(FOCUSED_CRATES); do \
		cargo package --list -p $$crate; \
		cargo publish --dry-run --allow-dirty -p $$crate; \
	done

publish-dry-run-facade:
	cargo publish --dry-run --allow-dirty -p use-geology

release-readiness: verify examples test-minimal publish-dry-run-focused

verify: fmt lint test build
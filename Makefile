.PHONY: build install uninstall clean release check fmt lint test help

## Build & Install

build: ## Build debug binary
	cargo build

release: ## Build release binary
	cargo build --release

install: ## Install binary + run interactive setup
	@cargo install --path .
	@alcove setup

uninstall: ## Remove skills, config, and binary
	@alcove uninstall
	@cargo uninstall alcove 2>/dev/null || true

## Development

check: ## Run cargo check
	cargo check

fmt: ## Format code
	cargo fmt

lint: ## Run clippy lints
	cargo clippy -- -D warnings

test: ## Run tests
	cargo test

clean: ## Clean build artifacts
	cargo clean

## Help

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-16s\033[0m %s\n", $$1, $$2}'

.DEFAULT_GOAL := help

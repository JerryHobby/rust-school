SHELL = /bin/bash   # does not work with zsh
.PHONY: help

help: 	
	@echo "Usage: make [target]"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

all: $(objects) 	
	@echo "Building all objects"
	@cargo build

clean: 	
	@echo "Remove all generated files"
	@rm -rf target

docs: 	
	@cargo doc --no-deps --open

run: 	
	@cargo run

build: 	
	@cargo build

release: 	
	@cargo build --release

test: 	
	@cargo test

update: 	
	@cargo update

version: 	
	@cargo version

commit: 	
	@cargo commit

bench: 	
	@cargo bench

check: 	
	@cargo check

clippy: 	
	@cargo clippy

fmt: 	
	@cargo fmt

bump: 	
	@version=$$(cargo pkgid | cut -d'#' -f2)
	@echo "Current version: " $$(cargo pkgid | cut -d'#' -f2)
	@read -p "Enter new version: " version; \
	updated_version=$$(cargo pkgid | cut -d'#' -f2 | sed -E "s/([0-9]+\.[0-9]+\.[0-9]+)$$/$$version/"); \
	sed -i -E "s/^version = .*/version = \"$$updated_version\"/" Cargo.toml
	@echo "New version saved: $$(cargo pkgid | cut -d'#' -f2)"



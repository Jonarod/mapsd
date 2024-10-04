SHELL := /bin/bash
TARGETS := x86_64-unknown-linux-gnu aarch64-apple-darwin
PROJECT_NAME := $(shell grep 'name\s*=\s*.*' Cargo.toml | head -n 1 | awk -F\" '{print $$2}')

.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}'

clean: ## Clean the project using cargo
	cargo clean

build_darwin: ## Build for aarch64-apple-darwin
	RUSTFLAGS="-Zlocation-detail=none" \
	cargo +nightly build \
			-Z build-std-features=panic_immediate_abort \
			-Z build-std=std,panic_abort \
			--target aarch64-apple-darwin \
			--release
	@# cargo build \
	# 		--target aarch64-apple-darwin \
	# 		--release


build_linux: ## Build for linux
	CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc \
	RUSTFLAGS="-Zlocation-detail=none" \
	cargo +nightly build \
			-Z build-std-features=panic_immediate_abort \
			-Z build-std=std,panic_abort \
			--target x86_64-unknown-linux-gnu \
			--release
	@# CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc \
	# cargo build \
	# 		--target x86_64-unknown-linux-gnu \
	# 		--release


build: ## Build for all targets
	make build_darwin build_linux

lint: ## Lint the project using cargo
	@rustup component add clippy 2> /dev/null
	cargo clippy

release_local_%:
	@mkdir -p ./release/$*
	@for file in $(shell ls -F ./target/$*/release/ | grep "*" | sed "s/\*\$$//" ); do \
		cp "./target/$*/release/$$file" "./release/$*"; \
  done
	@#cp ./src/assets/* "./release/$*"

release_local: ## Copy release binaries to the ./release folder
	@for target in $(TARGETS); do \
		make release_local_$$target; \
	done

get_name: ## Get name of project
	@echo "Project Name: $(PROJECT_NAME)"


release_ssh_%: ## SCP release files to a distant server through SSH
	@scp ./release/x86_64-unknown-linux-gnu/* $*:~/staging/$(PROJECT_NAME)

fmt: ## Format the project using cargo
	@rustup component add rustfmt 2> /dev/null
	cargo fmt


bump: ## Bump the version number
	@echo "Current version is $(shell cargo pkgid | cut -d# -f2)"
	@read -p "Enter new version number: " version; \
	sed -i "" "s/^version = .*/version = \"$$version\"/" Cargo.toml

push: ## Git commit + push
	@read -p "Commit message: " commit_message; \
		git add --all; \
		git commit -m "$$commit_message"
	@git push -u origin master

push_new_release: ## Version bump + Git tags last commit + push
	@make bump

	@if git tag -l v$$(cargo pkgid | cut -d# -f2) | grep -q v$$(cargo pkgid | cut -d# -f2); then \
		echo "This tag version already exists !"; \
		exit 1; \
	fi

	@if ! git diff --quiet --exit-code; then \
		read -p "Some changes are not staged. Do you want to git add them first? (Enter=Yes | n=No) " git_stage; \
		if [ "$$git_stage" = "yes" ] || [ "$$git_stage" = "y" ] || [ -z "$$git_stage" ]; then \
			git add --all; \
		fi \
	fi

	@if ! git diff --cached --quiet --exit-code; then \
		read -p "Staging is not committed. Do you want to git commit first? (Enter=Yes | n=No) " git_commit; \
		if [ "$$git_commit" = "yes" ] || [ "$$git_commit" = "y" ] || [ -z "$$git_commit" ]; then \
			read -p "Commit message: " commit_message; git commit -m "$$commit_message"; \
		fi \
	fi

	@if ! git diff --quiet main..origin/main; then \
		read -p "Local main branch is not synced with Remote main. Do you want to git push? (Enter=Yes | n=No) " git_push; \
		if [ "$$git_push" = "yes" ] || [ "$$git_push" = "y" ] || [ -z "$$git_push" ]; then \
			git push -u origin master; \
		fi \
	fi

	@git tag v$$(cargo pkgid | cut -d# -f2)
	@git push -u origin v$$(cargo pkgid | cut -d# -f2)


git_status:
	@if ! git diff --quiet --exit-code; then \
		echo "Some changes are not staged (git add/rm)"; \
	fi
	@if ! git diff --cached --quiet --exit-code; then \
		echo "Staging is not committed (git commit)"; \
	fi
	@if ! git diff --quiet master..origin/master; then \
		echo "Local master branch is not synced with Remote master (git push)"; \
	fi


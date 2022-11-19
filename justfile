set dotenv-load

@_choose:
	just --choose --unsorted

# Perform all verifications (compile, test, lint, etc.)
verify: lint test

# Watch the source files and run `just verify` when source changes
watch:
	cargo watch -- just verify

# Run the tests
test:
	cargo hack test --feature-powerset

# Run the static code analysis
lint:
	cargo fmt -- --check
	cargo hack clippy --feature-powerset --all-targets 

# Clean up compilation output
clean:
	rm -rf target
	rm -f Cargo.lock
	rm -rf node_modules

# Install cargo dev-tools used by the `verify` recipe (requires rustup to be already installed)
install-dev-tools:
	rustup install stable
	rustup override set stable
	cargo install cargo-hack cargo-watch

# Install a git hook to run tests before every commits
install-git-hooks:
	echo '#!/usr/bin/env sh' > .git/hooks/pre-commit
	echo 'just verify' >> .git/hooks/pre-commit
	chmod +x .git/hooks/pre-commit

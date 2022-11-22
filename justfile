set dotenv-load

@_list:
	just --list --unsorted

year := "2015"
day := "1"

# Fetch your personal input (requires an `AOC_SESSION`)
get-input:
	curl -s \
		-H "Cookie: session=$AOC_SESSION" \
		"https://adventofcode.com/{{year}}/day/{{day}}/input" \
		> "src/day$(printf "%02d" {{day}})/input.txt"

# Perform all verifications (compile, test, lint, etc.)
verify: test lint

# Watch the source files and run `just verify` when source changes
watch:
	cargo watch -- just day={{day}} test-day lint

# Run all the tests
test:
	cargo test

# Run the tests of the current day
test-day:
	cargo test -- day$(printf "%02d" {{day}})

# Run the static code analysis
lint:
	cargo fmt -- --check
	cargo clippy

# Clean up compilation output
clean:
	rm -rf target
	rm -f Cargo.lock
	rm -rf node_modules

# Genearte an empty input and solution for each day
@reset fromDay="1":
	for d in $(seq {{fromDay}} 25); do \
		module=day$(printf "%02d" $d); \
		rm -rf src/$module; \
		cp src/template.rs src/$module.rs; \
		cp -r src/template src/$module; \
		sed -i "s/\"INPUT\"/include_str!\(\"$module\/input.txt\"\)/" src/$module.rs; \
	done

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

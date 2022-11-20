set dotenv-load

@_list:
	just --list --unsorted

year := "2021"

# Fetch your personal input (requires an `AOC_SESSION`)
get-input day:
	curl -s \
		-H "Cookie: session=$AOC_SESSION" \
		"https://adventofcode.com/{{year}}/day/{{day}}/input" \
		> "src/day$(printf "%02d" {{day}})_input.txt"

# Perform all verifications (compile, test, lint, etc.)
verify: test lint

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

# Genearte an empty input and solution for each day
@reset fromDay="1":
	for d in $(seq {{fromDay}} 25); do \
		prefix=day$(printf "%02d" $d); \
		source_file_name="$prefix.rs"; \
		input_file_name="$(echo $prefix)_input.txt"; \
		cp src/template.rs src/$source_file_name; \
		sed -i "s/\"INPUT\"/include_str!\(\"$input_file_name\"\)/" src/$source_file_name; \
		touch src/$input_file_name; \
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

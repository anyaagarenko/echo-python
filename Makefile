package = echo-python
tomls = Cargo.toml mise.toml pyproject.toml rust-toolchain.toml rustfmt.toml
yamlfiles = $(shell find . \( -name "*.yaml" -o -name "*.yml" \) \
	! -path "./.git/*" \
	! -path "./node_modules/*" \
	! -path "./target/*" \
	| sort)

fmt:
	cargo fmt
	cargo clippy --all-targets --fix --allow-dirty --allow-staged
	mise exec -- toml-sort $(tomls)
	mise exec -- yaml-sort --input $(yamlfiles) --lineWidth -1
	make sort-dotfiles

check:
	cargo fmt -- --check
	cargo clippy --all-targets
	mise exec -- toml-sort $(tomls) --check
	mise exec -- yaml-sort --check --input $(yamlfiles) --lineWidth -1
	mise exec -- actionlint
	mise exec -- typos

test:
	cargo test

ci: check test

mr: fmt check test

bump-version:
	sed -E "s/^version = \"[0-9]+\\.[0-9]+\\.[0-9]+\"/version = \"$(version)\"/" Cargo.toml > Cargo.toml.tmp
	mv Cargo.toml.tmp Cargo.toml
	cargo update -p $(package) --precise "$(version)"
	mise exec -- toml-sort $(tomls)

sort-dotfile:
	sort --output $(dotfile) $(dotfile)
	awk "NF" $(dotfile) > $(dotfile).temp && mv $(dotfile).temp $(dotfile)

sort-dotfiles:
	make sort-dotfile dotfile=.gitignore

tomls = Cargo.toml mise.toml pyproject.toml rust-toolchain.toml rustfmt.toml
yamlsort = npx --yes yaml-sort@3.0.0
yamlfiles = $(shell find . \( -name "*.yaml" -o -name "*.yml" \) \
	! -path "./.git/*" \
	! -path "./node_modules/*" \
	! -path "./target/*" \
	| sort)

fmt:
	cargo fmt
	cargo clippy --all-targets --fix --allow-dirty --allow-staged
	mise exec -- toml-sort $(tomls)
	@$(yamlsort) --input $(yamlfiles) --lineWidth -1
	make sort-dotfiles

check:
	cargo fmt -- --check
	cargo clippy --all-targets
	mise exec -- toml-sort $(tomls) --check
	@$(yamlsort) --check --input $(yamlfiles) --lineWidth -1

test:
	cargo test

mr: fmt check test

sort-dotfile:
	sort --output $(dotfile) $(dotfile)
	awk "NF" $(dotfile) > $(dotfile).temp && mv $(dotfile).temp $(dotfile)

sort-dotfiles:
	make sort-dotfile dotfile=.gitignore

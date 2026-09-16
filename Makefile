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
	@set -e; \
	current=$$(sed -nE 's/^version = "(.*)"$$/\1/p' Cargo.toml); \
	if [ -n "$(VERSION)" ]; then new="$(VERSION)"; else \
	  new=$$(echo "$$current" | awk -F. '{print $$1"."$$2"."$$3+1}'); \
	fi; \
	if [ "$$new" = "$$current" ]; then echo "version is already $$current"; exit 1; fi; \
	sed -E "s/^version = \"[0-9]+\\.[0-9]+\\.[0-9]+\"/version = \"$$new\"/" Cargo.toml > Cargo.toml.tmp; \
	mv Cargo.toml.tmp Cargo.toml; \
	cargo update -p $(package) --precise "$$new"; \
	cargo check --locked
	mise exec -- toml-sort $(tomls)
	@printf '%s\n' "commit Cargo.toml and Cargo.lock, push main, then run the pypi workflow"

sort-dotfile:
	sort --output $(dotfile) $(dotfile)
	awk "NF" $(dotfile) > $(dotfile).temp && mv $(dotfile).temp $(dotfile)

sort-dotfiles:
	make sort-dotfile dotfile=.gitignore

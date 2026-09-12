tomls = Cargo.toml mise.toml pyproject.toml rust-toolchain.toml rustfmt.toml
yamlsort = npx --yes yaml-sort@3.0.0
yamlfiles = $(shell find . \( -name "*.yaml" -o -name "*.yml" \) \
	! -path "./.git/*" \
	! -path "./node_modules/*" \
	! -path "./target/*" \
	| sort)

-include .env
export PYPI_PUBLISH_TOKEN

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

publish-pypi:
	@test -n "$(version)" || (echo 'usage: make publish-pypi version=0.1.1' >&2; exit 1)
	@test -n "$(PYPI_PUBLISH_TOKEN)" || (echo 'set PYPI_PUBLISH_TOKEN in .env' >&2; exit 1)
	sed -i '' 's/^version = ".*"/version = "$(version)"/' Cargo.toml
	rm -rf dist
	uvx maturin build --release --locked --sdist --out dist
	docker run --rm --volume "$(CURDIR):/io" ghcr.io/pyo3/maturin build --release --locked --out /io/dist --target x86_64-unknown-linux-gnu
	UV_PUBLISH_TOKEN="$(PYPI_PUBLISH_TOKEN)" uv publish dist/*

sort-dotfile:
	sort --output $(dotfile) $(dotfile)
	awk "NF" $(dotfile) > $(dotfile).temp && mv $(dotfile).temp $(dotfile)

sort-dotfiles:
	make sort-dotfile dotfile=.gitignore

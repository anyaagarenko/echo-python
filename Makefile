tomls = Cargo.toml mise.toml pyproject.toml rustfmt.toml

fmt:
	cargo fmt
	cargo clippy --all-targets --fix --allow-dirty --allow-staged
	mise exec -- toml-sort $(tomls)
	make sort-dotfiles

check:
	cargo fmt -- --check
	cargo clippy --all-targets
	mise exec -- toml-sort $(tomls) --check

test:
	cargo test

mr: fmt check test sort-dotfiles

sort-dotfile:
	sort --output $(dotfile) $(dotfile)
	awk "NF" $(dotfile) > $(dotfile).temp && mv $(dotfile).temp $(dotfile)

sort-dotfiles:
	make sort-dotfile dotfile=.gitignore

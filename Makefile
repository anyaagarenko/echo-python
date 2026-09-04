fmt:
	cargo fmt
	cargo clippy --all-targets --fix --allow-dirty --allow-staged -- --deny warnings
	mise exec -- toml-sort pyproject.toml
	make sort-dotfiles

check:
	cargo fmt -- --check
	cargo clippy --all-targets -- --deny warnings
	mise exec -- toml-sort pyproject.toml --check

test:
	cargo test

mr: fmt check test sort-dotfiles

sort-dotfile:
	sort --output $(dotfile) $(dotfile)
	awk "NF" $(dotfile) > $(dotfile).temp && mv $(dotfile).temp $(dotfile)

sort-dotfiles:
	make sort-dotfile dotfile=.gitignore

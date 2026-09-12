fast style helper for python projects (ruff compatible)


install in a project:

  pip install echo-python
  uv add --dev echo-python
  mise use github:anyaagarenko/echo-python


cli:

  echo-python check .
  mise exec -- echo-python check .
  mise exec -- echo-python check path/to/file.py
  mise exec -- echo-python check --extend-select ECHO001 .
  mise exec -- echo-python check --ignore ECHO001 .
  mise exec -- echo-python check --select ECHO001 .


rules:

  ECHO001 - calls with multiple args must use keywords (self/cls exempt)
  ECHO002 - mixed lists: numbers by value, then words alphabetically
  ECHO003 - numeric lists sorted by value
  ECHO004 - word/name lists sorted alphabetically
  ECHO005 - params one per line when more than one (self/cls not counted)


configure in pyproject.toml:

  [tool.echo-python.lint]
  extend-select, ignore, select


ignore a finding:

  use noqa


development:

  install mise https://github.com/jdx/mise#1-install-mise
  see Makefile

publish to pypi from your machine (no github actions):

  bump version in Cargo.toml
  create an api token at https://pypi.org/manage/account/token/
  uvx maturin build --release --locked --sdist --out dist
  uv publish --token pypi-... dist/*

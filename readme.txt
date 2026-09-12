fast style helper for python projects (ruff compatible)


install in a project:

  pip install echo-python


cli:

  echo-python check .
  echo-python check path/to/file.py
  echo-python check --extend-select ECHO001 .
  echo-python check --ignore ECHO001 .
  echo-python check --select ECHO001 .


rules:

  ECHO001 - use keyword arguments for calls with multiple positional args
            https://github.com/anyaagarenko/echo-python/blob/main/docs/rules/echo-001.txt
  ECHO002 - mixed literals are not sorted (numbers then words)
            https://github.com/anyaagarenko/echo-python/blob/main/docs/rules/echo-002.txt
  ECHO003 - numeric literals are not sorted
            https://github.com/anyaagarenko/echo-python/blob/main/docs/rules/echo-003.txt
  ECHO004 - word literals are not sorted
            https://github.com/anyaagarenko/echo-python/blob/main/docs/rules/echo-004.txt
  ECHO005 - put each parameter on its own line when a function has more than one parameter
            https://github.com/anyaagarenko/echo-python/blob/main/docs/rules/echo-005.txt


configure in pyproject.toml:

  [tool.echo-python.lint]
  extend-select, ignore, select


ignore a finding:

  use noqa


development:

  install mise https://github.com/jdx/mise#1-install-mise
  see Makefile


contributions welcome


publishing:

  bump version in Cargo.toml on main
  github → actions → pypi → run workflow (branch: main)

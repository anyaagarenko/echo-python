style helper for python projects (ruff compatible)


install in a project:

  mise use github:anyaagarenko/echo-python


use as:

  echo-python check .
  echo-python check path/to/file.py


rules:

  ECHO001 - calls with multiple args must use keywords (self/cls exempt)
  ECHO002 - mixed lists: numbers by value, then words alphabetically
  ECHO003 - numeric lists sorted by value
  ECHO004 - word/name lists sorted alphabetically
  ECHO005 - params one per line when more than one (self/cls not counted)


cli:

  echo-python check --select ECHO003 .
  echo-python check --ignore ECHO001 .
  echo-python check --extend-select ECHO002 .


configure in pyproject.toml:

  [tool.echo-python.lint]
  extend-select = ["ECHO002"]
  ignore = ["ECHO001"]
  select = ["ALL"]

  [tool.echo-python.calls-use-kwargs]
  ignore = ["print", "len"]


ignore a finding:

  use noqa


development:

  install mise https://github.com/jdx/mise#1-install-mise
  see Makefile

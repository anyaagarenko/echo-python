style helper for python projects

install in a project:
  pip install echo-python
  echo-python check .
  echo-python check path/to/file.py

rules (ruff-compatible codes; use with [tool.ruff.lint] external = ["ECHO"]):
  ECHO001 - calls with multiple args must use keywords (self/cls exempt)
  ECHO002 - mixed lists: numbers by value, then words alphabetically
  ECHO003 - numeric lists sorted by value
  ECHO004 - word/name lists sorted alphabetically

cli select and ignore (ruff-like; default ALL; prefixes ok: ECHO, ECHO0):
  echo-python check --select ECHO003 .
  echo-python check --ignore ECHO001 .
  echo-python check --extend-select ECHO002 .
  --select replaces file select; --ignore / --extend-select merge (CLI wins)

configure in pyproject.toml:
  [tool.echo-python.lint]
  select = ["ALL"]
  ignore = ["ECHO001"]
  extend-select = ["ECHO002"]

  [tool.echo-python.calls-use-kwargs]
  ignore = ["print", "len"]

ignore a finding:
  f(1, 2)  # noqa: ECHO001
  x = ["a", 1]  # noqa: ECHO002
  x = [2, 1]  # noqa: ECHO003
  x = ["b", "a"]  # noqa: ECHO004

development (hack on the linter itself):
  see Makefile

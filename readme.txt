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
  ECHO002 - mixed list is not sorted (numbers then words)
  ECHO003 - numbers list is not sorted
  ECHO004 - words list is not sorted
  ECHO005 - put each parameter on its own line when a function has more than one parameter


configure in pyproject.toml:

  [tool.echo-python.lint]
  extend-select = ["ECHO002"]
  ignore = ["ECHO001"]
  select = ["ALL"]

  [tool.echo-python.echo001]
  ignore = ["pytest.mark.parametrize", "pytest.param"]


ignore a finding:

  f(1, 2)  # noqa: ECHO001


development:

  install mise https://github.com/jdx/mise#1-install-mise
  see Makefile


contributions welcome


publishing:

  bump version in Cargo.toml on main
  github → actions → pypi → run workflow (branch: main)

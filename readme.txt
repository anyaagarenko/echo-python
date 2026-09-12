style helper for python projects (ruff compatible)


install in a project:

  mise use github:anyaagarenko/echo-python


use as:

  mise exec -- echo-python check .
  mise exec -- echo-python check path/to/file.py


rules:

  ECHO001 - use keyword arguments for calls with multiple positional args
  ECHO002 - mixed list is not sorted (numbers then words)
  ECHO003 - numbers list is not sorted
  ECHO004 - words list is not sorted
  ECHO005 - put each parameter on its own line when a function has more than one parameter


cli:

  mise exec -- echo-python check --select ECHO003 .
  mise exec -- echo-python check --ignore ECHO001 .
  mise exec -- echo-python check --extend-select ECHO002 .


configure in pyproject.toml:

  [tool.echo-python.lint]
  extend-select = ["ECHO002"]
  ignore = ["ECHO001"]
  select = ["ALL"]

  [tool.echo-python.calls-use-kwargs]
  ignore = ["path", "parametrize", "pytest.param", "register", "spy"]


ignore a finding:

  use noqa


development:

  install mise https://github.com/jdx/mise#1-install-mise
  see Makefile

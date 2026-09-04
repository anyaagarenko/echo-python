style helper for python projects

install in a project:
  pip install echo-python
  echo-python check .
  echo-python check path/to/file.py

rules:
  echo-calls-use-kwargs - calls with multiple args must use keywords (self/cls exempt)
  echo-numbers-list-sorted - numeric lists sorted by value
  echo-words-list-sorted - word/name lists sorted alphabetically
  echo-mixed-list-sorted - mixed lists: numbers by value, then words alphabetically

ignore a finding:
  f(1, 2)  # noqa: echo-calls-use-kwargs
  x = [2, 1]  # noqa: echo-numbers-list-sorted
  x = ["b", "a"]  # noqa: echo-words-list-sorted
  x = ["a", 1]  # noqa: echo-mixed-list-sorted

configure in pyproject.toml:
  [tool.echo-python.calls-use-kwargs]
  ignore = ["print", "len"]

development (hack on the linter itself):
  see Makefile

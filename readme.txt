style helper for python projects

install in a project:
  pip install echo-python
  echo-python check .
  echo-python check path/to/file.py

rules:
  echo-numbers-list-sorted - numeric lists sorted by value
  echo-words-list-sorted - word/name lists sorted alphabetically
  echo-mixed-list-sorted - mixed lists: numbers by value, then words alphabetically

ignore a finding:
  x = [2, 1]  # noqa: echo-numbers-list-sorted
  x = ["b", "a"]  # noqa: echo-words-list-sorted
  x = ["a", 1]  # noqa: echo-mixed-list-sorted

development (hack on the linter itself):
  see Makefile

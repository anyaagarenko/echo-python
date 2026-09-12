# Agent guidelines

- Comments in code are forbidden.
- After every change, run linters and tests for the files and functionality that were affected.
- In Makefiles, put a target in `.PHONY` only if it overrides something systemic or matches a folder/file that already exists in the repo—not prophylactically, and not for custom names like `mr`.
- Keep `mod.rs` and other module-root files thin: type/struct definitions, module declarations, and re-exports only—no logic.
- One file, one concern. When a module grows, split by role (constructors, methods, iterators, reporting, parsing)—not by stuffing more into the same file.
- Multiple `impl` blocks for the same type across sibling files are fine when that keeps concerns apart.
- Keep functions short (prefer under ~15 lines); extract a step rather than grow a body.
- Prefer `pub(crate)` or private visibility; `pub` only for the crate’s public API.
- Do not name items with a leading `_` or `__` (no `_foo`, `__bar`, and no “unused” names silenced by underscore prefixes).
- Put unit tests in the same `.rs` file as the code they cover; keep end-to-end rule checks under `tests/`.
- Name tests as short behavior scenarios (e.g. `unsorted_numbers_are_reported`).
- Prefer many small tests over one test with many assertions.
- In `assert_eq!`, put the expected value first: `assert_eq!(expected, actual)`.
- Use long-form CLI flags only.
- Before every commit, run `make mr`.
- One action = one commit; push to the open PR after each change.
- If on `main`, create a short kebab-case branch (one or two words) first.
- Keep the PR description updated after every commit.

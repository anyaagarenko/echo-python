# Agent guidelines

- Comments in code are forbidden.
- After every change, run linters and tests for the files and functionality that were affected.
- In Makefiles, put a target in `.PHONY` only if it overrides something systemic or matches a folder/file that already exists in the repo—not prophylactically, and not for custom names like `mr`.
- Keep `mod.rs` files thin: module declarations and re-exports only—no logic.
- Use long-form CLI flags only.

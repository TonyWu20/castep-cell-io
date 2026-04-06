# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build all workspace crates
cargo build

# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p castep_cell_io
cargo test -p castep_cell_fmt

# Run a single test by name
cargo test -p castep_cell_io test_name

# Lint
cargo clippy

# Lint with auto-fix
cargo clippy --fix
```

Use `fd`, `sd` and `rg` to replace `find`, `sed` and `grep`

## Code style and principles

- **Modular architecture** strictly following the **Single Responsibility Principle (SRP)**: each module, struct, and function has exactly one reason to change.
- **Test-Driven Development (TDD)** is mandatory for all core logic:
  - Write failing tests first
  - Implement the minimal code to make tests pass
  - Refactor while keeping tests green
- Unit test coverage target: ≥ 85% for `lib/` modules
- Integration tests for CLI and plugin entry points
- **Builder pattern** **must** be used for all complex struct creation.
  - Use the **`bon`** crate (`#[derive(bon::Builder)]`) for all non-trivial structs.
  - This provides clean, ergonomic builders with method chaining and automatic collection helpers.
- **Functional programming style** must be used as much as possible:
  - Prefer iterators (`iter()`, `map`, `filter`, `filter_map`, `flat_map`, `fold`, `collect`, etc.) over imperative `for` loops.
  - Minimize mutable state; favor immutable transformations and method chaining.
  - Use higher-order functions and combinators wherever they improve clarity and reduce side effects.

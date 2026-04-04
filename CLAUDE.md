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
cargo test -p castep_cell_data

# Run a single test by name
cargo test -p castep_cell_io test_name

# Lint
cargo clippy

# Lint with auto-fix
cargo clippy --fix
```

The example `.cell` file used by tests is `Mg2SiO4_Cr_1.cell` at the workspace root — tests in `castep_cell_io` reference it by relative path and must be run from that directory (which `cargo test` handles automatically).

## Code style and principles

- **Modular architecture** strictly following the **Single Responsibility Principle (SRP)**: each module, struct, and function has exactly one reason to change.
- **Test-Driven Development (TDD)** is mandatory for all core logic:
  - Write failing tests first
  - Implement the minimal code to make tests pass
  - Refactor while keeping tests green
- Unit test coverage target: ≥ 85% for `lib/` modules
- Integration tests for CLI and plugin entry points
- **Builder pattern** **must** be used for all complex struct creation.
  - Use the **`derive_builder`** crate (`#[derive(Builder)]`) for all non-trivial structs.
  - This provides clean, ergonomic, immutable builders with method chaining.
- **Functional programming style** must be used as much as possible:
  - Prefer iterators (`iter()`, `map`, `filter`, `filter_map`, `flat_map`, `fold`, `collect`, etc.) over imperative `for` loops.
  - Minimize mutable state; favor immutable transformations and method chaining.
  - Use higher-order functions and combinators wherever they improve clarity and reduce side effects.

## Workspace Structure

This is a Cargo workspace with two crates:

- **`castep_cell_io`** — Core parsing, formatting, and trait definitions for the `.cell`/`.param` format. No serde dependency.
- **`castep_cell_data`** — Concrete CASTEP data types for both `.cell` and `.param` formats, built on top of `castep_cell_io`.

## Architecture: `.cell`/`.param` Format Pipeline

### 1. Parsing (`castep_cell_io/src/parser.rs`)

`parse_cell_file(input: &str) -> Result<Vec<Cell<'a>>, ...>` uses `chumsky` to parse text into an intermediate representation (IR). Both `.cell` and `.param` formats are **case-insensitive** and share three entry kinds:

```rust
enum Cell<'a> {
    KeyValue(&'a str, CellValue<'a>),   // e.g. `TASK : SinglePoint`
    Block(&'a str, Vec<CellValue<'a>>), // e.g. %BLOCK POSITIONS_FRAC ... %ENDBLOCK
    Flag(&'a str),                       // e.g. `SYMMETRY_GENERATE` (bare keyword = true)
}

enum CellValue<'a> {
    Null, Bool(bool), Str(&'a str), String(String),
    UInt(u32), Int(i32), Float(f64),
    Array(Vec<CellValue<'a>>),  // one line of a block
}
```

Each block line becomes a `CellValue::Array`. The parser strips inline comments (`#` or `!`).

### 2. Deserialization (`castep_cell_io/src/parse.rs`)

Four traits form a hierarchy from leaf to file level. Each domain type is responsible for knowing how to parse itself:

| Trait           | Implemented by                       | Role                                                       |
| --------------- | ------------------------------------ | ---------------------------------------------------------- |
| `FromCellValue` | Scalars, unit enums, keyword enums   | Parse a single `CellValue` into a Rust type                |
| `FromBlock`     | Block structs (e.g. `PositionsFrac`) | Parse a block's row slice; provides `from_cells` for free  |
| `FromKeyValue`  | Key-value types (e.g. `Task`)        | Parse the value at a known key; returns `Option` if absent |
| `FromCellFile`  | Top-level aggregator structs         | Assemble a complete file struct from the full token slice  |

**Entry point:**

```rust
pub fn parse<T: FromCellFile>(input: &str) -> CResult<T>
```

**Query helpers** (`castep_cell_io/src/query.rs`): `find_block`, `find_keyvalue`, `has_flag`, `row_as_f64_n`, `value_as_f64`, `value_as_str`, etc. Used inside `FromBlock`/`FromKeyValue` implementations.

### 3. Serialization (`castep_cell_io/src/format.rs`)

`to_string(cell: &Cell) -> String` and `to_string_many(cells: &[Cell]) -> String` format IR back to text. Floats use `{v:20.16}` fixed-width format; strings are right-aligned to a padded width bracket to match CASTEP's style. `CellValue::Null` formats as empty (used to skip optional fields).

**Serialization traits** (defined in `castep_cell_io/src/lib.rs`):

```rust
pub trait ToCellValue { fn to_cell_value(&self) -> CellValue; }
pub trait ToCell      { fn to_cell(&self) -> Cell; }
pub trait ToCellFile  { fn to_cell_file(&self) -> Vec<Cell>; }
```

Domain types implement these manually. `None` optional fields produce `CellValue::Null` via `.as_ref().map(...).unwrap_or(CellValue::Null)`, which the formatter silently skips.

### 4. Domain Types (`castep_cell_data`)

All CASTEP `.cell` and `.param` types live here. The established pattern for a **keyword/enum type** (fully migrated):

```rust
impl FromCellValue for Task {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "singlepoint" => Ok(Self::SinglePoint),
            other => Err(Error::Message(format!("unknown Task: {other}"))),
        }
    }
}

impl FromKeyValue for Task {
    const KEY_NAME: &'static str = "TASK";
    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> { Self::from_cell_value(value) }
}
```

**Migration status**: Leaf types (unit enums, keyword enums) in `castep_cell_data/src/units/` and simple `.param` key-value types already implement the new `FromCellValue`/`FromKeyValue` traits. Complex block types with multi-row parsing (e.g. `LatticeCart`, `PositionsFrac`) still carry old `#[serde(from = "...")]` intermediate-repr patterns and are **pending migration** to `FromBlock`.

## Known Incomplete Areas

- Top-level aggregator structs (`CellDocument`, `CastepParam`) do not yet exist in `castep_cell_data` — `FromCellFile` / `ToCellFile` have no implementations yet
- No `derive_builder` usage on `.cell` or `.param` domain types yet — public construction requires struct literals (fields must be `pub`) or explicit constructors
- No in-place mutation helpers or "wither" methods for modifying existing structs
- Integer support caps at `i32`/`u32` by design (`.cell` does not use larger integers)

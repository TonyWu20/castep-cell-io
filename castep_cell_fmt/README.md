# castep-cell-fmt

Low-level parsing and formatting backend for CASTEP `.cell` / `.param` file formats.

This crate provides the intermediate representation (IR), parser, formatter, and trait hierarchy used by [`castep-cell-io`](https://crates.io/crates/castep-cell-io). It is intended for crate authors implementing new CASTEP domain types, not for end users.

## Architecture

```
.cell / .param text
        │
        ▼
  parse_cell_file()  →  Vec<Cell<'_>>   (IR tokens)
        │
        ▼
  FromCellFile / FromBlock / FromKeyValue / FromCellValue  (deserialization traits)
        │
        ▼
  Domain types  (in castep-cell-io)
        │
        ▼
  ToCellFile / ToCell / ToCellValue  (serialization traits)
        │
        ▼
  to_string_many()  →  formatted text
```

## IR Types

```rust
enum Cell<'a> {
    KeyValue(&'a str, CellValue<'a>),   // e.g. TASK : SinglePoint
    Block(&'a str, Vec<CellValue<'a>>), // e.g. %BLOCK POSITIONS_FRAC ... %ENDBLOCK
    Flag(&'a str),                       // e.g. SYMMETRY_GENERATE
}

enum CellValue<'a> {
    Null, Bool(bool), Str(&'a str), String(String),
    UInt(u32), Int(i32), Float(f64),
    Array(Vec<CellValue<'a>>),  // one line of a block
}
```

## Trait Hierarchy

| Trait           | Role                                                  |
|-----------------|-------------------------------------------------------|
| `FromCellValue` | Parse a single `CellValue` (scalars, enums)           |
| `FromBlock`     | Parse a block's row slice into a struct               |
| `FromKeyValue`  | Parse the value at a known key; returns `Option`      |
| `FromCellFile`  | Assemble a top-level struct from the full token slice |
| `ToCellValue`   | Serialize a type to a `CellValue`                     |
| `ToCell`        | Serialize a type to a `Cell`                          |
| `ToCellFile`    | Serialize a type to `Vec<Cell>`                       |

## Implementing a new keyword type

```rust
use castep_cell_fmt::{CellValue, CResult, parse::{FromCellValue, FromKeyValue}, query::value_as_str};

#[derive(Debug)]
pub enum Task { SinglePoint, GeometryOptimization }

impl FromCellValue for Task {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "singlepoint" => Ok(Self::SinglePoint),
            "geometryoptimization" | "geometryoptimisation" => Ok(Self::GeometryOptimization),
            other => Err(castep_cell_fmt::Error::Message(format!("unknown Task: {other}"))),
        }
    }
}

impl FromKeyValue for Task {
    const KEY_NAME: &'static str = "TASK";
    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}
```

## License

MIT

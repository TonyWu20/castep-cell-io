# castep-cell-parser

Parser for `.cell` of `Castep`.

## Development

Currently, the parser can seek and parse the two required block entries in the `.cell` regardless of their appearance orders in the file: lattice parameters (`LATTICE_CART` or `LATTICE_ABC`) and ionic positions (`POSITIONS_FRAC` or `POSITIONS_ABS`).
`POSITIONS_XXX_INTERMEDIATE` and `POSITIONS_XXX_PRODUCT` are supported if feature `TS` is turned on.

More keywords and data are supported in the future.

## Usage

Add the crate by `cargo`

```shell
cargo add castep-cell-parser
```

In your code:

```rust
use std::{fs, path::Path};
use castep_cell_parser::{CellParser, CellDocument};

let path = Path::new("SAC_GDY_V.cell");
let input = fs::read_to_string(path).unwrap();
let mut cell_parser = CellParser::from_str(&input);
let cell_doc: CellDocument = cell_parser.parse().unwrap();
```

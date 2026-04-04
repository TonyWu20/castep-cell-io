# castep-cell-io

Parse, build, and format CASTEP `.cell` and `.param` input files in Rust.

## Quick start

```toml
[dependencies]
castep-cell-io = "0.3"
castep-cell-fmt = "0.1"
```

```rust
use castep_cell_fmt::{parse, ToCellFile, format::to_string_many_spaced};
use castep_cell_io::CellDocument;

let input = std::fs::read_to_string("structure.cell").unwrap();
let doc: CellDocument = parse(&input).unwrap();

// Round-trip back to text
let output = to_string_many_spaced(&doc.to_cell_file());
println!("{output}");
```

## Building a `.cell` document

```rust
use castep_cell_io::{CellDocumentBuilder, Lattice, Positions};
use castep_cell_io::cell::lattice_param::LatticeCart;
use castep_cell_io::cell::positions::positions_frac::PositionsFrac;

let doc = CellDocumentBuilder::default()
    .lattice(Lattice::Cart(lattice))
    .positions(Positions::Frac(positions))
    .build()
    .unwrap();
```

## Crate layout

| Module | Contents |
|--------|----------|
| `cell::lattice_param` | `LatticeCart`, `LatticeABC` |
| `cell::positions` | `PositionsFrac`, `PositionsAbs` |
| `cell::bz_sampling_kpoints` | K-point types |
| `cell::species` | Species mass, pot, LCAO, Hubbard U |
| `cell::constraints` | Ionic / cell / nonlinear constraints |
| `param` | All `.param` keyword types |
| `units` | Unit enums (`LengthUnit`, `EnergyUnit`, …) |

The parsing and formatting engine lives in [`castep-cell-fmt`](https://crates.io/crates/castep-cell-fmt).

## License

MIT

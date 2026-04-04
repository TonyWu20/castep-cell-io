With my implementation of `Deserializer`, I tested deserialization into a
custom struct representing the `LATTICE_CART` block of `.cell`.
The first line of `LATTICE_CART` block is optionally one `CellValue::Str` representing the
length unit.

```rust
    #[derive(Debug, Deserialize)]
    /// Block
    #[serde(from = "LatticeCartRepr")]
    struct LatticeCart {
        unit: Option<LengthUnit>,
        a: [f64; 3],
        b: [f64; 3],
        c: [f64; 3],
    }
    #[derive(Debug, Clone, Copy, Hash, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
    // #[serde(untagged)]
    // #[serde(rename_all = "lowercase")]
    pub enum LengthUnit {
        #[serde(rename = "bohr")]
        Bohr,
        #[serde(alias = "a0")]
        BohrA0,
        #[serde(rename = "m")]
        Meter,
        #[serde(rename = "cm")]
        Centimeter,
        #[serde(rename = "nm")]
        Nanometer,
        #[default]
        #[serde(rename = "ang")]
        Ang,
    }
    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum LatticeCartRepr {
        Essential([[f64; 3]; 3]),
        WithUnit(Vec<LengthUnit>, Vec<[f64; 3]>),
    }
    impl From<LatticeCartRepr> for LatticeCart {
        fn from(value: LatticeCartRepr) -> Self {
            match value {
                LatticeCartRepr::Essential(items) => LatticeCart {
                    unit: None,
                    a: items[0],
                    b: items[1],
                    c: items[2],
                },
                LatticeCartRepr::WithUnit(unit, items) => LatticeCart {
                    unit: Some(unit[0]),
                    a: items[0],
                    b: items[1],
                    c: items[2],
                },
            }
        }
    }
```

```rust
    /// Parsed Intermediate representations as
Cell::Block("LATTICE_CART", vec![
    CellValue::Array(vec![CellValue::Str("bohr")]),
    CellValue::Array(vec![CellValue::Float(1.0), CellValue::Float(0.0), CellValue::Float(0.0)]),
    CellValue::Array(vec![CellValue::Float(0.0), CellValue::Float(1.0), CellValue::Float(0.0)]),
    CellValue::Array(vec![CellValue::Float(0.0), CellValue::Float(0.0), CellValue::Float(1.0)]),
])
```

However, error message is:

```rust
"data did not match any variant of untagged enum LatticeCartRepr"
```

Meanwhile, this line of test code passed:

```rust
let unit = Vec::<LengthUnit>::deserialize(&mut CellValueDeserializer::new(
        &CellValue::Array(vec![CellValue::Str("bohr")]),
));

```

Please help me find out the reason.

This is the `serde` backend implementation of a custom format `.cell` used by a DFT calculation software `CASTEP`.
The deserialization and serialization are based on the intermediate representations, `Cell<'a>` and `CellValue<'a>`.
`Cell<'a>` holds the top-level data format in `.cell`, and `CellValue<'a>` holds the primitive types in `.cell`.

```rust
#![allow(dead_code)]
mod de;
mod error;
mod parser;
mod ser;

pub use de::{from_str, from_tokens};
pub use error::{CResult, Error};
pub use parser::parse_cell_file;
pub use parser::rich_error;
pub use ser::to_string;

// Intermediate representation for parsed data
#[derive(Debug, Clone)]
pub enum Cell<'a> {
    KeyValue(&'a str, CellValue<'a>),
    Block(&'a str, Vec<CellValue<'a>>),
    Flag(&'a str),
}

impl<'a> Cell<'a> {
    pub fn key(&self) -> &str {
        match self {
            Cell::KeyValue(key, _cell_value) => key,
            Cell::Block(name, _cell_value) => name,
            Cell::Flag(flag) => flag,
        }
    }
}

pub trait ToCellValue {
    fn to_cell_value(&self) -> CellValue;
}

pub trait ToCell {
    fn to_cell(&self) -> Cell;
}

pub trait ToCellFile {
    fn to_cell_file(&self) -> Vec<Cell>;
}

#[derive(Debug, Clone)]
pub enum CellValue<'a> {
    Null,
    Bool(bool),
    Str(&'a str),
    String(String),
    UInt(u32),
    Int(i32),
    Float(f64),
    Array(Vec<CellValue<'a>>),
}
```

Now, I create another crate `castep_cell_data` to define the corresponding entries in `.cell` used by `CASTEP`.
I am following the documentations from `CASTEP` to implement the equivalent structs and enums in `rust`.
Currently the codes are like this:

```rust
mod units {
    pub use length_units::LengthUnit;

    mod length_units {
        use castep_cell_serde::{CellValue, ToCellValue};
        use serde::{Deserialize, Serialize};
        #[derive(
            Debug,
            Clone,
            Copy,
            Hash,
            Serialize,
            Deserialize,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Default,
        )]
        #[serde(rename_all = "lowercase")]
        #[serde(rename = "LENGTH_UNIT")]
        /// Description
        /// This keyword specifies the units in which lengths will be reported.
        ///
        /// The available units and their associated identifiers are:
        ///
        /// Unit	Identifier
        /// Bohr	bohr, a0
        /// Meter	m
        /// Centimeter	cm
        /// Nanometer	nm
        /// Å	ang
        /// Default
        /// ang
        pub enum LengthUnit {
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

        impl ToCellValue for LengthUnit {
            fn to_cell_value(&self) -> CellValue {
                match self {
                    LengthUnit::Bohr => CellValue::String("bohr".into()),
                    LengthUnit::BohrA0 => CellValue::String("a0".into()),
                    LengthUnit::Meter => CellValue::String("m".into()),
                    LengthUnit::Centimeter => CellValue::String("cm".into()),
                    LengthUnit::Nanometer => CellValue::String("nm".into()),
                    LengthUnit::Ang => CellValue::String("ang".into()),
                }
            }
        }
    }
}
mod lattice_param {
    use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
    use serde::{Deserialize, Serialize};

    use crate::units::LengthUnit;

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
    /// Lattice vectors
    /// This data block contains the cell lattice vectors in Cartesian coordinates. It has the following format:
    /// %BLOCK LATTICE_CART
    /// [units]
    ///     R1x R1y R1z
    ///     R2x R2y R2z
    ///     R3x R3y R3z
    /// %ENDBLOCK LATTICE_CART
    /// Where R1x is the x-component of the first lattice vector, R2y is the y-component of the second lattice vector, and so on.
    /// [units] specifies the units in which the lattice vectors are defined. If no units are given, the default of Å is used.
    #[serde(from = "LatticeCartRepr")]
    pub struct LatticeCart {
        unit: Option<LengthUnit>,
        a: [f64; 3],
        b: [f64; 3],
        c: [f64; 3],
    }

    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum LatticeCartRepr {
        Essential([[f64; 3]; 3]),
        WithUnit([LengthUnit; 1], [f64; 3], [f64; 3], [f64; 3]),
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
                LatticeCartRepr::WithUnit(unit, a, b, c) => LatticeCart {
                    unit: Some(unit[0]),
                    a,
                    b,
                    c,
                },
            }
        }
    }

    impl ToCell for LatticeCart {
        fn to_cell(&self) -> Cell {
            Cell::Block(
                "LATTICE_CART",
                vec![
                    match &self.unit {
                        Some(u) => CellValue::Array(vec![u.to_cell_value()]),
                        None => CellValue::Null,
                    },
                    CellValue::Array(self.a.into_iter().map(CellValue::Float).collect()),
                    CellValue::Array(self.b.into_iter().map(CellValue::Float).collect()),
                    CellValue::Array(self.c.into_iter().map(CellValue::Float).collect()),
                ],
            )
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
    #[serde(from = "LatticeABCRepr")]
    /// This data block contains the cell lattice vectors in terms of the lattice vector magnitudes and the angles
    /// between them (a, b, c, α, β and γ). It has the following format:
    /// %BLOCK LATTICE_ABC
    /// [units]
    ///     Ra Rb Rc
    ///     Rα Rβ Rγ
    /// %ENDBLOCK LATTICE_ABC
    /// Where Ra is the value of a, Rγ is the value of γ, and so on.
    ///
    /// [units] specifies the units in which the lattice vector magnitudes are defined. If no units are given, the default of Å is used. Angles should be specified in degrees.
    pub struct LatticeABC {
        unit: Option<LengthUnit>,
        abc: [f64; 3],
        angles: [f64; 3],
    }
    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum LatticeABCRepr {
        Essential([[f64; 3]; 2]),
        WithUnit([LengthUnit; 1], [f64; 3], [f64; 3]),
    }

    impl From<LatticeABCRepr> for LatticeABC {
        fn from(value: LatticeABCRepr) -> Self {
            match value {
                LatticeABCRepr::Essential(items) => LatticeABC {
                    unit: None,
                    abc: items[0],
                    angles: items[1],
                },
                LatticeABCRepr::WithUnit(unit, abc, angles) => LatticeABC {
                    unit: Some(unit[0]),
                    abc,
                    angles,
                },
            }
        }
    }

    impl ToCell for LatticeABC {
        fn to_cell(&self) -> Cell {
            Cell::Block(
                "LATTICE_ABC",
                vec![
                    CellValue::Array(vec![match &self.unit {
                        Some(u) => u.to_cell_value(),
                        None => CellValue::Null,
                    }]),
                    CellValue::Array(self.abc.into_iter().map(CellValue::Float).collect()),
                    CellValue::Array(self.angles.into_iter().map(CellValue::Float).collect()),
                ],
            )
        }
    }

    #[cfg(test)]
    mod test_lattice_param {
        use castep_cell_serde::{ToCell, from_str, to_string};
        use serde::{Deserialize, Serialize};

        use crate::lattice_param::LatticeABC;

        use super::LatticeCart;

        #[test]
        fn lattice_param_serde() {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileCart {
                lattice_cart: LatticeCart,
            }
            let block_cart_str = r#"%BLOCK LATTICE_CART
bohr
      10.182880152352300       0.000000000000000       0.000000000000000
       0.000000000000000       5.969867637928440       0.000000000000000
       0.000000000000000       0.000000000000000       4.750940602435009
%ENDBLOCK LATTICE_CART
"#;
            let cell_file = dbg!(from_str::<CellFileCart>(block_cart_str).unwrap());
            println!("{}", to_string(&cell_file.lattice_cart.to_cell()).unwrap());
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileABC {
                lattice_abc: LatticeABC,
            }
            let block_abc_str = r#"%BLOCK LATTICE_ABC
bohr
      10.182880152352300       5.969867637928440       4.750940602435009
      90.000000000000000      90.000000000000000      90.000000000000000
%ENDBLOCK LATTICE_ABC
"#;
            let cell_file = dbg!(from_str::<CellFileABC>(block_abc_str).unwrap());
            println!("{}", to_string(&cell_file.lattice_abc.to_cell()).unwrap());
        }
    }
}
mod kpoints_list {
    use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
    #[serde(transparent)]
    /// Represents the KPOINTS_LIST block.
    ///
    /// Contains a list of k-points and their weights for Brillouin zone sampling.
    /// Format:
    /// %BLOCK KPOINTS_LIST
    ///    R1i R1j R1k R1w
    ///    R2i R2j R2k R2w
    /// ...
    /// %ENDBLOCK KPOINTS_LIST
    pub struct KpointsList {
        kpts: Vec<Kpoint>,
    }

    impl ToCell for KpointsList {
        fn to_cell(&self) -> Cell {
            Cell::Block(
                "KPOINTS_LIST",
                self.kpts
                    .iter()
                    .map(|kpt| kpt.to_cell_value())
                    .collect::<Vec<CellValue>>(),
            )
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
    /// A line of block `KpointsList`
    /// The first three entries on a line are the fractional positions of the
    /// k-point relative to the reciprocal space lattice vectors.
    ///
    /// The final entry on a line is the weight of the k-point relative to the
    /// others specified. The sum of the weights must be equal to 1.
    #[serde(from = "[f64;4]")]
    pub struct Kpoint {
        coord: [f64; 3],
        weight: f64,
    }

    impl PartialOrd for Kpoint {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.coord
                .iter()
                .zip(other.coord.iter())
                .find_map(|(a, b)| {
                    let diff = a - b;
                    if diff.abs() > 1e-6 {
                        a.partial_cmp(b)
                    } else {
                        None
                    }
                })
                .or(Some(std::cmp::Ordering::Equal))
        }
    }

    impl From<[f64; 4]> for Kpoint {
        fn from(value: [f64; 4]) -> Self {
            Kpoint {
                coord: value[0..3].try_into().unwrap(),
                weight: value[3],
            }
        }
    }

    impl ToCellValue for Kpoint {
        fn to_cell_value(&self) -> CellValue {
            CellValue::Array(
                self.coord
                    .into_iter()
                    .map(CellValue::Float)
                    .chain([CellValue::Float(self.weight)])
                    .collect(),
            )
        }
    }

    #[cfg(test)]
    mod test_kpoints_list {
        use castep_cell_serde::{ToCell, from_str, to_string};
        use serde::{Deserialize, Serialize};

        use crate::bz_sampling_kpoints::kpoints_list::KpointsList;

        #[test]
        fn kpoints_list_serde() {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFile {
                kpoints_list: KpointsList,
            }
            let block_kpt_str = r#"%BLOCK KPOINTS_LIST
   0.0000000000000000    0.2500000000000000    0.3333333333333333       0.333333333333333
   0.0000000000000000    0.2500000000000000    0.0000000000000000       0.333333333333333
   0.0000000000000000    0.2500000000000000   -0.3333333333333333       0.333333333333333
%ENDBLOCK KPOINTS_LIST
"#;
            let cell_file = dbg!(from_str::<CellFile>(block_kpt_str).unwrap());
            println!("{}", to_string(&cell_file.kpoints_list.to_cell()).unwrap());
        }
    }
}
```

Based on these provided information, have you understood my approach? Can you help me write the `rust` code implementation
when I give you the documentation from `CASTEP`?

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

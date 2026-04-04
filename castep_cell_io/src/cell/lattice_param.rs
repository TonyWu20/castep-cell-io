use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, parse::{FromBlock, FromCellValue}, CResult, Error, query::row_as_f64_n};

use crate::units::LengthUnit;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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
pub struct LatticeCart {
    pub unit: Option<LengthUnit>,
    pub a: [f64; 3],
    pub b: [f64; 3],
    pub c: [f64; 3],
}

impl FromBlock for LatticeCart {
    const BLOCK_NAME: &'static str = "LATTICE_CART";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.is_empty() {
            return Err(Error::Message("LATTICE_CART block is empty".into()));
        }

        let (unit, data_start) = if let CellValue::Array(arr) = &rows[0] {
            if arr.len() == 1 {
                if let Ok(u) = LengthUnit::from_cell_value(&arr[0]) {
                    (Some(u), 1)
                } else {
                    (None, 0)
                }
            } else {
                (None, 0)
            }
        } else {
            (None, 0)
        };

        if rows.len() < data_start + 3 {
            return Err(Error::Message(
                "LATTICE_CART requires 3 data rows".into(),
            ));
        }

        let a = row_as_f64_n::<3>(&rows[data_start])?;
        let b = row_as_f64_n::<3>(&rows[data_start + 1])?;
        let c = row_as_f64_n::<3>(&rows[data_start + 2])?;

        Ok(Self { unit, a, b, c })
    }
}

impl ToCell for LatticeCart {
    fn to_cell(&self) -> Cell {
        let mut block_content = Vec::new();
        if let Some(u) = &self.unit {
            block_content.push(CellValue::Array(vec![u.to_cell_value()]));
        }
        block_content.push(CellValue::Array(self.a.into_iter().map(CellValue::Float).collect()));
        block_content.push(CellValue::Array(self.b.into_iter().map(CellValue::Float).collect()));
        block_content.push(CellValue::Array(self.c.into_iter().map(CellValue::Float).collect()));
        Cell::Block("LATTICE_CART", block_content)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LatticeABC {
    pub unit: Option<LengthUnit>,
    pub abc: [f64; 3],
    pub angles: [f64; 3],
}

impl FromBlock for LatticeABC {
    const BLOCK_NAME: &'static str = "LATTICE_ABC";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.is_empty() {
            return Err(Error::Message("LATTICE_ABC block is empty".into()));
        }

        let (unit, data_start) = if let CellValue::Array(arr) = &rows[0] {
            if arr.len() == 1 {
                if let Ok(u) = LengthUnit::from_cell_value(&arr[0]) {
                    (Some(u), 1)
                } else {
                    (None, 0)
                }
            } else {
                (None, 0)
            }
        } else {
            (None, 0)
        };

        if rows.len() < data_start + 2 {
            return Err(Error::Message(
                "LATTICE_ABC requires 2 data rows".into(),
            ));
        }

        let abc = row_as_f64_n::<3>(&rows[data_start])?;
        let angles = row_as_f64_n::<3>(&rows[data_start + 1])?;

        Ok(Self { unit, abc, angles })
    }
}

impl ToCell for LatticeABC {
    fn to_cell(&self) -> Cell {
        let mut block_content = Vec::new();
        if let Some(u) = &self.unit {
            block_content.push(CellValue::Array(vec![u.to_cell_value()]));
        }
        block_content.push(CellValue::Array(self.abc.into_iter().map(CellValue::Float).collect()));
        block_content.push(CellValue::Array(self.angles.into_iter().map(CellValue::Float).collect()));
        Cell::Block("LATTICE_ABC", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // LatticeCart tests
    #[test]
    fn test_lattice_cart_no_unit() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Float(5.0), CellValue::Float(0.0), CellValue::Float(0.0)]),
            CellValue::Array(vec![CellValue::Float(0.0), CellValue::Float(5.0), CellValue::Float(0.0)]),
            CellValue::Array(vec![CellValue::Float(0.0), CellValue::Float(0.0), CellValue::Float(5.0)]),
        ];
        let result = LatticeCart::from_block_rows(&rows).unwrap();
        assert_eq!(result.unit, None);
        assert_eq!(result.a, [5.0, 0.0, 0.0]);
        assert_eq!(result.b, [0.0, 5.0, 0.0]);
        assert_eq!(result.c, [0.0, 0.0, 5.0]);
    }

    #[test]
    fn test_lattice_cart_with_unit_ang() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("ang")]),
            CellValue::Array(vec![CellValue::Float(5.0), CellValue::Float(0.0), CellValue::Float(0.0)]),
            CellValue::Array(vec![CellValue::Float(0.0), CellValue::Float(5.0), CellValue::Float(0.0)]),
            CellValue::Array(vec![CellValue::Float(0.0), CellValue::Float(0.0), CellValue::Float(5.0)]),
        ];
        let result = LatticeCart::from_block_rows(&rows).unwrap();
        assert_eq!(result.unit, Some(LengthUnit::Ang));
    }

    #[test]
    fn test_lattice_cart_with_unit_bohr() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("bohr")]),
            CellValue::Array(vec![CellValue::Float(9.45), CellValue::Float(0.0), CellValue::Float(0.0)]),
            CellValue::Array(vec![CellValue::Float(0.0), CellValue::Float(9.45), CellValue::Float(0.0)]),
            CellValue::Array(vec![CellValue::Float(0.0), CellValue::Float(0.0), CellValue::Float(9.45)]),
        ];
        let result = LatticeCart::from_block_rows(&rows).unwrap();
        assert_eq!(result.unit, Some(LengthUnit::Bohr));
    }

    #[test]
    fn test_lattice_cart_empty_rows() {
        let rows = vec![];
        let result = LatticeCart::from_block_rows(&rows);
        assert!(result.is_err());
    }

    #[test]
    fn test_lattice_cart_insufficient_rows() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Float(5.0), CellValue::Float(0.0), CellValue::Float(0.0)]),
            CellValue::Array(vec![CellValue::Float(0.0), CellValue::Float(5.0), CellValue::Float(0.0)]),
        ];
        let result = LatticeCart::from_block_rows(&rows);
        assert!(result.is_err());
    }

    #[test]
    fn test_lattice_cart_to_cell_no_unit() {
        let lattice = LatticeCart {
            unit: None,
            a: [5.0, 0.0, 0.0],
            b: [0.0, 5.0, 0.0],
            c: [0.0, 0.0, 5.0],
        };
        let cell = lattice.to_cell();
        match cell {
            Cell::Block(name, values) => {
                assert_eq!(name, "LATTICE_CART");
                assert_eq!(values.len(), 3);
            }
            _ => panic!("Expected Cell::Block"),
        }
    }

    // LatticeABC tests
    #[test]
    fn test_lattice_abc_no_unit() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Float(5.0), CellValue::Float(5.0), CellValue::Float(5.0)]),
            CellValue::Array(vec![CellValue::Float(90.0), CellValue::Float(90.0), CellValue::Float(90.0)]),
        ];
        let result = LatticeABC::from_block_rows(&rows).unwrap();
        assert_eq!(result.unit, None);
        assert_eq!(result.abc, [5.0, 5.0, 5.0]);
        assert_eq!(result.angles, [90.0, 90.0, 90.0]);
    }

    #[test]
    fn test_lattice_abc_with_unit() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("ang")]),
            CellValue::Array(vec![CellValue::Float(5.0), CellValue::Float(5.0), CellValue::Float(5.0)]),
            CellValue::Array(vec![CellValue::Float(90.0), CellValue::Float(90.0), CellValue::Float(90.0)]),
        ];
        let result = LatticeABC::from_block_rows(&rows).unwrap();
        assert_eq!(result.unit, Some(LengthUnit::Ang));
    }

    #[test]
    fn test_lattice_abc_empty_rows() {
        let rows = vec![];
        let result = LatticeABC::from_block_rows(&rows);
        assert!(result.is_err());
    }

    #[test]
    fn test_lattice_abc_insufficient_rows() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Float(5.0), CellValue::Float(5.0), CellValue::Float(5.0)]),
        ];
        let result = LatticeABC::from_block_rows(&rows);
        assert!(result.is_err());
    }

    #[test]
    fn test_lattice_abc_to_cell() {
        let lattice = LatticeABC {
            unit: Some(LengthUnit::Ang),
            abc: [5.0, 5.0, 5.0],
            angles: [90.0, 90.0, 90.0],
        };
        let cell = lattice.to_cell();
        match cell {
            Cell::Block(name, values) => {
                assert_eq!(name, "LATTICE_ABC");
                assert_eq!(values.len(), 3);
            }
            _ => panic!("Expected Cell::Block"),
        }
    }
}


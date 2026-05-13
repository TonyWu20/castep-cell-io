use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, parse::{FromBlock, FromCellValue}, CResult, Error, query::row_as_f64_n};

use crate::units::LengthUnit;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, bon::Builder)]
/// Lattice vectors in Cartesian coordinates.
///
/// This data block contains the cell lattice vectors in Cartesian coordinates.
/// **Row-major order**: Row 1 = lattice vector **A** (field `.a`),
/// Row 2 = lattice vector **B** (field `.b`), Row 3 = lattice vector **C** (field `.c`).
///
/// Format in CASTEP `.cell` file:
/// ```text
/// %BLOCK LATTICE_CART
/// [units]
///     A11 A12 A13        → field `.a`
///     B21 B22 B23        → field `.b`
///     C31 C32 C33        → field `.c`
/// %ENDBLOCK LATTICE_CART
/// ```
/// Where each row is one lattice vector. `[units]` specifies the units (defaults to Å).
///
/// # Converting to matrix types
///
/// The row-major order means downstream users constructing a 3×3 matrix
/// should place `.a` as row 1, `.b` as row 2, `.c` as row 3:
/// ```text
///     | a[0]  a[1]  a[2] |   ← lattice vector A
/// M = | b[0]  b[1]  b[2] |   ← lattice vector B
///     | c[0]  c[1]  c[2] |   ← lattice vector C
/// ```
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
    fn to_cell(&self) -> Cell<'_> {
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, bon::Builder)]
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
    fn to_cell(&self) -> Cell<'_> {
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

    #[test]
    fn test_lattice_cart_round_trip() {
        // Verify row-major convention: Row 1 = .a, Row 2 = .b, Row 3 = .c
        // across the full to_cell → from_block_rows cycle, including unit.
        for unit in [None, Some(LengthUnit::Ang), Some(LengthUnit::Bohr)] {
            let original = LatticeCart {
                unit,
                a: [5.0, 1.0, 2.0],
                b: [3.0, 5.0, 4.0],
                c: [6.0, 7.0, 5.0],
            };
            let cell = original.to_cell();
            let rows = match &cell {
                Cell::Block(_, r) => r.as_slice(),
                _ => panic!("Expected Cell::Block"),
            };
            let parsed = LatticeCart::from_block_rows(rows).unwrap();
            assert_eq!(parsed.unit, original.unit, "unit mismatch for {unit:?}");
            assert_eq!(parsed.a, original.a, "a vector mismatch");
            assert_eq!(parsed.b, original.b, "b vector mismatch");
            assert_eq!(parsed.c, original.c, "c vector mismatch");
        }
    }

    // Builder pattern tests for LatticeCart
    #[test]
    fn test_lattice_cart_builder_basic() {
        let lattice = LatticeCart::builder()
            .a([5.0, 0.0, 0.0])
            .b([0.0, 5.0, 0.0])
            .c([0.0, 0.0, 5.0])
            .build();

        assert_eq!(lattice.unit, None);
        assert_eq!(lattice.a, [5.0, 0.0, 0.0]);
        assert_eq!(lattice.b, [0.0, 5.0, 0.0]);
        assert_eq!(lattice.c, [0.0, 0.0, 5.0]);
    }

    #[test]
    fn test_lattice_cart_builder_with_unit() {
        let lattice = LatticeCart::builder()
            .a([5.0, 0.0, 0.0])
            .b([0.0, 5.0, 0.0])
            .c([0.0, 0.0, 5.0])
            .unit(LengthUnit::Ang)
            .build();

        assert_eq!(lattice.unit, Some(LengthUnit::Ang));
        assert_eq!(lattice.a, [5.0, 0.0, 0.0]);
    }

    #[test]
    fn test_lattice_cart_builder_without_unit() {
        let lattice = LatticeCart::builder()
            .a([9.45, 0.0, 0.0])
            .b([0.0, 9.45, 0.0])
            .c([0.0, 0.0, 9.45])
            .build();

        assert_eq!(lattice.unit, None);
        assert_eq!(lattice.a, [9.45, 0.0, 0.0]);
        assert_eq!(lattice.b, [0.0, 9.45, 0.0]);
        assert_eq!(lattice.c, [0.0, 0.0, 9.45]);
    }

    // Builder pattern tests for LatticeABC
    #[test]
    fn test_lattice_abc_builder_basic() {
        let lattice = LatticeABC::builder()
            .abc([5.0, 5.0, 5.0])
            .angles([90.0, 90.0, 90.0])
            .build();

        assert_eq!(lattice.unit, None);
        assert_eq!(lattice.abc, [5.0, 5.0, 5.0]);
        assert_eq!(lattice.angles, [90.0, 90.0, 90.0]);
    }

    #[test]
    fn test_lattice_abc_builder_with_unit() {
        let lattice = LatticeABC::builder()
            .abc([5.0, 5.0, 5.0])
            .angles([90.0, 90.0, 90.0])
            .unit(LengthUnit::Bohr)
            .build();

        assert_eq!(lattice.unit, Some(LengthUnit::Bohr));
        assert_eq!(lattice.abc, [5.0, 5.0, 5.0]);
        assert_eq!(lattice.angles, [90.0, 90.0, 90.0]);
    }

    #[test]
    fn test_lattice_abc_builder_without_unit() {
        let lattice = LatticeABC::builder()
            .abc([10.0, 10.0, 10.0])
            .angles([120.0, 90.0, 90.0])
            .build();

        assert_eq!(lattice.unit, None);
        assert_eq!(lattice.abc, [10.0, 10.0, 10.0]);
        assert_eq!(lattice.angles, [120.0, 90.0, 90.0]);
    }
}


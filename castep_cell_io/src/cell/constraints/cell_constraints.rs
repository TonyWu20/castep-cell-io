use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromBlock, CResult, Error, query::value_as_u32};

/// Represents the constraints on lattice parameters (lengths and angles) during relaxation/MD.
///
/// Consists of two sets of indices:
/// - One for the lattice vector magnitudes (a, b, c)
/// - One for the lattice angles (alpha, beta, gamma)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellConstraints {
    /// Constraints for lattice vector magnitudes [a, b, c].
    /// 0 = fixed, positive integer = free to vary, shared integers = linked values.
    pub lengths: [u32; 3],
    /// Constraints for lattice angles [alpha, beta, gamma] in degrees.
    /// 0 = fixed, positive integer = free to vary, shared integers = linked values.
    pub angles: [u32; 3],
}

impl FromBlock for CellConstraints {
    const BLOCK_NAME: &'static str = "CELL_CONSTRAINTS";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.len() < 2 {
            return Err(castep_cell_fmt::Error::Message(
                "CELL_CONSTRAINTS requires at least 2 rows".into(),
            ));
        }

        let lengths = match &rows[0] {
            CellValue::Array(arr) if arr.len() >= 3 => [
                value_as_u32(&arr[0])?,
                value_as_u32(&arr[1])?,
                value_as_u32(&arr[2])?,
            ],
            _ => return Err(castep_cell_fmt::Error::Message(
                "CELL_CONSTRAINTS lengths row must be an array of 3 values".into(),
            )),
        };

        let angles = match &rows[1] {
            CellValue::Array(arr) if arr.len() >= 3 => [
                value_as_u32(&arr[0])?,
                value_as_u32(&arr[1])?,
                value_as_u32(&arr[2])?,
            ],
            _ => return Err(castep_cell_fmt::Error::Message(
                "CELL_CONSTRAINTS angles row must be an array of 3 values".into(),
            )),
        };

        Ok(CellConstraints { lengths, angles })
    }
}

impl ToCell for CellConstraints {
    fn to_cell(&self) -> Cell {
        let block_content = [
            CellValue::Array(self.lengths.iter().map(|&v| CellValue::UInt(v)).collect()),
            CellValue::Array(self.angles.iter().map(|&v| CellValue::UInt(v)).collect()),
        ]
        .to_vec();

        Cell::Block("CELL_CONSTRAINTS", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_fmt::{CellValue, parse::FromBlock};

    #[test]
    fn test_cell_constraints_basic() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::UInt(1),
                CellValue::UInt(1),
                CellValue::UInt(0),
            ]),
            CellValue::Array(vec![
                CellValue::UInt(0),
                CellValue::UInt(0),
                CellValue::UInt(0),
            ]),
        ];
        let result = CellConstraints::from_block_rows(&rows).unwrap();
        assert_eq!(result.lengths, [1, 1, 0]);
        assert_eq!(result.angles, [0, 0, 0]);
    }

    #[test]
    fn test_cell_constraints_all_free() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::UInt(1),
                CellValue::UInt(2),
                CellValue::UInt(3),
            ]),
            CellValue::Array(vec![
                CellValue::UInt(1),
                CellValue::UInt(2),
                CellValue::UInt(3),
            ]),
        ];
        let result = CellConstraints::from_block_rows(&rows).unwrap();
        assert_eq!(result.lengths, [1, 2, 3]);
        assert_eq!(result.angles, [1, 2, 3]);
    }

    #[test]
    fn test_cell_constraints_all_fixed() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::UInt(0),
                CellValue::UInt(0),
                CellValue::UInt(0),
            ]),
            CellValue::Array(vec![
                CellValue::UInt(0),
                CellValue::UInt(0),
                CellValue::UInt(0),
            ]),
        ];
        let result = CellConstraints::from_block_rows(&rows).unwrap();
        assert_eq!(result.lengths, [0, 0, 0]);
        assert_eq!(result.angles, [0, 0, 0]);
    }

    #[test]
    fn test_cell_constraints_insufficient_rows() {
        let rows = vec![CellValue::Array(vec![
            CellValue::UInt(1),
            CellValue::UInt(1),
            CellValue::UInt(0),
        ])];
        assert!(CellConstraints::from_block_rows(&rows).is_err());
    }

    #[test]
    fn test_cell_constraints_empty() {
        assert!(CellConstraints::from_block_rows(&[]).is_err());
    }

    #[test]
    fn test_cell_constraints_insufficient_elements_lengths() {
        let rows = vec![
            CellValue::Array(vec![CellValue::UInt(1), CellValue::UInt(1)]),
            CellValue::Array(vec![
                CellValue::UInt(0),
                CellValue::UInt(0),
                CellValue::UInt(0),
            ]),
        ];
        assert!(CellConstraints::from_block_rows(&rows).is_err());
    }

    #[test]
    fn test_cell_constraints_insufficient_elements_angles() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::UInt(1),
                CellValue::UInt(1),
                CellValue::UInt(0),
            ]),
            CellValue::Array(vec![CellValue::UInt(0), CellValue::UInt(0)]),
        ];
        assert!(CellConstraints::from_block_rows(&rows).is_err());
    }

    #[test]
    fn test_cell_constraints_block_name() {
        assert_eq!(CellConstraints::BLOCK_NAME, "CELL_CONSTRAINTS");
    }
}


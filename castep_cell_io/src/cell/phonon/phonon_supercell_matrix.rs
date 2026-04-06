use castep_cell_fmt::{Cell, CellValue, ToCell, parse::FromBlock, CResult, Error, query::row_as_i32_n};

#[derive(Debug, Clone, Copy, PartialEq, Eq, bon::Builder)]
/// 3×3 integer matrix for phonon supercell
/// %BLOCK PHONON_SUPERCELL_MATRIX
///     m11 m12 m13
///     m21 m22 m23
///     m31 m32 m33
/// %ENDBLOCK PHONON_SUPERCELL_MATRIX
pub struct PhononSupercellMatrix {
    pub matrix: [[i32; 3]; 3],
}

impl FromBlock for PhononSupercellMatrix {
    const BLOCK_NAME: &'static str = "PHONON_SUPERCELL_MATRIX";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.len() < 3 {
            return Err(Error::Message(
                "PHONON_SUPERCELL_MATRIX requires 3 data rows".into(),
            ));
        }

        let row1 = row_as_i32_n::<3>(&rows[0])?;
        let row2 = row_as_i32_n::<3>(&rows[1])?;
        let row3 = row_as_i32_n::<3>(&rows[2])?;

        Ok(Self {
            matrix: [row1, row2, row3],
        })
    }
}

impl ToCell for PhononSupercellMatrix {
    fn to_cell(&self) -> Cell<'_> {
        Cell::Block(
            "PHONON_SUPERCELL_MATRIX",
            vec![
                CellValue::Array(self.matrix[0].into_iter().map(CellValue::Int).collect()),
                CellValue::Array(self.matrix[1].into_iter().map(CellValue::Int).collect()),
                CellValue::Array(self.matrix[2].into_iter().map(CellValue::Int).collect()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phonon_supercell_matrix_parse() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Int(2), CellValue::Int(0), CellValue::Int(0)]),
            CellValue::Array(vec![CellValue::Int(0), CellValue::Int(2), CellValue::Int(0)]),
            CellValue::Array(vec![CellValue::Int(0), CellValue::Int(0), CellValue::Int(2)]),
        ];
        let result = PhononSupercellMatrix::from_block_rows(&rows).unwrap();
        assert_eq!(result.matrix[0], [2, 0, 0]);
        assert_eq!(result.matrix[1], [0, 2, 0]);
        assert_eq!(result.matrix[2], [0, 0, 2]);
    }

    #[test]
    fn test_phonon_supercell_matrix_parse_general() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Int(1), CellValue::Int(1), CellValue::Int(0)]),
            CellValue::Array(vec![CellValue::Int(-1), CellValue::Int(1), CellValue::Int(0)]),
            CellValue::Array(vec![CellValue::Int(0), CellValue::Int(0), CellValue::Int(1)]),
        ];
        let result = PhononSupercellMatrix::from_block_rows(&rows).unwrap();
        assert_eq!(result.matrix[0], [1, 1, 0]);
        assert_eq!(result.matrix[1], [-1, 1, 0]);
        assert_eq!(result.matrix[2], [0, 0, 1]);
    }

    #[test]
    fn test_phonon_supercell_matrix_empty_rows() {
        let rows = vec![];
        let result = PhononSupercellMatrix::from_block_rows(&rows);
        assert!(result.is_err());
    }

    #[test]
    fn test_phonon_supercell_matrix_insufficient_rows() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Int(2), CellValue::Int(0), CellValue::Int(0)]),
            CellValue::Array(vec![CellValue::Int(0), CellValue::Int(2), CellValue::Int(0)]),
        ];
        let result = PhononSupercellMatrix::from_block_rows(&rows);
        assert!(result.is_err());
    }

    #[test]
    fn test_phonon_supercell_matrix_to_cell() {
        let matrix = PhononSupercellMatrix {
            matrix: [[2, 0, 0], [0, 2, 0], [0, 0, 2]],
        };
        let cell = matrix.to_cell();
        match cell {
            Cell::Block(name, values) => {
                assert_eq!(name, "PHONON_SUPERCELL_MATRIX");
                assert_eq!(values.len(), 3);
            }
            _ => panic!("Expected Cell::Block"),
        }
    }

    #[test]
    fn test_phonon_supercell_matrix_roundtrip() {
        let original = PhononSupercellMatrix {
            matrix: [[1, 1, 0], [-1, 1, 0], [0, 0, 1]],
        };
        let cell = original.to_cell();
        if let Cell::Block(_, values) = cell {
            let parsed = PhononSupercellMatrix::from_block_rows(&values).unwrap();
            assert_eq!(parsed, original);
        } else {
            panic!("Expected Cell::Block");
        }
    }
}

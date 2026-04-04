use castep_cell_fmt::{CellValue, parse::{FromBlock, FromCellValue}, CResult, Error};
use super::{AtomHubbardU, HubbardU, HubbardUUnit};

impl FromBlock for HubbardU {
    const BLOCK_NAME: &'static str = "HUBBARD_U";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.is_empty() {
            return Ok(Self {
                unit: None,
                atom_u_values: Vec::new(),
            });
        }

        let (unit, data_start) = if let CellValue::Array(arr) = &rows[0] {
            if arr.len() == 1 {
                if let Ok(u) = HubbardUUnit::from_cell_value(&arr[0]) {
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

        let atom_u_values = rows[data_start..]
            .iter()
            .map(|row| AtomHubbardU::from_block_rows(&[row.clone()]))
            .collect::<CResult<Vec<_>>>()?;

        Ok(Self {
            unit,
            atom_u_values,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_fmt::CellValue;

    #[test]
    fn test_hubbard_u_empty() {
        let result = HubbardU::from_block_rows(&[]).unwrap();
        assert!(result.unit.is_none());
        assert_eq!(result.atom_u_values.len(), 0);
    }

    #[test]
    fn test_hubbard_u_with_unit() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("eV")]),
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::Float(5.0),
            ]),
        ];
        let result = HubbardU::from_block_rows(&rows).unwrap();
        assert!(result.unit.is_some());
        assert_eq!(result.atom_u_values.len(), 1);
    }

    #[test]
    fn test_hubbard_u_without_unit() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::Float(5.0),
            ]),
        ];
        let result = HubbardU::from_block_rows(&rows).unwrap();
        assert!(result.unit.is_none());
        assert_eq!(result.atom_u_values.len(), 1);
    }

    #[test]
    fn test_hubbard_u_multiple_entries() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("eV")]),
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::Float(5.0),
            ]),
            CellValue::Array(vec![
                CellValue::Str("O"),
                CellValue::Float(3.0),
            ]),
        ];
        let result = HubbardU::from_block_rows(&rows).unwrap();
        assert_eq!(result.atom_u_values.len(), 2);
    }

    #[test]
    fn test_block_name() {
        assert_eq!(HubbardU::BLOCK_NAME, "HUBBARD_U");
    }
}

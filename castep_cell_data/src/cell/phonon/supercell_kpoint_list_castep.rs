use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromBlock, FromCellValue, CResult, query::value_as_f64};

#[derive(Debug, Clone, Copy, PartialEq)]
/// A line of block `SupercellKpointListCastep`
/// The first three entries on a line are the fractional positions of the
/// k-point relative to the reciprocal space lattice vectors.
///
/// The final entry on a line is the weight of the k-point relative to the
/// others specified. The sum of the weights must be equal to 1.
pub struct SupercellKpointListCastepEntry {
    pub coord: [f64; 3],
    pub weight: f64,
}

impl FromCellValue for SupercellKpointListCastepEntry {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 4 => {
                let coord = [
                    value_as_f64(&arr[0])?,
                    value_as_f64(&arr[1])?,
                    value_as_f64(&arr[2])?,
                ];
                let weight = value_as_f64(&arr[3])?;
                Ok(SupercellKpointListCastepEntry { coord, weight })
            }
            _ => Err(castep_cell_io::Error::Message(
                "SupercellKpointListCastepEntry must be an array of 4 floats".into(),
            )),
        }
    }
}

impl ToCellValue for SupercellKpointListCastepEntry {
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

/// Represents the SUPERCELL_KPOINT_LIST_CASTEP block.
///
/// Contains a list of k-points and their weights for supercell calculations.
/// # Format:
/// %BLOCK SUPERCELL_KPOINT_LIST_CASTEP
///    R1i R1j R1k R1w
///    R2i R2j R2k R2w
/// ...
/// %ENDBLOCK SUPERCELL_KPOINT_LIST_CASTEP
#[derive(Debug, Clone, PartialEq)]
pub struct SupercellKpointListCastep {
    pub kpoints: Vec<SupercellKpointListCastepEntry>,
}

impl FromBlock for SupercellKpointListCastep {
    const BLOCK_NAME: &'static str = "SUPERCELL_KPOINT_LIST_CASTEP";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        let kpoints = rows
            .iter()
            .map(SupercellKpointListCastepEntry::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;
        Ok(SupercellKpointListCastep { kpoints })
    }
}

impl ToCell for SupercellKpointListCastep {
    fn to_cell(&self) -> Cell {
        Cell::Block(
            "SUPERCELL_KPOINT_LIST_CASTEP",
            self.kpoints
                .iter()
                .map(|entry| entry.to_cell_value())
                .collect::<Vec<CellValue>>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supercell_kpoint_list_castep_entry_from_cell_value() {
        let val = CellValue::Array(vec![
            CellValue::Float(0.5),
            CellValue::Float(0.25),
            CellValue::Float(0.0),
            CellValue::Float(1.0),
        ]);
        let entry = SupercellKpointListCastepEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.coord, [0.5, 0.25, 0.0]);
        assert_eq!(entry.weight, 1.0);
    }

    #[test]
    fn test_supercell_kpoint_list_castep_entry_insufficient_elements() {
        let val = CellValue::Array(vec![
            CellValue::Float(0.5),
            CellValue::Float(0.25),
            CellValue::Float(0.0),
        ]);
        assert!(SupercellKpointListCastepEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_supercell_kpoint_list_castep_entry_to_cell_value() {
        let entry = SupercellKpointListCastepEntry {
            coord: [0.5, 0.25, 0.0],
            weight: 1.0,
        };
        let val = entry.to_cell_value();
        match val {
            CellValue::Array(arr) => {
                assert_eq!(arr.len(), 4);
                assert_eq!(arr[3], CellValue::Float(1.0));
            }
            _ => panic!("Expected Array"),
        }
    }

    #[test]
    fn test_supercell_kpoint_list_castep_multiple_entries() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(0.5),
            ]),
            CellValue::Array(vec![
                CellValue::Float(0.5),
                CellValue::Float(0.5),
                CellValue::Float(0.5),
                CellValue::Float(0.5),
            ]),
        ];
        let result = SupercellKpointListCastep::from_block_rows(&rows).unwrap();
        assert_eq!(result.kpoints.len(), 2);
        assert_eq!(result.kpoints[0].weight, 0.5);
        assert_eq!(result.kpoints[1].weight, 0.5);
    }

    #[test]
    fn test_supercell_kpoint_list_castep_empty() {
        let result = SupercellKpointListCastep::from_block_rows(&[]).unwrap();
        assert_eq!(result.kpoints.len(), 0);
    }

    #[test]
    fn test_supercell_kpoint_list_castep_block_name() {
        assert_eq!(SupercellKpointListCastep::BLOCK_NAME, "SUPERCELL_KPOINT_LIST_CASTEP");
    }

    #[test]
    fn test_supercell_kpoint_list_castep_to_cell() {
        let kpts = SupercellKpointListCastep {
            kpoints: vec![SupercellKpointListCastepEntry {
                coord: [0.0, 0.0, 0.0],
                weight: 1.0,
            }],
        };
        let cell = kpts.to_cell();
        match cell {
            Cell::Block(name, values) => {
                assert_eq!(name, "SUPERCELL_KPOINT_LIST_CASTEP");
                assert_eq!(values.len(), 1);
            }
            _ => panic!("Expected Block"),
        }
    }
}

use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromBlock, FromCellValue, CResult, query::value_as_f64};

#[derive(Debug, Clone, Copy, PartialEq)]
/// A line of block `MagresKpointsList`
/// The first three entries on a line are the fractional positions of the
/// k-point relative to the reciprocal space lattice vectors.
///
/// The final entry on a line is the weight of the k-point relative to the
/// others specified. The sum of the weights must be equal to 1.
pub struct MagresKpointsListEntry {
    pub coord: [f64; 3],
    pub weight: f64,
}

impl FromCellValue for MagresKpointsListEntry {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 4 => {
                let coord = [
                    value_as_f64(&arr[0])?,
                    value_as_f64(&arr[1])?,
                    value_as_f64(&arr[2])?,
                ];
                let weight = value_as_f64(&arr[3])?;
                Ok(MagresKpointsListEntry { coord, weight })
            }
            _ => Err(castep_cell_io::Error::Message(
                "MagresKpointsListEntry must be an array of 4 floats".into(),
            )),
        }
    }
}

impl ToCellValue for MagresKpointsListEntry {
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

/// Represents the MAGRES_KPOINTS_LIST block.
///
/// Contains a list of k-points and their weights for magnetic resonance calculations.
/// # Format:
/// %BLOCK MAGRES_KPOINTS_LIST
///    R1i R1j R1k R1w
///    R2i R2j R2k R2w
/// ...
/// %ENDBLOCK MAGRES_KPOINTS_LIST
#[derive(Debug, Clone, PartialEq)]
pub struct MagresKpointsList {
    pub kpoints: Vec<MagresKpointsListEntry>,
}

impl FromBlock for MagresKpointsList {
    const BLOCK_NAME: &'static str = "MAGRES_KPOINTS_LIST";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        let kpoints = rows
            .iter()
            .map(MagresKpointsListEntry::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;
        Ok(MagresKpointsList { kpoints })
    }
}

impl ToCell for MagresKpointsList {
    fn to_cell(&self) -> Cell {
        Cell::Block(
            "MAGRES_KPOINTS_LIST",
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
    fn test_magres_kpoints_list_entry_from_cell_value() {
        let val = CellValue::Array(vec![
            CellValue::Float(0.5),
            CellValue::Float(0.25),
            CellValue::Float(0.0),
            CellValue::Float(1.0),
        ]);
        let entry = MagresKpointsListEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.coord, [0.5, 0.25, 0.0]);
        assert_eq!(entry.weight, 1.0);
    }

    #[test]
    fn test_magres_kpoints_list_entry_insufficient_elements() {
        let val = CellValue::Array(vec![
            CellValue::Float(0.5),
            CellValue::Float(0.25),
            CellValue::Float(0.0),
        ]);
        assert!(MagresKpointsListEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_magres_kpoints_list_entry_to_cell_value() {
        let entry = MagresKpointsListEntry {
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
    fn test_magres_kpoints_list_multiple_entries() {
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
        let result = MagresKpointsList::from_block_rows(&rows).unwrap();
        assert_eq!(result.kpoints.len(), 2);
        assert_eq!(result.kpoints[0].weight, 0.5);
        assert_eq!(result.kpoints[1].weight, 0.5);
    }

    #[test]
    fn test_magres_kpoints_list_empty() {
        let result = MagresKpointsList::from_block_rows(&[]).unwrap();
        assert_eq!(result.kpoints.len(), 0);
    }

    #[test]
    fn test_magres_kpoints_list_block_name() {
        assert_eq!(MagresKpointsList::BLOCK_NAME, "MAGRES_KPOINTS_LIST");
    }

    #[test]
    fn test_magres_kpoints_list_to_cell() {
        let kpts = MagresKpointsList {
            kpoints: vec![MagresKpointsListEntry {
                coord: [0.0, 0.0, 0.0],
                weight: 1.0,
            }],
        };
        let cell = kpts.to_cell();
        match cell {
            Cell::Block(name, values) => {
                assert_eq!(name, "MAGRES_KPOINTS_LIST");
                assert_eq!(values.len(), 1);
            }
            _ => panic!("Expected Block"),
        }
    }
}

use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromBlock, CResult, query::value_as_f64};

/// Represents a single point entry within the BS_KPOINT_PATH block.
///
/// Each entry contains three fractional k-point coordinates relative to the reciprocal space lattice vectors.
/// Format: <x> <y> <z>
#[derive(Debug, Clone, Copy, PartialEq, bon::Builder)]
pub struct BsKpointPathEntry {
    /// Fractional k-point coordinates [x, y, z].
    pub coord: [f64; 3],
}

impl FromCellValue for BsKpointPathEntry {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 3 => {
                let coord = [
                    value_as_f64(&arr[0])?,
                    value_as_f64(&arr[1])?,
                    value_as_f64(&arr[2])?,
                ];
                Ok(BsKpointPathEntry { coord })
            }
            _ => Err(castep_cell_fmt::Error::Message(
                "BsKpointPathEntry must be an array of 3 floats".into(),
            )),
        }
    }
}

impl ToCellValue for BsKpointPathEntry {
    /// Converts the entry into a `CellValue::Array` representing one line of the block.
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Array(
            self.coord
                .into_iter()
                .map(CellValue::Float)
                .collect(),
        )
    }
}

/// Represents the BS_KPOINT_PATH block.
///
/// Contains a list of k-points that define a path through reciprocal space for band structure calculations.
/// Format:
/// %BLOCK BS_KPOINT_PATH
/// R1i R1j R1k
/// R2i R2j R2k
/// ...
/// %ENDBLOCK BS_KPOINT_PATH
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct BsKpointPath {
    /// The list of k-point entries.
    #[builder(default)]
    pub points: Vec<BsKpointPathEntry>,
}

impl FromBlock for BsKpointPath {
    const BLOCK_NAME: &'static str = "BS_KPOINT_PATH";
    const BLOCK_ALIASES: &'static [&'static str] = &["BS_KPOINTS_PATH"];

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        let points = rows
            .iter()
            .map(BsKpointPathEntry::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;
        Ok(BsKpointPath { points })
    }
}

impl ToCell for BsKpointPath {
    /// Converts the block into the intermediate `Cell` representation for serialization.
    fn to_cell(&self) -> Cell<'_> {
        Cell::Block(
            "BS_KPOINT_PATH",
            self.points
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
    fn test_bs_kpoint_path_entry_from_cell_value() {
        let val = CellValue::Array(vec![
            CellValue::Float(0.3333333333),
            CellValue::Float(0.3750000000),
            CellValue::Float(0.3333333333),
        ]);
        let entry = BsKpointPathEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.coord[0], 0.3333333333);
        assert_eq!(entry.coord[1], 0.3750000000);
        assert_eq!(entry.coord[2], 0.3333333333);
    }

    #[test]
    fn test_bs_kpoint_path_entry_insufficient_elements() {
        let val = CellValue::Array(vec![
            CellValue::Float(0.5),
            CellValue::Float(0.25),
        ]);
        assert!(BsKpointPathEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_bs_kpoint_path_entry_too_many_elements() {
        let val = CellValue::Array(vec![
            CellValue::Float(0.5),
            CellValue::Float(0.25),
            CellValue::Float(0.0),
            CellValue::Float(1.0),
        ]);
        assert!(BsKpointPathEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_bs_kpoint_path_entry_to_cell_value() {
        let entry = BsKpointPathEntry {
            coord: [0.3333333333, 0.3750000000, 0.3333333333],
        };
        let val = entry.to_cell_value();
        match val {
            CellValue::Array(arr) => {
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], CellValue::Float(0.3333333333));
            }
            _ => panic!("Expected Array"),
        }
    }

    #[test]
    fn test_bs_kpoint_path_multiple_entries() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Float(0.3333333333),
                CellValue::Float(0.3750000000),
                CellValue::Float(0.3333333333),
            ]),
            CellValue::Array(vec![
                CellValue::Float(0.3333333333),
                CellValue::Float(0.3750000000),
                CellValue::Float(0.0000000000),
            ]),
        ];
        let result = BsKpointPath::from_block_rows(&rows).unwrap();
        assert_eq!(result.points.len(), 2);
    }

    #[test]
    fn test_bs_kpoint_path_empty() {
        let result = BsKpointPath::from_block_rows(&[]).unwrap();
        assert_eq!(result.points.len(), 0);
    }

    #[test]
    fn test_bs_kpoint_path_block_name() {
        assert_eq!(BsKpointPath::BLOCK_NAME, "BS_KPOINT_PATH");
    }

    #[test]
    fn test_bs_kpoint_path_to_cell() {
        let path = BsKpointPath {
            points: vec![BsKpointPathEntry {
                coord: [0.3333333333, 0.3750000000, 0.3333333333],
            }],
        };
        let cell = path.to_cell();
        match cell {
            Cell::Block(name, values) => {
                assert_eq!(name, "BS_KPOINT_PATH");
                assert_eq!(values.len(), 1);
            }
            _ => panic!("Expected Block"),
        }
    }

    // Builder pattern tests
    #[test]
    fn test_bs_kpoint_path_entry_builder() {
        let entry = BsKpointPathEntry::builder()
            .coord([0.5, 0.5, 0.5])
            .build();
        assert_eq!(entry.coord, [0.5, 0.5, 0.5]);
    }

    #[test]
    fn test_bs_kpoint_path_builder_empty() {
        let path = BsKpointPath::builder().build();
        assert_eq!(path.points.len(), 0);
    }

    #[test]
    fn test_bs_kpoint_path_builder_single_point() {
        let entry = BsKpointPathEntry::builder()
            .coord([0.0, 0.0, 0.0])
            .build();
        let path = BsKpointPath::builder()
            .points(vec![entry])
            .build();
        assert_eq!(path.points.len(), 1);
        assert_eq!(path.points[0].coord, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_bs_kpoint_path_builder_multiple_points_with_vec() {
        let entry1 = BsKpointPathEntry::builder()
            .coord([0.0, 0.0, 0.0])
            .build();
        let entry2 = BsKpointPathEntry::builder()
            .coord([0.5, 0.5, 0.0])
            .build();
        let entry3 = BsKpointPathEntry::builder()
            .coord([0.5, 0.5, 0.5])
            .build();

        let path = BsKpointPath::builder()
            .points(vec![entry1, entry2, entry3])
            .build();

        assert_eq!(path.points.len(), 3);
        assert_eq!(path.points[0].coord, [0.0, 0.0, 0.0]);
        assert_eq!(path.points[1].coord, [0.5, 0.5, 0.0]);
        assert_eq!(path.points[2].coord, [0.5, 0.5, 0.5]);
    }

    #[test]
    fn test_bs_kpoint_path_builder_method_chaining() {
        let entry1 = BsKpointPathEntry::builder().coord([0.0, 0.0, 0.0]).build();
        let entry2 = BsKpointPathEntry::builder().coord([0.3333, 0.3750, 0.3333]).build();
        let entry3 = BsKpointPathEntry::builder().coord([0.5, 0.5, 0.5]).build();

        let path = BsKpointPath::builder()
            .points(vec![entry1, entry2, entry3])
            .build();

        assert_eq!(path.points.len(), 3);
        assert_eq!(path.points[0].coord, [0.0, 0.0, 0.0]);
        assert_eq!(path.points[1].coord, [0.3333, 0.3750, 0.3333]);
        assert_eq!(path.points[2].coord, [0.5, 0.5, 0.5]);
    }
}

use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromBlock, FromCellValue, CResult, query::value_as_f64};

#[derive(Debug, Clone, Copy, PartialEq, bon::Builder)]
/// A line of block `PhononKpointList`
/// The first three entries on a line are the fractional positions of the
/// k-point relative to the reciprocal space lattice vectors.
///
/// The final entry on a line is the weight of the k-point relative to the
/// others specified. The sum of the weights must be equal to 1.
pub struct PhononKpointListEntry {
    pub coord: [f64; 3],
    pub weight: f64,
}

impl FromCellValue for PhononKpointListEntry {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 4 => {
                let coord = [
                    value_as_f64(&arr[0])?,
                    value_as_f64(&arr[1])?,
                    value_as_f64(&arr[2])?,
                ];
                let weight = value_as_f64(&arr[3])?;
                Ok(PhononKpointListEntry { coord, weight })
            }
            _ => Err(castep_cell_fmt::Error::Message(
                "PhononKpointListEntry must be an array of 4 floats".into(),
            )),
        }
    }
}

impl ToCellValue for PhononKpointListEntry {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Array(
            self.coord
                .into_iter()
                .map(CellValue::Float)
                .chain([CellValue::Float(self.weight)])
                .collect(),
        )
    }
}

/// Represents the PHONON_KPOINT_LIST block.
///
/// Contains a list of k-points and their weights for phonon calculations.
/// # Format:
/// %BLOCK PHONON_KPOINT_LIST
///    R1i R1j R1k R1w
///    R2i R2j R2k R2w
/// ...
/// %ENDBLOCK PHONON_KPOINT_LIST
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct PhononKpointList {
    #[builder(default)]
    pub kpoints: Vec<PhononKpointListEntry>,
}

impl FromBlock for PhononKpointList {
    const BLOCK_NAME: &'static str = "PHONON_KPOINT_LIST";
    const BLOCK_ALIASES: &'static [&'static str] = &["PHONON_KPOINTS_LIST"];

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        let kpoints = rows
            .iter()
            .map(PhononKpointListEntry::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;
        Ok(PhononKpointList { kpoints })
    }
}

impl ToCell for PhononKpointList {
    fn to_cell(&self) -> Cell<'_> {
        Cell::Block(
            "PHONON_KPOINT_LIST",
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
    fn test_phonon_kpoint_list_entry_from_cell_value() {
        let val = CellValue::Array(vec![
            CellValue::Float(0.5),
            CellValue::Float(0.25),
            CellValue::Float(0.0),
            CellValue::Float(1.0),
        ]);
        let entry = PhononKpointListEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.coord, [0.5, 0.25, 0.0]);
        assert_eq!(entry.weight, 1.0);
    }

    #[test]
    fn test_phonon_kpoint_list_entry_insufficient_elements() {
        let val = CellValue::Array(vec![
            CellValue::Float(0.5),
            CellValue::Float(0.25),
            CellValue::Float(0.0),
        ]);
        assert!(PhononKpointListEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_phonon_kpoint_list_entry_to_cell_value() {
        let entry = PhononKpointListEntry {
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
    fn test_phonon_kpoint_list_multiple_entries() {
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
        let result = PhononKpointList::from_block_rows(&rows).unwrap();
        assert_eq!(result.kpoints.len(), 2);
        assert_eq!(result.kpoints[0].weight, 0.5);
        assert_eq!(result.kpoints[1].weight, 0.5);
    }

    #[test]
    fn test_phonon_kpoint_list_empty() {
        let result = PhononKpointList::from_block_rows(&[]).unwrap();
        assert_eq!(result.kpoints.len(), 0);
    }

    #[test]
    fn test_phonon_kpoint_list_block_name() {
        assert_eq!(PhononKpointList::BLOCK_NAME, "PHONON_KPOINT_LIST");
    }

    #[test]
    fn test_phonon_kpoint_list_to_cell() {
        let kpts = PhononKpointList {
            kpoints: vec![PhononKpointListEntry {
                coord: [0.0, 0.0, 0.0],
                weight: 1.0,
            }],
        };
        let cell = kpts.to_cell();
        match cell {
            Cell::Block(name, values) => {
                assert_eq!(name, "PHONON_KPOINT_LIST");
                assert_eq!(values.len(), 1);
            }
            _ => panic!("Expected Block"),
        }
    }

    // Builder pattern tests
    #[test]
    fn test_phonon_kpoint_list_entry_builder() {
        let entry = PhononKpointListEntry::builder()
            .coord([0.5, 0.25, 0.0])
            .weight(1.0)
            .build();
        assert_eq!(entry.coord, [0.5, 0.25, 0.0]);
        assert_eq!(entry.weight, 1.0);
    }

    #[test]
    fn test_phonon_kpoint_list_builder_empty() {
        let list = PhononKpointList::builder().build();
        assert_eq!(list.kpoints.len(), 0);
    }

    #[test]
    fn test_phonon_kpoint_list_builder_single_kpoint() {
        let entry = PhononKpointListEntry::builder()
            .coord([0.0, 0.0, 0.0])
            .weight(1.0)
            .build();
        let list = PhononKpointList::builder().kpoints(vec![entry]).build();
        assert_eq!(list.kpoints.len(), 1);
        assert_eq!(list.kpoints[0].coord, [0.0, 0.0, 0.0]);
        assert_eq!(list.kpoints[0].weight, 1.0);
    }

    #[test]
    fn test_phonon_kpoint_list_builder_multiple_kpoints_vec() {
        let entries = vec![
            PhononKpointListEntry::builder()
                .coord([0.0, 0.0, 0.0])
                .weight(0.5)
                .build(),
            PhononKpointListEntry::builder()
                .coord([0.5, 0.5, 0.5])
                .weight(0.5)
                .build(),
        ];
        let list = PhononKpointList::builder().kpoints(entries).build();
        assert_eq!(list.kpoints.len(), 2);
        assert_eq!(list.kpoints[0].weight, 0.5);
        assert_eq!(list.kpoints[1].weight, 0.5);
    }

    #[test]
    fn test_phonon_kpoint_list_builder_method_chaining() {
        let kpoints = vec![
            PhononKpointListEntry::builder()
                .coord([0.0, 0.0, 0.0])
                .weight(0.25)
                .build(),
            PhononKpointListEntry::builder()
                .coord([0.5, 0.0, 0.0])
                .weight(0.25)
                .build(),
            PhononKpointListEntry::builder()
                .coord([0.0, 0.5, 0.0])
                .weight(0.25)
                .build(),
            PhononKpointListEntry::builder()
                .coord([0.0, 0.0, 0.5])
                .weight(0.25)
                .build(),
        ];

        let list = PhononKpointList::builder()
            .kpoints(kpoints)
            .build();

        assert_eq!(list.kpoints.len(), 4);
        assert_eq!(list.kpoints[0].coord, [0.0, 0.0, 0.0]);
        assert_eq!(list.kpoints[1].coord, [0.5, 0.0, 0.0]);
        assert_eq!(list.kpoints[2].coord, [0.0, 0.5, 0.0]);
        assert_eq!(list.kpoints[3].coord, [0.0, 0.0, 0.5]);
        assert!(list.kpoints.iter().all(|k| k.weight == 0.25));
    }

    #[test]
    fn test_phonon_kpoint_list_builder_push_after_build() {
        let mut list = PhononKpointList::builder().build();
        list.kpoints.push(
            PhononKpointListEntry::builder()
                .coord([0.0, 0.0, 0.0])
                .weight(0.5)
                .build(),
        );
        list.kpoints.push(
            PhononKpointListEntry::builder()
                .coord([0.5, 0.5, 0.5])
                .weight(0.5)
                .build(),
        );
        assert_eq!(list.kpoints.len(), 2);
        assert_eq!(list.kpoints[0].weight, 0.5);
        assert_eq!(list.kpoints[1].weight, 0.5);
    }
}

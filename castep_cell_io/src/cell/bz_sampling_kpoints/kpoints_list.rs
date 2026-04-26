use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromBlock, FromCellValue, CResult};

use super::Kpoint;

#[derive(Debug, Clone, PartialEq, PartialOrd, bon::Builder)]
/// Represents the KPOINTS_LIST block.
///
/// Contains a list of k-points and their weights for Brillouin zone sampling.
/// # Format:
/// %BLOCK KPOINTS_LIST
///    R1i R1j R1k R1w
///    R2i R2j R2k R2w
/// ...
/// %ENDBLOCK KPOINTS_LIST
pub struct KpointsList {
    #[builder(default)]
    pub kpts: Vec<Kpoint>,
}

impl FromBlock for KpointsList {
    const BLOCK_NAME: &'static str = "KPOINT_LIST";
    const BLOCK_ALIASES: &'static [&'static str] = &["KPOINTS_LIST"];

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        let kpts = rows
            .iter()
            .map(Kpoint::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;
        Ok(KpointsList { kpts })
    }
}

impl ToCell for KpointsList {
    fn to_cell(&self) -> Cell<'_> {
        Cell::Block(
            "KPOINT_LIST",
            self.kpts
                .iter()
                .map(|kpt| kpt.to_cell_value())
                .collect::<Vec<CellValue>>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kpoints_list_multiple_entries() {
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
        let result = KpointsList::from_block_rows(&rows).unwrap();
        assert_eq!(result.kpts.len(), 2);
        assert_eq!(result.kpts[0].weight, 0.5);
        assert_eq!(result.kpts[1].weight, 0.5);
    }

    #[test]
    fn test_kpoints_list_empty() {
        let result = KpointsList::from_block_rows(&[]).unwrap();
        assert_eq!(result.kpts.len(), 0);
    }

    #[test]
    fn test_kpoints_list_block_name() {
        assert_eq!(KpointsList::BLOCK_NAME, "KPOINT_LIST");
    }

    #[test]
    fn test_kpoints_list_to_cell() {
        let kpts = KpointsList {
            kpts: vec![Kpoint {
                coord: [0.0, 0.0, 0.0],
                weight: 1.0,
            }],
        };
        let cell = kpts.to_cell();
        match cell {
            Cell::Block(name, values) => {
                assert_eq!(name, "KPOINT_LIST");
                assert_eq!(values.len(), 1);
            }
            _ => panic!("Expected Block"),
        }
    }

    #[test]
    fn test_kpoints_list_builder_basic() {
        let kpts_list = KpointsList::builder()
            .kpts(vec![Kpoint {
                coord: [0.5, 0.5, 0.5],
                weight: 1.0,
            }])
            .build();
        assert_eq!(kpts_list.kpts.len(), 1);
        assert_eq!(kpts_list.kpts[0].weight, 1.0);
    }

    #[test]
    fn test_kpoints_list_builder_empty() {
        let kpts_list = KpointsList::builder().build();
        assert_eq!(kpts_list.kpts.len(), 0);
    }

    #[test]
    fn test_kpoints_list_builder_with_multiple_kpoints() {
        let kpts = vec![
            Kpoint {
                coord: [0.0, 0.0, 0.0],
                weight: 0.5,
            },
            Kpoint {
                coord: [0.5, 0.5, 0.5],
                weight: 0.5,
            },
        ];
        let kpts_list = KpointsList::builder()
            .kpts(kpts)
            .build();
        assert_eq!(kpts_list.kpts.len(), 2);
        assert_eq!(kpts_list.kpts[0].weight, 0.5);
        assert_eq!(kpts_list.kpts[1].weight, 0.5);
    }

    #[test]
    fn test_kpoints_list_builder_chaining() {
        let kpt1 = Kpoint {
            coord: [0.0, 0.0, 0.0],
            weight: 0.25,
        };
        let kpt2 = Kpoint {
            coord: [0.5, 0.0, 0.0],
            weight: 0.25,
        };
        let kpt3 = Kpoint {
            coord: [0.5, 0.5, 0.0],
            weight: 0.25,
        };
        let kpt4 = Kpoint {
            coord: [0.5, 0.5, 0.5],
            weight: 0.25,
        };

        let kpts_list = KpointsList::builder()
            .kpts(vec![kpt1, kpt2, kpt3, kpt4])
            .build();

        assert_eq!(kpts_list.kpts.len(), 4);
        assert!(kpts_list.kpts.iter().all(|k| k.weight == 0.25));
    }
}



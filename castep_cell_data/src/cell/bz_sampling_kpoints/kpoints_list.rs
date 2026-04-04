use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromBlock, FromCellValue, CResult};

use super::Kpoint;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
    pub kpts: Vec<Kpoint>,
}

impl FromBlock for KpointsList {
    const BLOCK_NAME: &'static str = "KPOINTS_LIST";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        let kpts = rows
            .iter()
            .map(Kpoint::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;
        Ok(KpointsList { kpts })
    }
}

impl ToCell for KpointsList {
    fn to_cell(&self) -> Cell {
        Cell::Block(
            "KPOINTS_LIST",
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
        assert_eq!(KpointsList::BLOCK_NAME, "KPOINTS_LIST");
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
                assert_eq!(name, "KPOINTS_LIST");
                assert_eq!(values.len(), 1);
            }
            _ => panic!("Expected Block"),
        }
    }
}



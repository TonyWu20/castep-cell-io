use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromBlock, FromCellValue, CResult};

use super::Kpoint;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
/// Represents the KPOINTS_LIST block.
/// This data block contains a list of k-points at which the Brillouin zone
/// will be sampled during non-self consistent band structure calculations,
/// along with the associated weights. It has the same format as `KPOINTS_LIST`.
/// # Format:
/// %BLOCK BS_KPOINTS_LIST
///    R1i R1j R1k R1w
///    R2i R2j R2k R2w
/// ...
/// %ENDBLOCK BS_KPOINTS_LIST
pub struct BSKpointList {
    pub kpts: Vec<Kpoint>,
}

impl FromBlock for BSKpointList {
    const BLOCK_NAME: &'static str = "BS_KPOINTS_LIST";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        let kpts = rows
            .iter()
            .map(Kpoint::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;
        Ok(BSKpointList { kpts })
    }
}

impl ToCell for BSKpointList {
    fn to_cell(&self) -> Cell {
        Cell::Block(
            "BS_KPOINT_LIST",
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
    fn test_bs_kpoints_list_multiple_entries() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(0.25),
            ]),
            CellValue::Array(vec![
                CellValue::Float(0.5),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(0.75),
            ]),
        ];
        let result = BSKpointList::from_block_rows(&rows).unwrap();
        assert_eq!(result.kpts.len(), 2);
        assert_eq!(result.kpts[0].weight, 0.25);
        assert_eq!(result.kpts[1].weight, 0.75);
    }

    #[test]
    fn test_bs_kpoints_list_empty() {
        let result = BSKpointList::from_block_rows(&[]).unwrap();
        assert_eq!(result.kpts.len(), 0);
    }

    #[test]
    fn test_bs_kpoints_list_block_name() {
        assert_eq!(BSKpointList::BLOCK_NAME, "BS_KPOINTS_LIST");
    }

    #[test]
    fn test_bs_kpoints_list_to_cell() {
        let kpts = BSKpointList {
            kpts: vec![Kpoint {
                coord: [0.0, 0.0, 0.0],
                weight: 1.0,
            }],
        };
        let cell = kpts.to_cell();
        match cell {
            Cell::Block(name, values) => {
                assert_eq!(name, "BS_KPOINT_LIST");
                assert_eq!(values.len(), 1);
            }
            _ => panic!("Expected Block"),
        }
    }
}



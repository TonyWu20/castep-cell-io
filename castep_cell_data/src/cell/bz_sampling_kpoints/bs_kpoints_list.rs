use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromBlock, FromCellValue, CResult};

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



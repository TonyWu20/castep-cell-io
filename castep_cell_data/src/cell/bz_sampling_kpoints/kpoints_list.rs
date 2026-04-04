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



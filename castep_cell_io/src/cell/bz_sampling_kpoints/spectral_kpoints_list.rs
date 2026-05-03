use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromBlock, FromCellValue, CResult};

use super::Kpoint;

#[derive(Debug, Clone, PartialEq, PartialOrd, bon::Builder)]
/// Represents the SPECTRAL_KPOINT_LIST block.
/// This data block contains a list of k-points at which the Brillouin zone
/// will be sampled during spectral calculations,
/// along with the associated weights.
/// # Format:
/// %BLOCK SPECTRAL_KPOINT_LIST
///    R1i R1j R1k R1w
///    R2i R2j R2k R2w
/// ...
/// %ENDBLOCK SPECTRAL_KPOINT_LIST
pub struct SpectralKpointsList {
    #[builder(default)]
    pub kpts: Vec<Kpoint>,
}

impl FromBlock for SpectralKpointsList {
    const BLOCK_NAME: &'static str = "SPECTRAL_KPOINT_LIST";
    const BLOCK_ALIASES: &'static [&'static str] = &["SPECTRAL_KPOINTS_LIST", "BS_KPOINT_LIST", "BS_KPOINTS_LIST"];

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        let kpts = rows
            .iter()
            .map(Kpoint::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;
        Ok(SpectralKpointsList { kpts })
    }
}

impl ToCell for SpectralKpointsList {
    fn to_cell(&self) -> Cell<'_> {
        Cell::Block(
            "SPECTRAL_KPOINT_LIST",
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
    fn test_spectral_kpoints_list_multiple_entries() {
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
        let result = SpectralKpointsList::from_block_rows(&rows).unwrap();
        assert_eq!(result.kpts.len(), 2);
        assert_eq!(result.kpts[0].weight, 0.25);
    }
}

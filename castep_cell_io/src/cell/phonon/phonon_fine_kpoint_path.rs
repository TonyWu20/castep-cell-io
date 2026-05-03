use castep_cell_fmt::{Cell, CellValue, CResult, FromBlock, FromCellValue, ToCell, ToCellValue};
use super::phonon_kpoint_path::PhononKpointPathEntry;

/// Represents the PHONON_FINE_KPOINT_PATH block.
///
/// Contains a list of q-vectors that define a fine path through reciprocal space for phonon calculations.
/// Format:
/// %BLOCK PHONON_FINE_KPOINT_PATH
/// R1i R1j R1k
/// R2i R2j R2k
/// ...
/// %ENDBLOCK PHONON_FINE_KPOINT_PATH
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct PhononFineKpointPath {
    /// The list of q-vector entries.
    #[builder(default)]
    pub points: Vec<PhononKpointPathEntry>,
}

impl FromBlock for PhononFineKpointPath {
    const BLOCK_NAME: &'static str = "PHONON_FINE_KPOINT_PATH";
    const BLOCK_ALIASES: &'static [&'static str] = &["PHONON_FINE_KPOINTS_PATH"];

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        let points = rows
            .iter()
            .map(PhononKpointPathEntry::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;
        Ok(PhononFineKpointPath { points })
    }
}

impl ToCell for PhononFineKpointPath {
    fn to_cell(&self) -> Cell<'_> {
        Cell::Block(
            "PHONON_FINE_KPOINT_PATH",
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
    fn test_phonon_fine_kpoint_path_multiple_entries() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
            ]),
            CellValue::Array(vec![
                CellValue::Float(0.5),
                CellValue::Float(0.5),
                CellValue::Float(0.5),
            ]),
        ];
        let result = PhononFineKpointPath::from_block_rows(&rows).unwrap();
        assert_eq!(result.points.len(), 2);
        assert_eq!(result.points[0].coord, [0.0, 0.0, 0.0]);
    }
}

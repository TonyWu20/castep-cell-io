use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromBlock, CResult, Error, query::value_as_u32};

/// Represents the constraints on lattice parameters (lengths and angles) during relaxation/MD.
///
/// Consists of two sets of indices:
/// - One for the lattice vector magnitudes (a, b, c)
/// - One for the lattice angles (alpha, beta, gamma)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellConstraints {
    /// Constraints for lattice vector magnitudes [a, b, c].
    /// 0 = fixed, positive integer = free to vary, shared integers = linked values.
    pub lengths: [u32; 3],
    /// Constraints for lattice angles [alpha, beta, gamma] in degrees.
    /// 0 = fixed, positive integer = free to vary, shared integers = linked values.
    pub angles: [u32; 3],
}

impl FromBlock for CellConstraints {
    const BLOCK_NAME: &'static str = "CELL_CONSTRAINTS";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.len() < 2 {
            return Err(castep_cell_io::Error::Message(
                "CELL_CONSTRAINTS requires at least 2 rows".into(),
            ));
        }

        let lengths = match &rows[0] {
            CellValue::Array(arr) if arr.len() >= 3 => [
                value_as_u32(&arr[0])?,
                value_as_u32(&arr[1])?,
                value_as_u32(&arr[2])?,
            ],
            _ => return Err(castep_cell_io::Error::Message(
                "CELL_CONSTRAINTS lengths row must be an array of 3 values".into(),
            )),
        };

        let angles = match &rows[1] {
            CellValue::Array(arr) if arr.len() >= 3 => [
                value_as_u32(&arr[0])?,
                value_as_u32(&arr[1])?,
                value_as_u32(&arr[2])?,
            ],
            _ => return Err(castep_cell_io::Error::Message(
                "CELL_CONSTRAINTS angles row must be an array of 3 values".into(),
            )),
        };

        Ok(CellConstraints { lengths, angles })
    }
}

impl ToCell for CellConstraints {
    fn to_cell(&self) -> Cell {
        let block_content = [
            CellValue::Array(self.lengths.iter().map(|&v| CellValue::UInt(v)).collect()),
            CellValue::Array(self.angles.iter().map(|&v| CellValue::UInt(v)).collect()),
        ]
        .to_vec();

        Cell::Block("CELL_CONSTRAINTS", block_content)
    }
}



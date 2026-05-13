use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromBlock, FromCellFile, ToCellFile, ToCell};

use super::velocities::*;

/// Molecular dynamics dynamics parameters
///
/// Contains the ionic velocities block for MD restart.
#[derive(Debug, Clone, Default, Builder)]
pub struct DynamicsParams {
    pub ionic_velocities: Option<IonicVelocities>,
}

impl DynamicsParams {
    pub fn validate(self) -> Result<Self, String> {
        Ok(self)
    }
}

impl FromCellFile for DynamicsParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_ionic_velocities(IonicVelocities::from_cells(tokens).ok())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for DynamicsParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.ionic_velocities { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_ok() {
        let p = DynamicsParams::default();
        assert!(p.validate().is_ok());
    }
}

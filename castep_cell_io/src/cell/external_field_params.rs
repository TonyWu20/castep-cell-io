use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromBlock, FromCellFile, ToCellFile, ToCell};

use super::external_fields::*;

/// External field parameters (electric field and pressure)
///
/// Both fields are independent — they can coexist.
#[derive(Debug, Clone, Default, Builder)]
pub struct ExternalFieldParams {
    pub external_efield: Option<ExternalEfield>,
    pub external_pressure: Option<ExternalPressure>,
}

impl ExternalFieldParams {
    pub fn validate(self) -> Result<Self, String> {
        Ok(self)
    }
}

impl FromCellFile for ExternalFieldParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_external_efield(ExternalEfield::from_cells(tokens).ok())
            .maybe_external_pressure(ExternalPressure::from_cells(tokens).ok())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for ExternalFieldParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.external_efield { cells.push(v.to_cell()); }
        if let Some(v) = &self.external_pressure { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_ok() {
        let p = ExternalFieldParams::default();
        assert!(p.validate().is_ok());
    }

    #[test]
    fn test_validate_with_efield_ok() {
        let p = ExternalFieldParams {
            external_efield: Some(ExternalEfield {
                unit: None,
                field_vector: [0.0, 0.0, 0.0],
            }),
            ..Default::default()
        };
        assert!(p.validate().is_ok());
    }
}

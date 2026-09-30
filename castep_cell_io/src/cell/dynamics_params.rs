use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromBlock, FromCellFile, FromKeyValue, ToCellFile, ToCell};

use super::noise::*;
use super::velocities::*;

/// Molecular dynamics dynamics parameters
///
/// Contains the ionic velocities block for MD restart, plus the 6.11
/// random-noise keywords applied to the cell and/or positions before an MD
/// run (`cell_random_noise` in `Fundamental/cell.f90`).
#[derive(Debug, Clone, Default, Builder)]
pub struct DynamicsParams {
    pub ionic_velocities: Option<IonicVelocities>,
    /// 6.11: random noise added onto each ion's position (default: none).
    pub positions_noise: Option<PositionsNoise>,
    /// 6.11: random noise added onto the cell parameters (default: none).
    pub cell_noise: Option<CellNoise>,
}

impl DynamicsParams {
    pub fn validate(self) -> Result<Self, String> {
        // CASTEP applies noise only when it is > 0 (`cell_random_noise`); a
        // negative magnitude is meaningless, so reject it.
        if let Some(n) = &self.positions_noise
            && n.value < 0.0
        {
            return Err("POSITIONS_NOISE must be >= 0 (0 means no noise)".into());
        }
        if let Some(n) = &self.cell_noise
            && n.value < 0.0
        {
            return Err("CELL_NOISE must be >= 0 (0 means no noise)".into());
        }
        Ok(self)
    }
}

impl FromCellFile for DynamicsParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_ionic_velocities(IonicVelocities::from_cells(tokens).ok())
            .maybe_positions_noise(PositionsNoise::from_cells(tokens).ok().flatten())
            .maybe_cell_noise(CellNoise::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for DynamicsParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.ionic_velocities { cells.push(v.to_cell()); }
        if let Some(v) = &self.positions_noise { cells.push(v.to_cell()); }
        if let Some(v) = &self.cell_noise { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::LengthUnit;

    #[test]
    fn test_validate_ok() {
        let p = DynamicsParams::default();
        assert!(p.validate().is_ok());
    }

    #[test]
    fn test_validate_zero_noise_ok() {
        let p = DynamicsParams::builder()
            .positions_noise(PositionsNoise { value: 0.0, unit: None })
            .cell_noise(CellNoise { value: 0.0, unit: Some(LengthUnit::Ang) })
            .build();
        assert!(p.validate().is_ok());
    }

    #[test]
    fn test_validate_negative_noise_rejected() {
        let p = DynamicsParams::builder()
            .positions_noise(PositionsNoise { value: -0.1, unit: None })
            .build();
        assert!(p.validate().is_err());

        let p = DynamicsParams::builder()
            .cell_noise(CellNoise { value: -0.1, unit: None })
            .build();
        assert!(p.validate().is_err());
    }
}

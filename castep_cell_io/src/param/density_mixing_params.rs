use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromCellFile, ToCellFile, ToCell, FromKeyValue};
use super::density_mixing::*;

/// Density Mixing parameters for CASTEP calculations
///
/// This group contains settings that control the charge density mixing scheme
/// used during self-consistent field (SCF) iterations, including mixing amplitudes,
/// cutoff energies, and history length for various mixing algorithms.
#[derive(Debug, Clone, Default, Builder)]
pub struct DensityMixingParams {
    pub mix_charge_amp: Option<MixChargeAmp>,
    pub mix_charge_gmax: Option<MixChargeGmax>,
    pub mix_cut_off_energy: Option<MixCutOffEnergy>,
    pub mix_history_length: Option<MixHistoryLength>,
    pub mix_metric_q: Option<MixMetricQ>,
    pub mix_spin_amp: Option<MixSpinAmp>,
    pub mix_spin_gmax: Option<MixSpinGmax>,
    pub mixing_scheme: Option<MixingScheme>,
}

impl DensityMixingParams {
    /// Validates intra-group constraints for density mixing parameters
    pub fn validate(self) -> Result<Self, String> {
        // No specific intra-group validation needed for density mixing params
        Ok(self)
    }
}

impl FromCellFile for DensityMixingParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_mix_charge_amp(MixChargeAmp::from_cells(tokens).ok().flatten())
            .maybe_mix_charge_gmax(MixChargeGmax::from_cells(tokens).ok().flatten())
            .maybe_mix_cut_off_energy(MixCutOffEnergy::from_cells(tokens).ok().flatten())
            .maybe_mix_history_length(MixHistoryLength::from_cells(tokens).ok().flatten())
            .maybe_mix_metric_q(MixMetricQ::from_cells(tokens).ok().flatten())
            .maybe_mix_spin_amp(MixSpinAmp::from_cells(tokens).ok().flatten())
            .maybe_mix_spin_gmax(MixSpinGmax::from_cells(tokens).ok().flatten())
            .maybe_mixing_scheme(MixingScheme::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for DensityMixingParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.mix_charge_amp { cells.push(v.to_cell()); }
        if let Some(v) = &self.mix_charge_gmax { cells.push(v.to_cell()); }
        if let Some(v) = &self.mix_cut_off_energy { cells.push(v.to_cell()); }
        if let Some(v) = &self.mix_history_length { cells.push(v.to_cell()); }
        if let Some(v) = &self.mix_metric_q { cells.push(v.to_cell()); }
        if let Some(v) = &self.mix_spin_amp { cells.push(v.to_cell()); }
        if let Some(v) = &self.mix_spin_gmax { cells.push(v.to_cell()); }
        if let Some(v) = &self.mixing_scheme { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_fmt::parse;

    #[test]
    fn test_default_construction() {
        let params = DensityMixingParams::default();
        assert!(params.mix_charge_amp.is_none());
        assert!(params.mixing_scheme.is_none());
    }

    #[test]
    fn test_builder_construction() {
        let params = DensityMixingParams::builder().build();
        assert!(params.mix_charge_amp.is_none());
    }

    #[test]
    fn test_validate_empty() {
        let params = DensityMixingParams::default();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_to_cell_file_empty() {
        let params = DensityMixingParams::default();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 0);
    }
}

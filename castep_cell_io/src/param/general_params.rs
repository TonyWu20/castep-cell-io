use bon::Builder;
use castep_cell_fmt::{Cell, CResult, Error, FromCellFile, ToCellFile, ToCell, FromKeyValue};
use super::general::*;

/// General parameters for CASTEP calculations
///
/// This group contains fundamental settings that control the overall behavior
/// of CASTEP calculations, including task type, output options, and general
/// calculation parameters.
#[derive(Debug, Clone, Default, Builder)]
pub struct GeneralParams {
    pub task: Option<Task>,
    pub comment: Option<Comment>,
    pub continuation: Option<Continuation>,
    pub reuse: Option<Reuse>,
    pub backup_interval: Option<BackupInterval>,
    pub calculate_densdiff: Option<CalculateDensdiff>,
    pub calculate_elf: Option<CalculateElf>,
    pub calculate_hirshfeld: Option<CalculateHirshfeld>,
    pub calculate_stress: Option<CalculateStress>,
    pub charge_unit: Option<ChargeUnit>,
    pub checkpoint: Option<Checkpoint>,
    pub data_distribution: Option<DataDistribution>,
    pub iprint: Option<Iprint>,
    pub num_backup_iter: Option<NumBackupIter>,
    pub opt_strategy: Option<OptStrategy>,
    pub page_wvfns: Option<PageWvfns>,
    pub print_clock: Option<PrintClock>,
    pub print_memory_usage: Option<PrintMemoryUsage>,
    pub rand_seed: Option<RandSeed>,
    pub run_time: Option<RunTime>,
    pub stop: Option<Stop>,
    pub write_checkpoint: Option<WriteCheckpoint>,
    pub write_formatted_density: Option<WriteFormattedDensity>,
    pub write_formatted_elf: Option<WriteFormattedElf>,
    pub write_formatted_potential: Option<WriteFormattedPotential>,
    pub write_orbitals: Option<WriteOrbitals>,
}

impl GeneralParams {
    /// Validates intra-group constraints for general parameters
    pub fn validate(self) -> Result<Self, String> {
        // No specific intra-group validation needed for general params
        Ok(self)
    }
}

impl FromCellFile for GeneralParams {
    fn from_cell_file(tokens: &[Cell<'_>]) -> CResult<Self> {
        Self::builder()
            .maybe_task(Task::from_cells(tokens).ok().flatten())
            .maybe_comment(Comment::from_cells(tokens).ok().flatten())
            .maybe_continuation(Continuation::from_cells(tokens).ok().flatten())
            .maybe_reuse(Reuse::from_cells(tokens).ok().flatten())
            .maybe_backup_interval(BackupInterval::from_cells(tokens).ok().flatten())
            .maybe_calculate_densdiff(CalculateDensdiff::from_cells(tokens).ok().flatten())
            .maybe_calculate_elf(CalculateElf::from_cells(tokens).ok().flatten())
            .maybe_calculate_hirshfeld(CalculateHirshfeld::from_cells(tokens).ok().flatten())
            .maybe_calculate_stress(CalculateStress::from_cells(tokens).ok().flatten())
            .maybe_charge_unit(ChargeUnit::from_cells(tokens).ok().flatten())
            .maybe_checkpoint(Checkpoint::from_cells(tokens).ok().flatten())
            .maybe_data_distribution(DataDistribution::from_cells(tokens).ok().flatten())
            .maybe_iprint(Iprint::from_cells(tokens).ok().flatten())
            .maybe_num_backup_iter(NumBackupIter::from_cells(tokens).ok().flatten())
            .maybe_opt_strategy(OptStrategy::from_cells(tokens).ok().flatten())
            .maybe_page_wvfns(PageWvfns::from_cells(tokens).ok().flatten())
            .maybe_print_clock(PrintClock::from_cells(tokens).ok().flatten())
            .maybe_print_memory_usage(PrintMemoryUsage::from_cells(tokens).ok().flatten())
            .maybe_rand_seed(RandSeed::from_cells(tokens).ok().flatten())
            .maybe_run_time(RunTime::from_cells(tokens).ok().flatten())
            .maybe_stop(Stop::from_cells(tokens).ok().flatten())
            .maybe_write_checkpoint(WriteCheckpoint::from_cells(tokens).ok().flatten())
            .maybe_write_formatted_density(WriteFormattedDensity::from_cells(tokens).ok().flatten())
            .maybe_write_formatted_elf(WriteFormattedElf::from_cells(tokens).ok().flatten())
            .maybe_write_formatted_potential(WriteFormattedPotential::from_cells(tokens).ok().flatten())
            .maybe_write_orbitals(WriteOrbitals::from_cells(tokens).ok().flatten())
            .build()
            .validate()
            .map_err(|e| Error::Message(e.to_string()))
    }
}

impl ToCellFile for GeneralParams {
    fn to_cell_file(&self) -> Vec<Cell<'_>> {
        let mut cells = Vec::new();
        if let Some(v) = &self.task { cells.push(v.to_cell()); }
        if let Some(v) = &self.comment { cells.push(v.to_cell()); }
        if let Some(v) = &self.continuation { cells.push(v.to_cell()); }
        if let Some(v) = &self.reuse { cells.push(v.to_cell()); }
        if let Some(v) = &self.backup_interval { cells.push(v.to_cell()); }
        if let Some(v) = &self.calculate_densdiff { cells.push(v.to_cell()); }
        if let Some(v) = &self.calculate_elf { cells.push(v.to_cell()); }
        if let Some(v) = &self.calculate_hirshfeld { cells.push(v.to_cell()); }
        if let Some(v) = &self.calculate_stress { cells.push(v.to_cell()); }
        if let Some(v) = &self.charge_unit { cells.push(v.to_cell()); }
        if let Some(v) = &self.checkpoint { cells.push(v.to_cell()); }
        if let Some(v) = &self.data_distribution { cells.push(v.to_cell()); }
        if let Some(v) = &self.iprint { cells.push(v.to_cell()); }
        if let Some(v) = &self.num_backup_iter { cells.push(v.to_cell()); }
        if let Some(v) = &self.opt_strategy { cells.push(v.to_cell()); }
        if let Some(v) = &self.page_wvfns { cells.push(v.to_cell()); }
        if let Some(v) = &self.print_clock { cells.push(v.to_cell()); }
        if let Some(v) = &self.print_memory_usage { cells.push(v.to_cell()); }
        if let Some(v) = &self.rand_seed { cells.push(v.to_cell()); }
        if let Some(v) = &self.run_time { cells.push(v.to_cell()); }
        if let Some(v) = &self.stop { cells.push(v.to_cell()); }
        if let Some(v) = &self.write_checkpoint { cells.push(v.to_cell()); }
        if let Some(v) = &self.write_formatted_density { cells.push(v.to_cell()); }
        if let Some(v) = &self.write_formatted_elf { cells.push(v.to_cell()); }
        if let Some(v) = &self.write_formatted_potential { cells.push(v.to_cell()); }
        if let Some(v) = &self.write_orbitals { cells.push(v.to_cell()); }
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general_params_default() {
        let params = GeneralParams::default();
        assert!(params.task.is_none());
        assert!(params.comment.is_none());
        assert!(params.continuation.is_none());
        assert!(params.reuse.is_none());
    }

    #[test]
    fn test_general_params_builder() {
        let params = GeneralParams::builder().build();
        assert!(params.task.is_none());
        assert!(params.comment.is_none());
    }

    #[test]
    fn test_general_params_to_cell_file_empty() {
        let params = GeneralParams::default();
        let cells = params.to_cell_file();
        assert_eq!(cells.len(), 0);
    }

    #[test]
    fn test_general_params_field_count() {
        // Verify that the struct has the expected number of fields
        let params = GeneralParams::default();
        // This is a compile-time check via the struct definition
        let _ = params.task;
        let _ = params.comment;
        let _ = params.continuation;
        let _ = params.reuse;
        let _ = params.backup_interval;
        let _ = params.calculate_densdiff;
        let _ = params.calculate_elf;
        let _ = params.calculate_hirshfeld;
        let _ = params.calculate_stress;
        let _ = params.charge_unit;
        let _ = params.checkpoint;
        let _ = params.data_distribution;
        let _ = params.iprint;
        let _ = params.num_backup_iter;
        let _ = params.opt_strategy;
        let _ = params.page_wvfns;
        let _ = params.print_clock;
        let _ = params.print_memory_usage;
        let _ = params.rand_seed;
        let _ = params.run_time;
        let _ = params.stop;
        let _ = params.write_checkpoint;
        let _ = params.write_formatted_density;
        let _ = params.write_formatted_elf;
        let _ = params.write_formatted_potential;
        let _ = params.write_orbitals;
    }
}

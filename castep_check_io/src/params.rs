//! The parameters section of a CASTEP model file.
//!
//! The section is 340 records in the 6.11 layout: the `BEGIN_PARAMETERS_DUMP`
//! tag, the version record, 23 `BEGIN_*`/`END_*` block tag pairs, and every
//! value record of `parameters_dump`. The layout was extracted from the
//! CASTEP 6.11 source and validated record-for-record against real files
//! (see `notes/plans/check-params-spec.txt`).
//!
//! The codec is table-driven. Reading keeps the raw payload of every record.
//! Writing replays the stored payloads. A roundtrip is byte-exact because
//! nothing is re-encoded, except the version record and records absent from
//! an older file, which are filled with default encodings.

use crate::error::{CResult, CheckError, FileVersion};
use crate::record::{char_field, char_str, parse_f64s, parse_i32s, parse_logical4s, RecordReader, RecordWriter};
use crate::version::{check_file_version, gate_open, parse_version_record, target_version_record};

/// Record widths that vary across local CASTEP builds.
///
/// The version axis (ADR-0005) covers release-to-release layout changes.
/// Local builds can change record widths inside one release. The standard
/// CASTEP 6.11 values are the default. A file read with a layout is
/// written with that layout unless replaced with `CheckDocument::with_layout`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct ParamsLayout {
    /// Byte width of the `devel_code` record in the DEVEL block.
    ///
    /// Standard 6.11 is 80. A local build may widen it, for example to
    /// `file_maxpath` (256). The writer space-pads to this width and
    /// refuses to truncate a stored value that exceeds it.
    #[serde(default = "ParamsLayout::default_devel_code_len")]
    pub devel_code_len: u32,
}

impl Default for ParamsLayout {
    fn default() -> Self {
        Self { devel_code_len: 80 }
    }
}

impl ParamsLayout {
    fn default_devel_code_len() -> u32 {
        80
    }

    /// The width the reader and writer use for the `devel_code` record.
    pub fn devel_code_width(&self) -> u32 {
        self.devel_code_len
    }
}

/// The root of a layout config file.
///
/// ```toml
/// [params]
/// devel_code_len = 256
///
/// [read.params]
/// devel_code_len = 80
/// ```
///
/// The `params` side is the write width. The `read` side is the input
/// width. Missing sides take the standard values.
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct LayoutConfig {
    /// The write section widths.
    pub params: ParamsLayout,
    /// The read side. Absent: the standard layout reads the input.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<Box<LayoutConfig>>,
}

impl LayoutConfig {
    /// Parses a layout config from a TOML string.
    pub fn parse_toml(s: &str) -> CResult<Self> {
        Ok(toml::from_str(s)?)
    }

    /// Reads and parses a layout config file.
    pub fn from_file(path: impl AsRef<std::path::Path>) -> CResult<Self> {
        let data = std::fs::read(path.as_ref())
            .map_err(|e| CheckError::ConfigFile { path: path.as_ref().to_path_buf(), source: e })?;
        Self::parse_toml(&String::from_utf8_lossy(&data))
    }

    /// The write layout.
    pub fn write_layout(&self) -> ParamsLayout {
        self.params
    }

    /// The read layout. The standard layout when unset.
    pub fn read_layout(&self) -> ParamsLayout {
        self.read.as_ref().map(|c| c.params).unwrap_or_default()
    }
}

/// The kind of a parameter record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A 4-byte big-endian integer.
    I32,
    /// An 8-byte big-endian double.
    F64,
    /// A 4-byte logical. 0 is false, 1 is true.
    Bool,
    /// A fixed-length character field, space-padded.
    Char,
    /// A 30-byte section tag.
    Tag,
}

/// One record of the 6.11 parameters layout.
#[derive(Debug, Clone, Copy)]
pub struct Field {
    /// The field name. A `Kind::Tag` field reads `name:tag_text`.
    pub name: &'static str,
    /// The record kind.
    pub kind: Kind,
    /// The payload size in bytes, in the 6.11 target layout.
    pub size: usize,
    /// The minimum file version that carries this record. `None` is base.
    pub gate: Option<FileVersion>,
}

impl Field {
    /// The tag text of a `Kind::Tag` field. `None` for value fields.
    pub fn tag(&self) -> Option<&str> {
        self.name.split_once(':').map(|pair| pair.1)
    }

    /// The lookup key. Tag fields use their `name:tag_text` form.
    pub fn key(&self) -> &str {
        self.name
    }

    /// The bare value name, without a tag suffix.
    pub fn value_name(&self) -> &str {
        self.name.split_once(':').map_or(self.name, |pair| pair.0)
    }
}

/// The full 6.11 parameters layout: 340 records in file order.
///
/// The table was generated from the CASTEP 6.11 source and validated
/// against real model files.
pub static PARAM_FIELDS: &[Field] = &[
        Field { name: "header:BEGIN_PARAMETERS_DUMP", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "ver_string", kind: Kind::Char, size: 10, gate: None },
        Field { name: "header:BEGIN_GENERAL", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "seedname", kind: Kind::Char, size: 256, gate: None },
        Field { name: "comment", kind: Kind::Char, size: 80, gate: None },
        Field { name: "iprint", kind: Kind::I32, size: 4, gate: None },
        Field { name: "continuation", kind: Kind::Char, size: 256, gate: None },
        Field { name: "reuse", kind: Kind::Char, size: 256, gate: None },
        Field { name: "checkpoint", kind: Kind::Char, size: 256, gate: None },
        Field { name: "task", kind: Kind::Char, size: 30, gate: None },
        Field { name: "calculate_stress", kind: Kind::Bool, size: 4, gate: None },
        Field { name: "run_time", kind: Kind::I32, size: 4, gate: None },
        Field { name: "num_backup_iter", kind: Kind::I32, size: 4, gate: None },
        Field { name: "print_clock", kind: Kind::Bool, size: 4, gate: None },
        Field { name: "length_unit", kind: Kind::Char, size: 30, gate: None },
        Field { name: "mass_unit", kind: Kind::Char, size: 30, gate: None },
        Field { name: "time_unit", kind: Kind::Char, size: 30, gate: None },
        Field { name: "charge_unit", kind: Kind::Char, size: 30, gate: None },
        Field { name: "energy_unit", kind: Kind::Char, size: 30, gate: None },
        Field { name: "force_unit", kind: Kind::Char, size: 30, gate: None },
        Field { name: "velocity_unit", kind: Kind::Char, size: 30, gate: None },
        Field { name: "pressure_unit", kind: Kind::Char, size: 30, gate: None },
        Field { name: "inv_length_unit", kind: Kind::Char, size: 30, gate: None },
        Field { name: "frequency_unit", kind: Kind::Char, size: 30, gate: None },
        Field { name: "page_wvfns", kind: Kind::I32, size: 4, gate: None },
        Field { name: "rand_seed", kind: Kind::I32, size: 4, gate: None },
        Field { name: "data_distribution", kind: Kind::Char, size: 10, gate: None },
        Field { name: "opt_strategy", kind: Kind::Char, size: 10, gate: None },
        Field { name: "backup_interval", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "force_constant_unit", kind: Kind::Char, size: 30, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "volume_unit", kind: Kind::Char, size: 30, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "opt_strategy_bias", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "calculate_densdiff", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(3, 1)) },
        Field { name: "num_farms_requested", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(3, 1)) },
        Field { name: "num_proc_in_smp", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(3, 1)) },
        Field { name: "num_proc_in_smp_fine", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(3, 2)) },
        Field { name: "message_size", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(3, 2)) },
        Field { name: "requested_seed", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(3, 2)) },
        Field { name: "ir_intensity_unit", kind: Kind::Char, size: 30, gate: Some(FileVersion::new(4, 0)) },
        Field { name: "print_memory_usage", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(4, 1)) },
        Field { name: "write_formatted_potential", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(4, 1)) },
        Field { name: "write_formatted_density", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(4, 1)) },
        Field { name: "dipole_unit", kind: Kind::Char, size: 30, gate: Some(FileVersion::new(4, 1)) },
        Field { name: "efield_unit", kind: Kind::Char, size: 30, gate: Some(FileVersion::new(4, 1)) },
        Field { name: "calc_molecular_dipole", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(4, 1)) },
        Field { name: "write_orbitals", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(4, 2)) },
        Field { name: "calculate_elf", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(4, 3)) },
        Field { name: "write_formatted_elf", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(4, 3)) },
        Field { name: "cml_output", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(4, 3)) },
        Field { name: "cml_filename", kind: Kind::Char, size: 256, gate: Some(FileVersion::new(4, 3)) },
        Field { name: "calculate_Hirshfeld", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(5, 0)) },
        Field { name: "entropy_unit", kind: Kind::Char, size: 30, gate: Some(FileVersion::new(5, 5)) },
        Field { name: "write_cif_structure", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(5, 5)) },
        Field { name: "write_cell_structure", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(5, 5)) },
        Field { name: "write_bib", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(6, 0)) },
        Field { name: "end_header:END_GENERAL", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_XC", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "xc_functional", kind: Kind::Char, size: 15, gate: None },
        Field { name: "xc_vxc_deriv_epsilon", kind: Kind::F64, size: 8, gate: None },
        Field { name: "nlxc_page_ex_pot", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "nlxc_ppd_integral", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "nlxc_ppd_size_x", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "nlxc_ppd_size_y", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "nlxc_ppd_size_z", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "nlxc_impose_trs", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "nlxc_exchange_reflect_kpts", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "nlxc_k_scrn_den_function", kind: Kind::Char, size: 30, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "nlxc_k_scrn_averaging_scheme", kind: Kind::Char, size: 30, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "nlxc_re_est_k_scrn", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "nlxc_calc_full_ex_pot", kind: Kind::Bool, size: 4, gate: None },
        Field { name: "nlxc_div_corr_on", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(4, 0)) },
        Field { name: "nlxc_div_corr_S_width", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(4, 0)) },
        Field { name: "nlxc_div_corr_tol", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(4, 0)) },
        Field { name: "nlxc_div_corr_npts_step", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(4, 0)) },
        Field { name: "sedc_apply", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(5, 0)) },
        Field { name: "sedc_scheme", kind: Kind::Char, size: 10, gate: Some(FileVersion::new(5, 0)) },
        Field { name: "nlxc_exchange_screening", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(5, 0)) },
        Field { name: "nlxc_exchange_fraction", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(6, 1)) },
        Field { name: "end_header:END_XC", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_PSPOT", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "pspot_nonlocal_type", kind: Kind::Char, size: 10, gate: None },
        Field { name: "pspot_beta_phi_type", kind: Kind::Char, size: 10, gate: None },
        Field { name: "end_header:END_PSPOT", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_BASIS", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "basis_precision", kind: Kind::Char, size: 10, gate: None },
        Field { name: "cut_off_energy", kind: Kind::F64, size: 8, gate: None },
        Field { name: "grid_scale", kind: Kind::F64, size: 8, gate: None },
        Field { name: "fine_gmax", kind: Kind::F64, size: 8, gate: None },
        Field { name: "finite_basis_corr", kind: Kind::I32, size: 4, gate: None },
        Field { name: "basis_de_dloge", kind: Kind::F64, size: 8, gate: None },
        Field { name: "finite_basis_npoints", kind: Kind::I32, size: 4, gate: None },
        Field { name: "finite_basis_spacing", kind: Kind::F64, size: 8, gate: None },
        Field { name: "fixed_npw", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "fine_grid_scale", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(4, 0)) },
        Field { name: "fft_max_prime_factor", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(4, 4)) },
        Field { name: "end_header:END_BASIS", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_ELECTRONIC", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "ceiling(nelectrons)", kind: Kind::I32, size: 4, gate: None },
        Field { name: "ceiling(nup)", kind: Kind::I32, size: 4, gate: None },
        Field { name: "ceiling(ndown)", kind: Kind::I32, size: 4, gate: None },
        Field { name: "nspins", kind: Kind::I32, size: 4, gate: None },
        Field { name: "nbands", kind: Kind::I32, size: 4, gate: None },
        Field { name: "elec_temp", kind: Kind::F64, size: 8, gate: None },
        Field { name: "ceiling(spin)", kind: Kind::I32, size: 4, gate: None },
        Field { name: "ceiling(charge)", kind: Kind::I32, size: 4, gate: None },
        Field { name: "spin_polarized", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(3, 2)) },
        Field { name: "electronic_minimizer", kind: Kind::Char, size: 10, gate: Some(FileVersion::new(3, 2)) },
        Field { name: "nelectrons", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(4, 1)) },
        Field { name: "nup", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(4, 1)) },
        Field { name: "ndown", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(4, 1)) },
        Field { name: "spin", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(4, 1)) },
        Field { name: "charge", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(4, 1)) },
        Field { name: "end_header:END_ELECTRONIC", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_ELEC_MIN", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "max_sd_steps", kind: Kind::I32, size: 4, gate: None },
        Field { name: "max_cg_steps", kind: Kind::I32, size: 4, gate: None },
        Field { name: "max_diis_steps", kind: Kind::I32, size: 4, gate: None },
        Field { name: "metals_method", kind: Kind::Char, size: 4, gate: None },
        Field { name: "elec_energy_tol", kind: Kind::F64, size: 8, gate: None },
        Field { name: "elec_eigenvalue_tol", kind: Kind::F64, size: 8, gate: None },
        Field { name: "elec_convergence_win", kind: Kind::I32, size: 4, gate: None },
        Field { name: "max_SCF_cycles", kind: Kind::I32, size: 4, gate: None },
        Field { name: "spin_fix", kind: Kind::I32, size: 4, gate: None },
        Field { name: "fix_occupancy", kind: Kind::Bool, size: 4, gate: None },
        Field { name: "smearing_scheme", kind: Kind::Char, size: 20, gate: None },
        Field { name: "smearing_width", kind: Kind::F64, size: 8, gate: None },
        Field { name: "efermi_tol", kind: Kind::F64, size: 8, gate: None },
        Field { name: "num_occ_cycles", kind: Kind::I32, size: 4, gate: None },
        Field { name: "elec_dump_file", kind: Kind::Char, size: 256, gate: None },
        Field { name: "num_dump_cycles", kind: Kind::I32, size: 4, gate: None },
        Field { name: "elec_restore_file", kind: Kind::Char, size: 256, gate: None },
        Field { name: "elec_force_tol", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(4, 1)) },
        Field { name: "dipole_correction", kind: Kind::Char, size: 20, gate: Some(FileVersion::new(6, 1)) },
        Field { name: "dipole_dir", kind: Kind::Char, size: 1, gate: Some(FileVersion::new(6, 1)) },
        Field { name: "end_header:END_ELEC_MIN", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_DM", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "mixing_scheme", kind: Kind::Char, size: 15, gate: None },
        Field { name: "mix_history_length", kind: Kind::I32, size: 4, gate: None },
        Field { name: "mix_charge_amp", kind: Kind::F64, size: 8, gate: None },
        Field { name: "mix_charge_gmax", kind: Kind::F64, size: 8, gate: None },
        Field { name: "mix_spin_amp", kind: Kind::F64, size: 8, gate: None },
        Field { name: "mix_spin_gmax", kind: Kind::F64, size: 8, gate: None },
        Field { name: "mix_cut_off_energy", kind: Kind::F64, size: 8, gate: None },
        Field { name: "mix_metric_q", kind: Kind::F64, size: 8, gate: None },
        Field { name: "end_header:END_DM", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_POPN", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "popn_calculate", kind: Kind::Bool, size: 4, gate: None },
        Field { name: "popn_bond_cutoff", kind: Kind::F64, size: 8, gate: None },
        Field { name: "pdos_calculate_weights", kind: Kind::Bool, size: 4, gate: None },
        Field { name: "end_header:END_POPN", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_BS", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "bs_max_iter", kind: Kind::I32, size: 4, gate: None },
        Field { name: "bs_max_cg_steps", kind: Kind::I32, size: 4, gate: None },
        Field { name: "bs_nbands", kind: Kind::I32, size: 4, gate: None },
        Field { name: "bs_eigenvalue_tol", kind: Kind::F64, size: 8, gate: None },
        Field { name: "bs_xc_functional", kind: Kind::Char, size: 15, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "bs_re_est_k_scrn", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "bs_write_eigenvalues", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(4, 4)) },
        Field { name: "end_header:END_BS", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_GEOM", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "geom_method", kind: Kind::Char, size: 15, gate: None },
        Field { name: "geom_max_iter", kind: Kind::I32, size: 4, gate: None },
        Field { name: "geom_energy_tol", kind: Kind::F64, size: 8, gate: None },
        Field { name: "geom_force_tol", kind: Kind::F64, size: 8, gate: None },
        Field { name: "geom_disp_tol", kind: Kind::F64, size: 8, gate: None },
        Field { name: "geom_stress_tol", kind: Kind::F64, size: 8, gate: None },
        Field { name: "geom_convergence_win", kind: Kind::I32, size: 4, gate: None },
        Field { name: "geom_modulus_est", kind: Kind::F64, size: 8, gate: None },
        Field { name: "geom_frequency_est", kind: Kind::F64, size: 8, gate: None },
        Field { name: "geom_spin_fix", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(4, 0)) },
        Field { name: "geom_linmin_tol", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(4, 0)) },
        Field { name: "geom_use_linmin", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(5, 5)) },
        Field { name: "geom_lbfgs_max_updates", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(6, 0)) },
        Field { name: "geom_tpsd_init_stepsize", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(6, 1)) },
        Field { name: "geom_tpsd_iterchange", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(6, 1)) },
        Field { name: "end_header:END_GEOM", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_MD", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "md_num_iter", kind: Kind::I32, size: 4, gate: None },
        Field { name: "md_delta_t", kind: Kind::F64, size: 8, gate: None },
        Field { name: "md_ensemble", kind: Kind::Char, size: 3, gate: None },
        Field { name: "md_temperature", kind: Kind::F64, size: 8, gate: None },
        Field { name: "md_thermostat", kind: Kind::Char, size: 11, gate: None },
        Field { name: "MD_LEGACY_1", kind: Kind::F64, size: 8, gate: None },
        Field { name: "MD_LEGACY_2", kind: Kind::F64, size: 8, gate: None },
        Field { name: "md_extrap", kind: Kind::Char, size: 6, gate: None },
        Field { name: "md_extrap_fit", kind: Kind::Bool, size: 4, gate: None },
        Field { name: "md_damping_scheme", kind: Kind::Char, size: 16, gate: None },
        Field { name: "md_damping_reset", kind: Kind::I32, size: 4, gate: None },
        Field { name: "md_opt_damped_delta_t", kind: Kind::Bool, size: 4, gate: None },
        Field { name: "md_elec_energy_tol", kind: Kind::F64, size: 8, gate: None },
        Field { name: "md_elec_eigenvalue_tol", kind: Kind::F64, size: 8, gate: None },
        Field { name: "md_elec_convergence_win", kind: Kind::I32, size: 4, gate: None },
        Field { name: "md_barostat", kind: Kind::Char, size: 20, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "md_ion_t", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "md_cell_t", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "md_nhc_length", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "md_use_pathint", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(3, 1)) },
        Field { name: "md_num_beads", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(3, 1)) },
        Field { name: "md_pathint_staging", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(4, 0)) },
        Field { name: "md_pathint_num_stages", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(4, 0)) },
        Field { name: "md_sample_iter", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(4, 0)) },
        Field { name: "md_eqm_method", kind: Kind::Char, size: 10, gate: Some(FileVersion::new(4, 1)) },
        Field { name: "md_eqm_ion_t", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(4, 1)) },
        Field { name: "md_eqm_cell_t", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(4, 1)) },
        Field { name: "md_eqm_t", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(4, 1)) },
        Field { name: "md_elec_force_tol", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(4, 1)) },
        Field { name: "md_pathint_init", kind: Kind::Char, size: 10, gate: Some(FileVersion::new(5, 5)) },
        Field { name: "md_use_plumed", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(6, 0)) },
        Field { name: "end_header:END_MD", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_OPTICS", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "optics_nbands", kind: Kind::I32, size: 4, gate: None },
        Field { name: "optics_xc_functional", kind: Kind::Char, size: 15, gate: None },
        Field { name: "end_header:END_OPTICS", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_TSSEARCH", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "tssearch_method", kind: Kind::Char, size: 6, gate: None },
        Field { name: "tssearch_lstqst_protocol", kind: Kind::Char, size: 17, gate: None },
        Field { name: "tssearch_qst_max_iter", kind: Kind::I32, size: 4, gate: None },
        Field { name: "tssearch_cg_max_iter", kind: Kind::I32, size: 4, gate: None },
        Field { name: "tssearch_force_tol", kind: Kind::F64, size: 8, gate: None },
        Field { name: "tssearch_disp_tol", kind: Kind::F64, size: 8, gate: None },
        Field { name: "end_header:END_TSSEARCH", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_PHONON", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "phonon_const_basis", kind: Kind::I32, size: 4, gate: None },
        Field { name: "phonon_energy_tol", kind: Kind::F64, size: 8, gate: None },
        Field { name: "phonon_max_cg_steps", kind: Kind::I32, size: 4, gate: None },
        Field { name: "phonon_max_cycles", kind: Kind::I32, size: 4, gate: None },
        Field { name: "phonon_convergence_win", kind: Kind::I32, size: 4, gate: None },
        Field { name: "phonon_preconditioner", kind: Kind::Char, size: 8, gate: None },
        Field { name: "phonon_use_kpoint_symmetry", kind: Kind::Bool, size: 4, gate: None },
        Field { name: "phonon_dos_spacing", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "phonon_finite_disp", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "phonon_method", kind: Kind::Char, size: 20, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "phonon_calc_lo_to_splitting", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "phonon_sum_rule", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "calculate_born_charges", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "born_charge_sum_rule", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(3, 0)) },
        Field { name: "phonon_calculate_dos", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(3, 1)) },
        Field { name: "phonon_dos_limit", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(3, 1)) },
        Field { name: "phonon_force_constant_cutoff", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(3, 1)) },
        Field { name: "phonon_fine_method", kind: Kind::Char, size: 20, gate: Some(FileVersion::new(3, 1)) },
        Field { name: "phonon_method_2", kind: Kind::Char, size: 20, gate: Some(FileVersion::new(3, 1)) },
        Field { name: "phonon_force_constant_cut_scale", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(4, 4)) },
        Field { name: "phonon_sum_rule_method", kind: Kind::Char, size: 16, gate: Some(FileVersion::new(4, 4)) },
        Field { name: "calculate_raman", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(4, 4)) },
        Field { name: "phonon_fine_cutoff_method", kind: Kind::Char, size: 10, gate: Some(FileVersion::new(5, 0)) },
        Field { name: "phonon_dfpt_method", kind: Kind::Char, size: 12, gate: Some(FileVersion::new(6, 0)) },
        Field { name: "raman_range_low", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(6, 0)) },
        Field { name: "raman_range_high", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(6, 0)) },
        Field { name: "phonon_write_force_constants", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(6, 0)) },
        Field { name: "phonon_write_dynamical", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(6, 0)) },
        Field { name: "end_header:END_PHONON", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_EFIELD", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "efield_max_cg_steps", kind: Kind::I32, size: 4, gate: None },
        Field { name: "efield_max_cycles", kind: Kind::I32, size: 4, gate: None },
        Field { name: "efield_convergence_win", kind: Kind::I32, size: 4, gate: None },
        Field { name: "efield_energy_tol", kind: Kind::F64, size: 8, gate: None },
        Field { name: "efield_calc_ion_permittivity", kind: Kind::Bool, size: 4, gate: None },
        Field { name: "efield_ignore_molec_modes", kind: Kind::Char, size: 20, gate: None },
        Field { name: "excited_state_scissors", kind: Kind::F64, size: 8, gate: None },
        Field { name: "efield_freq_spacing", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(4, 0)) },
        Field { name: "efield_oscillator_q", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(4, 0)) },
        Field { name: "efield_dfpt_method", kind: Kind::Char, size: 12, gate: Some(FileVersion::new(6, 0)) },
        Field { name: "end_header:END_EFIELD", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_THERMODYNAMICS", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "thermo_calculate_helmholtz", kind: Kind::Bool, size: 4, gate: None },
        Field { name: "thermo_t_start", kind: Kind::F64, size: 8, gate: None },
        Field { name: "thermo_t_stop", kind: Kind::F64, size: 8, gate: None },
        Field { name: "thermo_t_npoints", kind: Kind::I32, size: 4, gate: None },
        Field { name: "thermo_t_spacing", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(3, 2)) },
        Field { name: "end_header:END_THERMODYNAMICS", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_WANNIER", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "wannier_spread_type", kind: Kind::Char, size: 10, gate: None },
        Field { name: "dummy_real_1", kind: Kind::F64, size: 8, gate: None },
        Field { name: "dummy_real_2", kind: Kind::F64, size: 8, gate: None },
        Field { name: "dummy_real_3", kind: Kind::F64, size: 8, gate: None },
        Field { name: "wannier_min_algor", kind: Kind::Char, size: 10, gate: None },
        Field { name: "wannier_max_sd_steps", kind: Kind::I32, size: 4, gate: None },
        Field { name: "wannier_sd_step", kind: Kind::F64, size: 8, gate: None },
        Field { name: "wannier_ion_moments", kind: Kind::Bool, size: 4, gate: None },
        Field { name: "wannier_ion_rmax", kind: Kind::F64, size: 8, gate: None },
        Field { name: "wannier_ion_cut", kind: Kind::Bool, size: 4, gate: None },
        Field { name: "wannier_ion_cut_fraction", kind: Kind::F64, size: 8, gate: None },
        Field { name: "wannier_spread_tol", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(4, 0)) },
        Field { name: "wannier_print_cube", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(4, 0)) },
        Field { name: "wannier_ion_cut_tol", kind: Kind::F64, size: 8, gate: Some(FileVersion::new(4, 0)) },
        Field { name: "wannier_restart", kind: Kind::Char, size: 10, gate: Some(FileVersion::new(4, 0)) },
        Field { name: "wannier_ion_cmoments", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(4, 0)) },
        Field { name: "end_header:END_WANNIER", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_MAGRES", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "magres_task", kind: Kind::Char, size: 20, gate: None },
        Field { name: "magres_method", kind: Kind::Char, size: 10, gate: None },
        Field { name: "magres_max_cg_steps", kind: Kind::I32, size: 4, gate: None },
        Field { name: "magres_convergence_win", kind: Kind::I32, size: 4, gate: None },
        Field { name: "magres_conv_tol", kind: Kind::F64, size: 8, gate: None },
        Field { name: "magres_xc_functional", kind: Kind::Char, size: 15, gate: None },
        Field { name: "magres_max_sc_cycles", kind: Kind::I32, size: 4, gate: Some(FileVersion::new(4, 3)) },
        Field { name: "magres_jcoupling_task", kind: Kind::Char, size: 15, gate: Some(FileVersion::new(4, 3)) },
        Field { name: "magres_write_response", kind: Kind::Bool, size: 4, gate: Some(FileVersion::new(4, 3)) },
        Field { name: "end_header:END_MAGRES", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_ELNES", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "elnes_nbands", kind: Kind::I32, size: 4, gate: None },
        Field { name: "elnes_xc_functional", kind: Kind::Char, size: 15, gate: None },
        Field { name: "elnes_eigenvalue_tol", kind: Kind::F64, size: 8, gate: None },
        Field { name: "end_header:END_ELNES", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_SPECTRAL", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "spectral_theory", kind: Kind::Char, size: 10, gate: None },
        Field { name: "spectral_task", kind: Kind::Char, size: 30, gate: None },
        Field { name: "spectral_max_iter", kind: Kind::I32, size: 4, gate: None },
        Field { name: "spectral_max_steps_per_iter", kind: Kind::I32, size: 4, gate: None },
        Field { name: "spectral_nbands", kind: Kind::I32, size: 4, gate: None },
        Field { name: "spectral_eigenvalue_tol", kind: Kind::F64, size: 8, gate: None },
        Field { name: "spectral_xc_functional", kind: Kind::Char, size: 15, gate: None },
        Field { name: "spectral_re_est_k_scrn", kind: Kind::Bool, size: 4, gate: None },
        Field { name: "spectral_write_eigenvalues", kind: Kind::Bool, size: 4, gate: None },
        Field { name: "end_header:END_SPECTRAL", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_TDDFT", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "tddft_on", kind: Kind::Bool, size: 4, gate: None },
        Field { name: "tddft_num_states", kind: Kind::I32, size: 4, gate: None },
        Field { name: "tddft_selected_state", kind: Kind::I32, size: 4, gate: None },
        Field { name: "tddft_eigenvalue_tol", kind: Kind::F64, size: 8, gate: None },
        Field { name: "tddft_convergence_win", kind: Kind::I32, size: 4, gate: None },
        Field { name: "tddft_max_iter", kind: Kind::I32, size: 4, gate: None },
        Field { name: "tddft_nextra_states", kind: Kind::I32, size: 4, gate: None },
        Field { name: "tddft_xc_functional", kind: Kind::Char, size: 15, gate: None },
        Field { name: "tddft_method", kind: Kind::Char, size: 20, gate: None },
        Field { name: "tddft_eigenvalue_method", kind: Kind::Char, size: 20, gate: None },
        Field { name: "tddft_approximation", kind: Kind::Char, size: 20, gate: None },
        Field { name: "end_header:END_TDDFT", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_GA", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "ga_pop_size", kind: Kind::I32, size: 4, gate: None },
        Field { name: "ga_max_gens", kind: Kind::I32, size: 4, gate: None },
        Field { name: "ga_mutate_rate", kind: Kind::F64, size: 8, gate: None },
        Field { name: "ga_mutate_amp", kind: Kind::F64, size: 8, gate: None },
        Field { name: "ga_fixed_N", kind: Kind::Bool, size: 4, gate: None },
        Field { name: "ga_bulk_slice", kind: Kind::Bool, size: 4, gate: None },
        Field { name: "end_header:END_GA", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "header:BEGIN_DEVEL", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "devel_code", kind: Kind::Char, size: 80, gate: None },
        Field { name: "end_header:END_DEVEL", kind: Kind::Tag, size: 30, gate: None },
        Field { name: "end_header:END_PARAMETERS_DUMP", kind: Kind::Tag, size: 30, gate: None },
];

/// The parsed parameters section.
///
/// Payloads are in spec order. A payload that was not present in the file
/// (a closed version gate) is `None`. The writer fills those with default
/// encodings.
#[derive(Debug)]
pub struct RawParams {
    /// The file version, parsed from the version record.
    pub version: FileVersion,
    /// One entry per `PARAM_FIELDS` entry, in the same order.
    pub payloads: Vec<Option<Vec<u8>>>,
}

impl RawParams {
    /// Finds the payload index of a field by key.
    pub fn index_of(key: &str) -> Option<usize> {
        PARAM_FIELDS.iter().position(|f| f.key() == key)
    }

    /// The raw payload of a field, if present.
    pub fn raw(&self, key: &str) -> Option<&[u8]> {
        let i = Self::index_of(key)?;
        self.payloads.get(i)?.as_deref()
    }

    /// Reads a field as an i32.
    pub fn i32(&self, key: &str) -> CResult<Option<i32>> {
        let p = self.raw(key).ok_or_else(|| CheckError::Section(key.into()))?;
        parse_i32s(p).map(|v| v.into_iter().next())
    }

    /// Reads a field as an f64.
    pub fn f64(&self, key: &str) -> CResult<Option<f64>> {
        let p = self.raw(key).ok_or_else(|| CheckError::Section(key.into()))?;
        parse_f64s(p).map(|v| v.into_iter().next())
    }

    /// Reads a field as a boolean (4-byte logical).
    pub fn boolean(&self, key: &str) -> CResult<Option<bool>> {
        let p = self.raw(key).ok_or_else(|| CheckError::Section(key.into()))?;
        parse_logical4s(p).map(|v| v.into_iter().next())
    }

    /// Reads a character field as trimmed bytes.
    pub fn bytes(&self, key: &str) -> Option<&[u8]> {
        self.raw(key).map(char_field)
    }

    /// Reads a character field as lossy UTF-8.
    pub fn text(&self, key: &str) -> Option<&str> {
        self.bytes(key).map(char_str)
    }

    /// Replaces a character field. Pads the right with spaces.
    pub fn set_text(&mut self, key: &str, value: &str) -> CResult<()> {
        let i = Self::index_of(key).ok_or_else(|| CheckError::Section(key.into()))?;
        if self.payloads.len() <= i {
            self.payloads.resize(i + 1, None);
        }
        let n = PARAM_FIELDS[i].size;
        let bytes = value.as_bytes();
        if bytes.len() > n {
            return Err(CheckError::Section(format!(
                "{key}: {n} bytes is too small for a {len}-byte value",
                len = bytes.len()
            )));
        }
        let mut payload = bytes.to_vec();
        payload.resize(n, b' ');
        self.payloads[i] = Some(payload);
        Ok(())
    }

    /// Replaces an i32 field.
    pub fn set_i32(&mut self, key: &str, value: i32) -> CResult<()> {
        self.set_payload(key, &value.to_be_bytes())
    }

    /// Replaces an f64 field.
    pub fn set_f64(&mut self, key: &str, value: f64) -> CResult<()> {
        self.set_payload(key, &value.to_be_bytes())
    }

    /// Replaces a boolean field.
    pub fn set_boolean(&mut self, key: &str, value: bool) -> CResult<()> {
        self.set_payload(key, &((value as i32).to_be_bytes()))
    }

    fn set_payload(&mut self, key: &str, bytes: &[u8]) -> CResult<()> {
        let i = Self::index_of(key).ok_or_else(|| CheckError::Section(key.into()))?;
        if self.payloads.len() <= i {
            self.payloads.resize(i + 1, None);
        }
        let n = PARAM_FIELDS[i].size;
        if bytes.len() != n {
            return Err(CheckError::Section(format!(
                "{key}: payload is {len} bytes, expected {n}",
                len = bytes.len()
            )));
        }
        self.payloads[i] = Some(bytes.to_vec());
        Ok(())
    }
}

/// Reads the parameters section from a record stream.
///
/// `is_castep_bin` skips the leading `CASTEP_BIN` magic record.
pub fn read_params(
    reader: &mut RecordReader<&[u8]>,
    is_castep_bin: bool,
    layout: &ParamsLayout,
) -> CResult<RawParams> {
    if is_castep_bin {
        let magic = reader.read_fixed(10)?;
        if char_field(&magic) != b"CASTEP_BIN" {
            return Err(CheckError::Section(
                "expected the CASTEP_BIN magic record".into(),
            ));
        }
    }
    let first = reader.read_fixed(PARAM_FIELDS[0].size)?;
    if char_field(&first) != b"BEGIN_PARAMETERS_DUMP" {
        return Err(CheckError::Section(
            "expected the BEGIN_PARAMETERS_DUMP tag".into(),
        ));
    }
    let version_rec = reader.read_fixed(PARAM_FIELDS[1].size)?;
    let version = parse_version_record(&version_rec)?;
    check_file_version(version)?;

    let mut payloads: Vec<Option<Vec<u8>>> = Vec::with_capacity(PARAM_FIELDS.len());
    payloads.push(Some(first));
    payloads.push(Some(version_rec));

    for field in &PARAM_FIELDS[2..] {
        if field.gate.is_some_and(|g| !gate_open(version, g)) {
            payloads.push(None);
            continue;
        }
        let size = record_width(field, layout);
        let payload = reader.read_fixed(size)?;
        if field
            .tag()
            .is_some_and(|tag| char_field(&payload) != tag.as_bytes())
        {
            return Err(CheckError::Section(format!(
                "tag is {found}, expected {tag}",
                found = char_str(&payload),
                tag = field.tag().unwrap()
            )));
        }
        payloads.push(Some(payload));
    }

    Ok(RawParams { version, payloads })
}

/// The default encoding of an absent record, by kind.
fn default_payload(field: &Field) -> Vec<u8> {
    match field.kind {
        Kind::I32 => vec![0u8; 4],
        Kind::F64 => vec![0u8; 8],
        Kind::Bool => vec![0u8; 4],
        Kind::Char => vec![b' '; field.size],
        Kind::Tag => pad_right(field.tag().unwrap_or("").as_bytes(), field.size),
    }
}

fn pad_right(bytes: &[u8], n: usize) -> Vec<u8> {
    let mut out = bytes.to_vec();
    out.resize(n, b' ');
    out
}

/// The record width of a field under a layout.
///
/// `devel_code` follows the layout. Every other field keeps its table width.
fn record_width(field: &Field, layout: &ParamsLayout) -> usize {
    if field.name == "devel_code" {
        layout.devel_code_width() as usize
    } else {
        field.size
    }
}

/// Adapts the stored `devel_code` payload to the target width.
///
/// Space-pads a short value. Refuses to truncate a value that exceeds the
/// target width, because that would drop developer code.
fn adapt_devel_code(payload: &[u8], target: u32) -> CResult<Vec<u8>> {
    let content = char_field(payload);
    if content.len() > target as usize {
        return Err(CheckError::LayoutOverflow {
            field: "devel_code",
            len: content.len() as u32,
            target,
        });
    }
    Ok(pad_right(content, target as usize))
}

/// Writes the parameters section in the target layout.
///
/// Stored payloads are replayed byte-exact. The version record is the target
/// version record. Absent records get default encodings. The `devel_code`
/// record is emitted at the layout width.
pub fn write_params<W: std::io::Write>(
    writer: &mut RecordWriter<W>,
    params: &RawParams,
    is_castep_bin: bool,
    layout: &ParamsLayout,
) -> CResult<()> {
    if is_castep_bin {
        writer.write_record(b"CASTEP_BIN")?;
    }
    for (i, field) in PARAM_FIELDS.iter().enumerate() {
        match field.kind {
            Kind::Tag => {
                writer.write_char_padded(field.tag().unwrap_or(""), field.size)?;
            }
            _ => {
                if i == 1 {
                    // The version record is always the target version string.
                    writer.write_record(&target_version_record())?;
                    continue;
                }
                if field.name == "devel_code" {
                    let payload = match params.payloads.get(i) {
                        Some(Some(payload)) => adapt_devel_code(payload, layout.devel_code_width())?,
                        _ => vec![b' '; layout.devel_code_width() as usize],
                    };
                    writer.write_record(&payload)?;
                    continue;
                }
                match params.payloads.get(i) {
                    Some(Some(payload)) => writer.write_record(payload)?,
                    _ => writer.write_record(&default_payload(field))?,
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_is_340_records() {
        assert_eq!(PARAM_FIELDS.len(), 340);
    }

    #[test]
    fn every_field_has_a_positive_size() {
        assert!(PARAM_FIELDS.iter().all(|f| f.size > 0));
    }

    #[test]
    fn field_keys_are_unique() {
        use std::collections::HashSet;
        let keys: Vec<&str> = PARAM_FIELDS.iter().map(|f| f.key()).collect();
        let set: HashSet<&str> = keys.iter().copied().collect();
        assert_eq!(set.len(), keys.len());
    }

    #[test]
    fn typed_getters_roundtrip() {
        let mut p = RawParams {
            version: FileVersion::new(6, 11),
            payloads: vec![],
        };
        let key = "seedname";
        p.set_text(key, "abc").unwrap();
        assert_eq!(p.text(key), Some("abc"));
        let key = "iprint";
        p.set_i32(key, 2).unwrap();
        assert_eq!(p.i32(key).unwrap(), Some(2));
        let key = "elec_temp";
        p.set_f64(key, 0.1).unwrap();
        assert_eq!(p.f64(key).unwrap(), Some(0.1));
        let key = "calculate_stress";
        p.set_boolean(key, true).unwrap();
        assert_eq!(p.boolean(key).unwrap(), Some(true));
    }

    #[test]
    fn setter_rejects_oversized_text() {
        let mut p = RawParams {
            version: FileVersion::new(6, 11),
            payloads: vec![],
        };
        assert!(p.set_text("xc_functional", "x".repeat(40).as_str()).is_err());
        assert!(p.set_i32("iprint", 0).is_ok());
        let err = p.set_i32("seedname", 1).unwrap_err();
        assert!(matches!(err, CheckError::Section(_)));
    }

    #[test]
    fn missing_keys_are_errors() {
        let p = RawParams {
            version: FileVersion::new(6, 11),
            payloads: vec![],
        };
        assert!(p.i32("no_such_field").is_err());
        assert!(p.raw("no_such_field").is_none());
    }
}

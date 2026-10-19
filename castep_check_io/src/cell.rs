//! The cell sections of a CASTEP model file.
//!
//! Each section is a run of `[256-byte tag][value record]` pairs, written
//! by `cell_dump_cell` and `cell_dump_global`. A section repeats twice in a
//! model file: the live cell and the original cell.
//!
//! Value sizes derive from the counts read earlier in the section
//! (`num_species`, `max_ions_in_species`, k-point counts, ...). Four value
//! pairs are conditional on their count being nonzero: `SYMMETRY_*` on
//! `num_symmetry_operations`, `CRYSTAL_SYMMETRY_*` on
//! `num_crystal_symmetry_operations`, `IONIC_CONSTRAINTS` on
//! `num_ionic_constraints`, and `PHONON_GAMMA_DIRECTIONS` on
//! `num_phonon_gamma_directions`.

use crate::error::{CResult, CheckError};
use crate::record::{char_field, char_str, parse_f64s, parse_i32s, parse_logical4s, RecordReader, RecordWriter};

/// The 256-byte tag record size.
pub const TAG_SIZE: usize = 256;

/// The counts a reader keeps as it walks a cell section.
#[derive(Debug, Clone, Copy, Default)]
pub struct Counts {
    pub ns: i32,
    pub mi: i32,
    pub nkpts: i32,
    pub nso: i32,
    pub nco: i32,
    pub nic: i32,
    pub nk_bs: i32,
    pub nk_spectral: i32,
    pub nk_phonon: i32,
    pub npgd: i32,
    pub nk_phonon_fine: i32,
    pub nk_scf: i32,
    pub nk_magres: i32,
    pub nk_elnes: i32,
    pub nk_optics: i32,
}

/// The size of a cell value record, as a function of the counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    /// A fixed byte size.
    Fixed(usize),
    /// `ns * 4` i32 values.
    PerNsI32,
    /// `ns * 8` f64 values.
    PerNsF64,
    /// `ns * n` bytes of char fields.
    PerNsChar(usize),
    /// `mi * ns * 4` i32 values.
    PerMiNsI32,
    /// `3 * mi * ns * 4` logical values.
    ThreeMiNsLogical,
    /// `3 * mi * ns * 8` f64 values.
    ThreeMiNsF64,
    /// `mi * ns * 8` f64 values.
    PerMiNsF64,
    /// `4 * mi * ns * 8` f64 values.
    FourMiNsF64,
    /// `(mi * ns) ^ 2 * 4` i32 values.
    MiNsSqI32,
    /// `n * 3 * 8` f64 k-point coordinates.
    KptF64x3,
    /// `n * 8` f64 k-point weights.
    KptF64,
    /// `n * 9 * 8` f64 symmetry rotations.
    SymF64x9,
    /// `n * 3 * 8` f64 symmetry displacements.
    SymF64x3,
    /// `n * mi * ns * 4` i32 equivalent-atom indices.
    SymMiNsI32,
    /// `nic * 3 * mi * ns * 8` f64 constraint values.
    NicMiNsF64x3,
    /// `npgd * 3 * 8` f64 gamma directions.
    PgdF64x3,
}

impl Size {
    /// Resolves the payload size against a counts snapshot.
    pub fn bytes(&self, c: &Counts) -> CResult<usize> {
        let n = |v: i32, what: &str| -> CResult<usize> {
            if v < 0 {
                return Err(CheckError::Section(format!(
                    "cell: negative count {what} = {v}"
                )));
            }
            Ok(v as usize)
        };
        Ok(match self {
            Size::Fixed(b) => *b,
            Size::PerNsI32 => n(c.ns, "num_species")? * 4,
            Size::PerNsF64 => n(c.ns, "num_species")? * 8,
            Size::PerNsChar(e) => n(c.ns, "num_species")? * e,
            Size::PerMiNsI32 => n(c.ns, "num_species")? * n(c.mi, "max_ions_in_species")? * 4,
            Size::ThreeMiNsLogical => {
                3 * n(c.ns, "num_species")? * n(c.mi, "max_ions_in_species")? * 4
            }
            Size::ThreeMiNsF64 => {
                3 * n(c.ns, "num_species")? * n(c.mi, "max_ions_in_species")? * 8
            }
            Size::PerMiNsF64 => n(c.ns, "num_species")? * n(c.mi, "max_ions_in_species")? * 8,
            Size::FourMiNsF64 => {
                4 * n(c.ns, "num_species")? * n(c.mi, "max_ions_in_species")? * 8
            }
            Size::MiNsSqI32 => {
                let p = n(c.ns, "num_species")? * n(c.mi, "max_ions_in_species")?;
                p * p * 4
            }
            Size::KptF64x3 => n(c.kn(), "kpoints")? * 24,
            Size::KptF64 => n(c.kn(), "kpoints")? * 8,
            Size::SymF64x9 => 9 * n(c.sym(), "symmetry operations")? * 8,
            Size::SymF64x3 => 3 * n(c.sym(), "symmetry operations")? * 8,
            Size::SymMiNsI32 => {
                n(c.sym(), "symmetry operations")?
                    * n(c.ns, "num_species")?
                    * n(c.mi, "max_ions_in_species")?
                    * 4
            }
            Size::NicMiNsF64x3 => {
                n(c.nic, "num_ionic_constraints")?
                    * 3
                    * n(c.ns, "num_species")?
                    * n(c.mi, "max_ions_in_species")?
                    * 8
            }
            Size::PgdF64x3 => 3 * n(c.npgd, "phonon gamma directions")? * 8,
        })
    }
}

impl Counts {
    /// The active per-task k-point count for a size expression.
    fn kn(&self) -> i32 {
        self.nkpts
    }

    /// The symmetry count, per call site.
    fn sym(&self) -> i32 {
        self.nso
    }
}

/// One `[tag][value]` record of a cell section.
#[derive(Debug, Clone, Copy)]
pub struct CellRec {
    /// The 256-byte tag text.
    pub tag: &'static str,
    /// The value key. `""` for a bare tag record.
    pub value: &'static str,
    /// The value size. Ignored for bare tag records.
    pub size: Size,
    /// Which count must be positive for the value pair to be present.
    /// `None` is always present.
    pub cond_key: Option<&'static str>,
}

/// The `BEGIN_UNIT_CELL`...`END_UNIT_CELL` record table.
pub static UNIT_CELL: &[CellRec] = &[
            CellRec { tag: "BEGIN_UNIT_CELL", value: "", size: Size::Fixed(0), cond_key: None },
            CellRec { tag: "CELL%REAL_LATTICE", value: "real_lattice", size: Size::Fixed(72), cond_key: None },
            CellRec { tag: "CELL%RECIP_LATTICE", value: "recip_lattice", size: Size::Fixed(72), cond_key: None },
            CellRec { tag: "CELL%VOLUME", value: "volume", size: Size::Fixed(8), cond_key: None },
            CellRec { tag: "CELL%NUM_SPECIES", value: "num_species", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "CELL%NUM_IONS", value: "num_ions", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "CELL%MAX_IONS_IN_SPECIES", value: "max_ions_in_species", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "CELL_VERSION_NUMBER", value: "cell_version", size: Size::Fixed(8), cond_key: None },
            CellRec { tag: "CELL%NUM_IONS_IN_SPECIES", value: "num_ions_in_species", size: Size::PerNsI32, cond_key: None },
            CellRec { tag: "CELL%IONIC_POSITIONS", value: "ionic_positions", size: Size::ThreeMiNsF64, cond_key: None },
            CellRec { tag: "CELL%IONIC_VELOCITIES", value: "ionic_velocities", size: Size::ThreeMiNsF64, cond_key: None },
            CellRec { tag: "CELL%IONIC_CHARGE", value: "ionic_charge", size: Size::PerNsI32, cond_key: None },
            CellRec { tag: "CELL%IONIC_CHARGE_REAL", value: "ionic_charge_real", size: Size::PerNsF64, cond_key: None },
            CellRec { tag: "CELL%ATOM_MOVE", value: "atom_move", size: Size::ThreeMiNsLogical, cond_key: None },
            CellRec { tag: "CELL%SPECIES_MASS", value: "species_mass", size: Size::PerNsF64, cond_key: None },
            CellRec { tag: "CELL%SPECIES_SYMBOL", value: "species_symbol", size: Size::PerNsChar(8), cond_key: None },
            CellRec { tag: "CELL%SPECIES_POT", value: "species_pot", size: Size::PerNsChar(256), cond_key: None },
            CellRec { tag: "CELL%SPECIES_LCAO_STATES", value: "species_lcao_states", size: Size::PerNsI32, cond_key: None },
            CellRec { tag: "CELL%ION_PACK_SPECIES", value: "ion_pack_species", size: Size::PerMiNsI32, cond_key: None },
            CellRec { tag: "CELL%ION_PACK_INDEX", value: "ion_pack_index", size: Size::PerMiNsI32, cond_key: None },
            CellRec { tag: "CELL%INITIAL_MAGNETIC_MOMENT", value: "initial_magnetic_moment", size: Size::PerMiNsF64, cond_key: None },
            CellRec { tag: "CELL%SUPERCELL_MATRIX", value: "supercell_matrix", size: Size::Fixed(36), cond_key: None },
            CellRec { tag: "CELL%SUPERCELL_CELL_INDEX", value: "supercell_cell_index", size: Size::PerMiNsI32, cond_key: None },
            CellRec { tag: "CELL%SUPERCELL_ION_INDEX", value: "supercell_ion_index", size: Size::PerMiNsI32, cond_key: None },
            CellRec { tag: "CELL%NUM_MIXTURE_ATOMS", value: "num_mixture_atoms", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "CELL%MIXTURE_WEIGHT", value: "mixture_weight", size: Size::PerMiNsF64, cond_key: None },
            CellRec { tag: "CELL%LIST_MIXTURE_ATOMS", value: "list_mixture_atoms", size: Size::MiNsSqI32, cond_key: None },
            CellRec { tag: "CELL%ATOM_INIT_MAGMOM", value: "atom_init_magmom", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "CELL%ATOM_INIT_SPIN", value: "atom_init_spin", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "CELL%INDX_MIXTURE_ATOMS", value: "indx_mixture_atoms", size: Size::PerMiNsI32, cond_key: None },
            CellRec { tag: "CELL%SPECIES_SPIN_ISOTOPE", value: "species_spin_isotope", size: Size::PerNsI32, cond_key: None },
            CellRec { tag: "CELL%SPECIES_GAMMA", value: "species_gamma", size: Size::PerNsF64, cond_key: None },
            CellRec { tag: "CELL%SPECIES_EFG_ISOTOPE", value: "species_efg_isotope", size: Size::PerNsI32, cond_key: None },
            CellRec { tag: "CELL%SPECIES_Q", value: "species_q", size: Size::PerNsF64, cond_key: None },
            CellRec { tag: "CELL%HUBBARD_U", value: "hubbard_u", size: Size::FourMiNsF64, cond_key: None },
            CellRec { tag: "CELL%HUBBARD_ALPHA", value: "hubbard_alpha", size: Size::FourMiNsF64, cond_key: None },
            CellRec { tag: "CELL%CHEMICAL_POTENTIAL", value: "chemical_potential", size: Size::PerNsF64, cond_key: None },
            CellRec { tag: "END_UNIT_CELL", value: "", size: Size::Fixed(0), cond_key: None },
];

/// The `BEGIN_CELL_GLOBAL`...`END_CELL_GLOBAL` record table.
pub static CELL_GLOBAL: &[CellRec] = &[
            CellRec { tag: "BEGIN_CELL_GLOBAL", value: "", size: Size::Fixed(0), cond_key: None },
            CellRec { tag: "NKPTS", value: "nkpts", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "KPOINTS", value: "kpoints", size: Size::KptF64x3, cond_key: None },
            CellRec { tag: "KPOINT_WEIGHTS", value: "kpoint_weights", size: Size::KptF64, cond_key: None },
            CellRec { tag: "NUM_SYMMETRY_OPERATIONS", value: "num_symmetry_operations", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "SYMMETRY_OPERATIONS", value: "symmetry_operations", size: Size::SymF64x9, cond_key: Some("num_symmetry_operations") },
            CellRec { tag: "SYMMETRY_DISPS", value: "symmetry_disps", size: Size::SymF64x3, cond_key: Some("num_symmetry_operations") },
            CellRec { tag: "SYMMETRY_EQUIV_ATOMS", value: "symmetry_equiv_atoms", size: Size::SymMiNsI32, cond_key: Some("num_symmetry_operations") },
            CellRec { tag: "NUM_CRYSTAL_SYMMETRY_OPERATIONS", value: "num_crystal_symmetry_operations", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "CRYSTAL_SYMMETRY_OPERATIONS", value: "crystal_symmetry_operations", size: Size::SymF64x9, cond_key: Some("num_crystal_symmetry_operations") },
            CellRec { tag: "CRYSTAL_SYMMETRY_DISPS", value: "crystal_symmetry_disps", size: Size::SymF64x3, cond_key: Some("num_crystal_symmetry_operations") },
            CellRec { tag: "CRYSTAL_SYMMETRY_EQUIV_ATOMS", value: "crystal_symmetry_equiv_atoms", size: Size::SymMiNsI32, cond_key: Some("num_crystal_symmetry_operations") },
            CellRec { tag: "CELL_SYMMETRY_ION_CONSTRAINTS", value: "cell_symmetry_ion_constraints", size: Size::ThreeMiNsLogical, cond_key: None },
            CellRec { tag: "NUM_IONIC_CONSTRAINTS", value: "num_ionic_constraints", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "IONIC_CONSTRAINTS", value: "ionic_constraints", size: Size::NicMiNsF64x3, cond_key: Some("num_ionic_constraints") },
            CellRec { tag: "NUM_CELL_CONSTRAINTS", value: "num_cell_constraints", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "CELL_CONSTRAINTS", value: "cell_constraints", size: Size::Fixed(24), cond_key: None },
            CellRec { tag: "EXTERNAL_PRESSURE", value: "external_pressure", size: Size::Fixed(48), cond_key: None },
            CellRec { tag: "EXTERNAL_EFIELD", value: "external_efield", size: Size::Fixed(24), cond_key: None },
            CellRec { tag: "NUM_BS_KPOINTS", value: "num_bs_kpoints", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "BS_KPOINTS", value: "bs_kpoints", size: Size::KptF64x3, cond_key: Some("num_bs_kpoints") },
            CellRec { tag: "BS_KPOINT_WEIGHTS", value: "bs_kpoint_weights", size: Size::KptF64, cond_key: Some("num_bs_kpoints") },
            CellRec { tag: "NUM_SPECTRAL_KPOINTS", value: "num_spectral_kpoints", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "SPECTRAL_KPOINTS", value: "spectral_kpoints", size: Size::KptF64x3, cond_key: Some("num_spectral_kpoints") },
            CellRec { tag: "SPECTRAL_KPOINT_WEIGHTS", value: "spectral_kpoint_weights", size: Size::KptF64, cond_key: Some("num_spectral_kpoints") },
            CellRec { tag: "NUM_PHONON_KPOINTS", value: "num_phonon_kpoints", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "PHONON_KPOINTS", value: "phonon_kpoints", size: Size::KptF64x3, cond_key: Some("num_phonon_kpoints") },
            CellRec { tag: "NUM_PHONON_GAMMA_DIRECTIONS", value: "num_phonon_gamma_directions", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "PHONON_GAMMA_DIRECTIONS", value: "phonon_gamma_directions", size: Size::PgdF64x3, cond_key: Some("num_phonon_gamma_directions") },
            CellRec { tag: "PHONON_KPOINT_WEIGHTS", value: "phonon_kpoint_weights", size: Size::KptF64, cond_key: Some("num_phonon_kpoints") },
            CellRec { tag: "NUM_PHONON_FINE_KPOINTS", value: "num_phonon_fine_kpoints", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "PHONON_FINE_KPOINTS", value: "phonon_fine_kpoints", size: Size::KptF64x3, cond_key: Some("num_phonon_fine_kpoints") },
            CellRec { tag: "PHONON_FINE_KPOINT_WEIGHTS", value: "phonon_fine_kpoint_weights", size: Size::KptF64, cond_key: Some("num_phonon_fine_kpoints") },
            CellRec { tag: "NUM_SCF_KPOINTS", value: "num_scf_kpoints", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "SCF_KPOINTS", value: "scf_kpoints", size: Size::KptF64x3, cond_key: Some("num_scf_kpoints") },
            CellRec { tag: "SCF_KPOINT_WEIGHTS", value: "scf_kpoint_weights", size: Size::KptF64, cond_key: Some("num_scf_kpoints") },
            CellRec { tag: "NUM_MAGRES_KPOINTS", value: "num_magres_kpoints", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "MAGRES_KPOINTS", value: "magres_kpoints", size: Size::KptF64x3, cond_key: Some("num_magres_kpoints") },
            CellRec { tag: "MAGRES_KPOINT_WEIGHTS", value: "magres_kpoint_weights", size: Size::KptF64, cond_key: Some("num_magres_kpoints") },
            CellRec { tag: "NUM_ELNES_KPOINTS", value: "num_elnes_kpoints", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "ELNES_KPOINTS", value: "elnes_kpoints", size: Size::KptF64x3, cond_key: Some("num_elnes_kpoints") },
            CellRec { tag: "ELNES_KPOINT_WEIGHTS", value: "elnes_kpoint_weights", size: Size::KptF64, cond_key: Some("num_elnes_kpoints") },
            CellRec { tag: "CELL_SYMMORPHIC", value: "cell_symmorphic", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "KPOINT_MP_GRID", value: "kpoint_mp_grid", size: Size::Fixed(12), cond_key: None },
            CellRec { tag: "KPOINT_MP_OFFSET", value: "kpoint_mp_offset", size: Size::Fixed(24), cond_key: None },
            CellRec { tag: "KPOINT_MP_DENSITY", value: "kpoint_mp_density", size: Size::Fixed(8), cond_key: None },
            CellRec { tag: "FIX_COM", value: "fix_com", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "FIX_ALL_IONS", value: "fix_all_ions", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "FIX_ALL_CELL", value: "fix_all_cell", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "SYMMETRY_GENERATE", value: "symmetry_generate", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "SYMMETRY_TOL", value: "symmetry_tol", size: Size::Fixed(8), cond_key: None },
            CellRec { tag: "NUM_OPTICS_KPOINTS", value: "num_optics_kpoints", size: Size::Fixed(4), cond_key: None },
            CellRec { tag: "OPTICS_KPOINTS", value: "optics_kpoints", size: Size::KptF64x3, cond_key: Some("num_optics_kpoints") },
            CellRec { tag: "OPTICS_KPOINT_WEIGHTS", value: "optics_kpoint_weights", size: Size::KptF64, cond_key: Some("num_optics_kpoints") },
            CellRec { tag: "OPTICS_KPOINT_MP_GRID", value: "optics_kpoint_mp_grid", size: Size::Fixed(12), cond_key: None },
            CellRec { tag: "OPTICS_KPOINT_MP_OFFSET", value: "optics_kpoint_mp_offset", size: Size::Fixed(24), cond_key: None },
            CellRec { tag: "OPTICS_KPOINT_MP_DENSITY", value: "optics_kpoint_mp_density", size: Size::Fixed(8), cond_key: None },
            CellRec { tag: "QUANTISATION_AXIS", value: "quantisation_axis", size: Size::Fixed(24), cond_key: None },
            CellRec { tag: "END_CELL_GLOBAL", value: "", size: Size::Fixed(0), cond_key: None },
];

/// One parsed cell section: the value payloads, keyed by tag, in file order.
#[derive(Debug, Default)]
pub struct RawCell {
    /// The value payloads. A bare tag record is not stored.
    pub values: Vec<(String, Vec<u8>)>,
}

impl RawCell {
    /// An empty cell section.
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    /// Finds a value payload by tag.
    pub fn get(&self, tag: &str) -> Option<&[u8]> {
        self.values
            .iter()
            .find(|(k, _)| k == tag)
            .map(|(_, v)| v.as_slice())
    }

    /// Reads a value as i32s.
    pub fn i32s(&self, tag: &str) -> CResult<Vec<i32>> {
        parse_i32s(self.get(tag).ok_or_else(|| CheckError::Section(tag.into()))?)
    }

    /// Reads a value as f64s.
    pub fn f64s(&self, tag: &str) -> CResult<Vec<f64>> {
        parse_f64s(self.get(tag).ok_or_else(|| CheckError::Section(tag.into()))?)
    }

    /// Reads a value as 4-byte logicals.
    pub fn logicals(&self, tag: &str) -> CResult<Vec<bool>> {
        parse_logical4s(self.get(tag).ok_or_else(|| CheckError::Section(tag.into()))?)
    }

    /// Reads a value as trimmed bytes.
    pub fn bytes(&self, tag: &str) -> Option<&[u8]> {
        self.get(tag).map(char_field)
    }

    /// Reads a value as lossy UTF-8.
    pub fn text(&self, tag: &str) -> Option<&str> {
        self.bytes(tag).map(char_str)
    }

    /// Replaces a value payload.
    pub fn set_payload(&mut self, tag: &str, payload: Vec<u8>) {
        if let Some(slot) = self.values.iter_mut().find(|(k, _)| k == tag) {
            slot.1 = payload;
        } else {
            self.values.push((tag.to_string(), payload));
        }
    }

    /// Replaces a char-field value. Pads the right with spaces.
    pub fn set_text(&mut self, tag: &str, value: &str, n: usize) -> CResult<()> {
        let bytes = value.as_bytes();
        if bytes.len() > n {
            return Err(CheckError::Section(format!(
                "{tag}: {n} bytes is too small for a {len}-byte value",
                len = bytes.len()
            )));
        }
        let mut payload = bytes.to_vec();
        payload.resize(n, b' ');
        self.set_payload(tag, payload);
        Ok(())
    }

    /// The counts, parsed from the stored value payloads.
    pub fn counts(&self) -> Counts {
        let i32 = |c: &RawCell, tag: &str| -> i32 {
            c.get(tag).and_then(|p| parse_i32s(p).ok()).and_then(|v| v.first().copied()).unwrap_or(0)
        };
        Counts {
            ns: i32(self, "CELL%NUM_SPECIES"),
            mi: i32(self, "CELL%MAX_IONS_IN_SPECIES"),
            nkpts: i32(self, "NKPTS"),
            nso: i32(self, "NUM_SYMMETRY_OPERATIONS"),
            nco: i32(self, "NUM_CRYSTAL_SYMMETRY_OPERATIONS"),
            nic: i32(self, "NUM_IONIC_CONSTRAINTS"),
            nk_bs: i32(self, "NUM_BS_KPOINTS"),
            nk_spectral: i32(self, "NUM_SPECTRAL_KPOINTS"),
            nk_phonon: i32(self, "NUM_PHONON_KPOINTS"),
            npgd: i32(self, "NUM_PHONON_GAMMA_DIRECTIONS"),
            nk_phonon_fine: i32(self, "NUM_PHONON_FINE_KPOINTS"),
            nk_scf: i32(self, "NUM_SCF_KPOINTS"),
            nk_magres: i32(self, "NUM_MAGRES_KPOINTS"),
            nk_elnes: i32(self, "NUM_ELNES_KPOINTS"),
            nk_optics: i32(self, "NUM_OPTICS_KPOINTS"),
        }
    }

}

impl Counts {
    /// Applies a freshly read value record to the counts, keyed by the
    /// 256-byte tag name instead of the value name.
    pub fn update_tag(&mut self, tag: &str, payload: &[u8]) {
        let v = match parse_i32s(payload) {
            Ok(v) => v.first().copied().unwrap_or(0),
            Err(_) => return,
        };
        let key = match tag {
            "CELL%NUM_SPECIES" => &mut self.ns,
            "CELL%MAX_IONS_IN_SPECIES" => &mut self.mi,
            "NKPTS" => &mut self.nkpts,
            "NUM_SYMMETRY_OPERATIONS" => &mut self.nso,
            "NUM_CRYSTAL_SYMMETRY_OPERATIONS" => &mut self.nco,
            "NUM_IONIC_CONSTRAINTS" => &mut self.nic,
            "NUM_BS_KPOINTS" => &mut self.nk_bs,
            "NUM_SPECTRAL_KPOINTS" => &mut self.nk_spectral,
            "NUM_PHONON_KPOINTS" => &mut self.nk_phonon,
            "NUM_PHONON_GAMMA_DIRECTIONS" => &mut self.npgd,
            "NUM_PHONON_FINE_KPOINTS" => &mut self.nk_phonon_fine,
            "NUM_SCF_KPOINTS" => &mut self.nk_scf,
            "NUM_MAGRES_KPOINTS" => &mut self.nk_magres,
            "NUM_ELNES_KPOINTS" => &mut self.nk_elnes,
            "NUM_OPTICS_KPOINTS" => &mut self.nk_optics,
            _ => return,
        };
        *key = v;
    }

    /// Looks up a count by the key a `CellRec` condition uses.
    pub fn get(&self, key: &str) -> i32 {
        match key {
            "num_symmetry_operations" => self.nso,
            "num_crystal_symmetry_operations" => self.nco,
            "num_ionic_constraints" => self.nic,
            "num_phonon_gamma_directions" => self.npgd,
            "num_bs_kpoints" => self.nk_bs,
            "num_spectral_kpoints" => self.nk_spectral,
            "num_phonon_kpoints" => self.nk_phonon,
            "num_phonon_fine_kpoints" => self.nk_phonon_fine,
            "num_scf_kpoints" => self.nk_scf,
            "num_magres_kpoints" => self.nk_magres,
            "num_elnes_kpoints" => self.nk_elnes,
            "num_optics_kpoints" => self.nk_optics,
            _ => 0,
        }
    }
}

/// The count value that gates a record.
fn gate_count(c: &Counts, cond_key: Option<&str>) -> i32 {
    cond_key.map(|k| c.get(k)).unwrap_or(0)
}

/// Reads one cell section (unit cell and/or global) from a record stream.
///
/// `table` is `UNIT_CELL` or `CELL_GLOBAL`. The reader keeps the value
/// payloads and the counts.
pub fn read_cell_section(
    reader: &mut RecordReader<&[u8]>,
    table: &[CellRec],
    base: &Counts,
) -> CResult<RawCell> {
    let mut raw = RawCell::new();
    let mut c = *base;
    for rec in table {
        // A conditional record pair is absent when its gate count is zero.
        // The decision is made before the tag record is consumed.
        let count = gate_count(&c, rec.cond_key);
        let present = rec.cond_key.is_none() || count > 0;
        if !present {
            continue;
        }
        let tag_payload = reader.read_fixed(TAG_SIZE)?;
        let tag = char_field(&tag_payload);
        if char_str(tag) != rec.tag {
            return Err(CheckError::Section(format!(
                "cell tag is {found}, expected {expected}",
                found = char_str(tag),
                expected = rec.tag
            )));
        }
        if rec.value.is_empty() {
            continue;
        }
        let size = rec.size.bytes(&c)?;
        let payload = reader.read_fixed(size)?;
        Counts::update(&mut c, rec.value, &payload);
        raw.set_payload(rec.tag, payload);
    }
    Ok(raw)
}

impl Counts {
    /// Applies a freshly read value record to the counts, if it is a count.
    pub fn update(&mut self, value: &str, payload: &[u8]) -> Option<i32> {
        let v = parse_i32s(payload).ok()?.first().copied()?;
        let key = match value {
            "num_species" => &mut self.ns,
            "max_ions_in_species" => &mut self.mi,
            "nkpts" => &mut self.nkpts,
            "num_symmetry_operations" => &mut self.nso,
            "num_crystal_symmetry_operations" => &mut self.nco,
            "num_ionic_constraints" => &mut self.nic,
            "num_bs_kpoints" => &mut self.nk_bs,
            "num_spectral_kpoints" => &mut self.nk_spectral,
            "num_phonon_kpoints" => &mut self.nk_phonon,
            "num_phonon_gamma_directions" => &mut self.npgd,
            "num_phonon_fine_kpoints" => &mut self.nk_phonon_fine,
            "num_scf_kpoints" => &mut self.nk_scf,
            "num_magres_kpoints" => &mut self.nk_magres,
            "num_elnes_kpoints" => &mut self.nk_elnes,
            "num_optics_kpoints" => &mut self.nk_optics,
            _ => return None,
        };
        *key = v;
        Some(v)
    }
}

/// Writes one cell section in the target layout.
///
/// Value payloads are replayed byte-exact. Conditional pairs are emitted
/// only when the stored counts make them present.
pub fn write_cell_section<W: std::io::Write>(
    writer: &mut RecordWriter<W>,
    raw: &RawCell,
    table: &[CellRec],
    base: &Counts,
) -> CResult<()> {
    let mut c = *base;
    for (tag, payload) in &raw.values {
        Counts::update_tag(&mut c, tag, payload);
    }
    for rec in table {
        let count = gate_count(&c, rec.cond_key);
        let present = rec.cond_key.is_none() || count > 0;
        if !present {
            continue;
        }
        writer.write_char_padded(rec.tag, TAG_SIZE)?;
        if rec.value.is_empty() {
            continue;
        }
        let payload = raw.get(rec.tag).ok_or_else(|| {
            CheckError::Section(format!("cell: missing value for tag {}", rec.tag))
        })?;
        writer.write_record(payload)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tables_have_unique_tags() {
        for table in [UNIT_CELL, CELL_GLOBAL] {
            use std::collections::HashSet;
            let tags: HashSet<&str> = table.iter().map(|r| r.tag).collect();
            assert_eq!(tags.len(), table.len());
        }
    }

    #[test]
    fn tables_begin_and_end_correctly() {
        assert_eq!(UNIT_CELL.first().unwrap().tag, "BEGIN_UNIT_CELL");
        assert_eq!(UNIT_CELL.last().unwrap().tag, "END_UNIT_CELL");
        assert_eq!(CELL_GLOBAL.first().unwrap().tag, "BEGIN_CELL_GLOBAL");
        assert_eq!(CELL_GLOBAL.last().unwrap().tag, "END_CELL_GLOBAL");
    }

    #[test]
    fn sizes_resolve_from_counts() {
        let c = Counts {
            ns: 3,
            mi: 16,
            nkpts: 1,
            nso: 2,
            nic: 3,
            npgd: 0,
            ..Default::default()
        };
        assert_eq!(Size::PerNsI32.bytes(&c).unwrap(), 12);
        assert_eq!(Size::PerNsF64.bytes(&c).unwrap(), 24);
        assert_eq!(Size::PerNsChar(8).bytes(&c).unwrap(), 24);
        assert_eq!(Size::PerMiNsI32.bytes(&c).unwrap(), 192);
        assert_eq!(Size::ThreeMiNsF64.bytes(&c).unwrap(), 1152);
        assert_eq!(Size::ThreeMiNsLogical.bytes(&c).unwrap(), 576);
        assert_eq!(Size::FourMiNsF64.bytes(&c).unwrap(), 1536);
        assert_eq!(Size::MiNsSqI32.bytes(&c).unwrap(), 9216);
        assert_eq!(Size::KptF64x3.bytes(&c).unwrap(), 24);
        assert_eq!(Size::KptF64.bytes(&c).unwrap(), 8);
        assert_eq!(Size::SymF64x9.bytes(&c).unwrap(), 144);
        assert_eq!(Size::SymMiNsI32.bytes(&c).unwrap(), 384);
        assert_eq!(Size::NicMiNsF64x3.bytes(&c).unwrap(), 3456);
        assert_eq!(Size::PgdF64x3.bytes(&c).unwrap(), 0);
    }

    #[test]
    fn size_resolution_rejects_negative_counts() {
        let c = Counts {
            ns: -1,
            ..Default::default()
        };
        assert!(Size::PerNsI32.bytes(&c).is_err());
    }

    #[test]
    fn counts_parse_from_payloads() {
        let mut raw = RawCell::new();
        raw.set_payload("CELL%NUM_SPECIES", 3u32.to_be_bytes().to_vec());
        raw.set_payload("CELL%MAX_IONS_IN_SPECIES", 16u32.to_be_bytes().to_vec());
        raw.set_payload("NKPTS", 1u32.to_be_bytes().to_vec());
        let c = raw.counts();
        assert_eq!(c.ns, 3);
        assert_eq!(c.mi, 16);
        assert_eq!(c.nkpts, 1);
        assert_eq!(c.nso, 0);
    }

    #[test]
    fn typed_accessors_work() {
        let mut raw = RawCell::new();
        raw.set_payload("CELL%VOLUME", 12.5f64.to_be_bytes().to_vec());
        assert_eq!(raw.f64s("CELL%VOLUME").unwrap(), vec![12.5]);
        raw.set_payload("CELL%ATOM_MOVE", vec![0, 0, 0, 1, 0, 0, 0, 1]);
        assert_eq!(raw.logicals("CELL%ATOM_MOVE").unwrap(), vec![true, true]);
        raw.set_text("CELL%SPECIES_SYMBOL", "Cu", 8).unwrap();
        assert_eq!(raw.text("CELL%SPECIES_SYMBOL"), Some("Cu"));
        assert!(raw.get("CELL%MISSING").is_none());
    }

    #[test]
    fn counts_update_maps_count_fields() {
        let mut c = Counts::default();
        assert_eq!(Counts::update(&mut c, "nkpts", 5u32.to_be_bytes().as_slice()), Some(5));
        assert_eq!(c.nkpts, 5);
        assert_eq!(Counts::update(&mut c, "volume", &[0u8; 8]), None);
        assert!(c.nkpts == 5);
    }
}

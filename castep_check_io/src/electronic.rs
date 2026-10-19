//! The electronic section of a CASTEP model file.
//!
//! After the two cell sections, a model file holds: the ground-state
//! flags, energies, the wavefunction block (`.check` files only), the
//! eigenvalue records, the charge density grid, and a run of optional
//! analysis sections, closed by the `END` record.
//!
//! All payloads are kept raw so that a roundtrip is byte-exact. The
//! optional sections are matched by their 10-byte headers. Known headers
//! have typed value readers; unknown ones are consumed record by record
//! up to the next known header.

use crate::cell::Counts;
use crate::error::{CResult, CheckError};
use crate::record::{parse_f64s, parse_i32s, RecordReader, RecordWriter};

/// A k-point block of the wavefunction section.
#[derive(Debug)]
pub struct WaveBlock {
    /// The 28-byte k-point record: three f64 coordinates and the
    /// `waves_at_kp` count.
    pub kpt: Vec<u8>,
    /// The plane-wave grid coordinate records. Three records.
    pub pw_grids: Vec<Vec<u8>>,
    /// The coefficient records, one per band.
    pub coeffs: Vec<Vec<u8>>,
}

/// The wavefunction section.
#[derive(Debug)]
pub struct WaveSection {
    /// The 4-byte kind tag: `wave` or `Gpt `.
    pub kind: [u8; 4],
    /// The grid dimensions `ngx, ngy, ngz`.
    pub grid: [i32; 3],
    /// The wavefunction dimensions:
    /// `size(coeffs,1), nbands_max, nkpts, nspins`.
    pub dims: [i32; 4],
    /// One block per `(spin, k-point)` pair.
    pub blocks: Vec<WaveBlock>,
}

/// One k-point of the eigenvalue records.
#[derive(Debug)]
pub struct EigenKpt {
    /// The 24-byte k-point coordinates record.
    pub kcoords: Vec<u8>,
    /// One `(occupancies, eigenvalues)` pair per spin.
    pub spins: Vec<(Vec<u8>, Vec<u8>)>,
}

/// An optional analysis section.
#[derive(Debug)]
pub struct OptionalSection {
    /// The 10-byte header, trimmed.
    pub header: String,
    /// The raw value records of the section, in file order.
    pub values: Vec<Vec<u8>>,
}

/// The parsed electronic section.
#[derive(Debug)]
pub struct RawElectronic {
    /// Whether a ground-state wavefunction was found.
    pub found_gs_wvfn: bool,
    /// Whether a ground-state density was found.
    pub found_gs_den: bool,
    /// The total energy, in Hartree.
    pub total_energy: f64,
    /// The first Fermi level.
    pub fermi: f64,
    /// The maximum band count.
    pub nbands_max: i32,
    /// The spin count.
    pub nspins: i32,
    /// The wavefunction block, present in `.check` files.
    pub wave: Option<WaveSection>,
    /// The eigenvalue records, one entry per k-point.
    pub eigen: Vec<EigenKpt>,
    /// The fine charge grid dimensions.
    pub fine_grid: [i32; 3],
    /// The raw density column records.
    pub density_columns: Vec<Vec<u8>>,
    /// The optional analysis sections, in file order.
    pub optional: Vec<OptionalSection>,
}

/// The known optional section headers.
pub const KNOWN_HEADERS: &[&str] = &[
    "E_FERMI",
    "OEP_POT",
    "DE_DLOGE",
    "FORCES",
    "STRESS",
    "MD",
    "PIMD",
    "MD_NOSE",
    "MD_NOSE_2",
    "BFGS",
    "LBFGS",
    "TPSD",
    "DAMPED_MD",
    "FIRE",
    "VAR_CELL",
    "PHONON",
    "PHON_GAMMA",
    "SECONDD",
    "FORCE_CON",
    "FINE_FREQ",
    "BORN_CHGS",
    "PART_CHGS",
    "DIELECTRIC",
    "RAMAN_TENS",
    "SHIELDING",
    "EFG",
    "GTENSOR",
    "HYPERFINE",
    "JCOUP",
    "JCOUP_FC",
    "JCOUP_SPIN",
    "JCOUP_ORB",
    "GENETIC",
    "LDAUOCCM",
];

/// The value record sizes of a typed header, in bytes.
///
/// `counts` supplies the cell counts that size-dependent headers need.
/// A header that is not typed here is consumed raw.
fn header_value_size(header: &str, counts: &Counts) -> Option<Vec<usize>> {
    match header {
        "E_FERMI" => Some(vec![8]),
        "FORCES" => Some(vec![3 * counts.mi as usize * counts.ns as usize * 8]),
        "STRESS" => Some(vec![48, 48]),
        "MD" => Some(vec![4, 8]),
        _ => None,
    }
}

/// Reads the electronic section.
///
/// `is_check` is true for `.check` files, which carry the wavefunction
/// block. `nkpts` is the k-point count from the cell section.
pub fn read_electronic(
    reader: &mut RecordReader<&[u8]>,
    is_check: bool,
    nkpts: i32,
    counts: &Counts,
) -> CResult<RawElectronic> {
    let found_wvfn = parse_i32s(&reader.read_fixed(4)?)?;
    let found_den = parse_i32s(&reader.read_fixed(4)?)?;
    let energy = parse_f64s(&reader.read_fixed(8)?)?;
    let fermi = parse_f64s(&reader.read_fixed(8)?)?;
    let dims2 = parse_i32s(&reader.read_fixed(8)?)?;

    let mut out = RawElectronic {
        found_gs_wvfn: found_wvfn.first().copied() == Some(1),
        found_gs_den: found_den.first().copied() == Some(1),
        total_energy: energy.first().copied().unwrap_or(0.0),
        fermi: fermi.first().copied().unwrap_or(0.0),
        nbands_max: dims2.first().copied().unwrap_or(0),
        nspins: dims2.get(1).copied().unwrap_or(1),
        wave: None,
        eigen: Vec::new(),
        fine_grid: [0; 3],
        density_columns: Vec::new(),
        optional: Vec::new(),
    };

    if is_check {
        out.wave = Some(read_wave(reader)?);
    }

    let nbands = out.nbands_max;
    let nspins = out.nspins;
    for _ in 0..nkpts.max(0) {
        let kcoords = reader.read_fixed(24)?;
        let mut spins = Vec::with_capacity(nspins.max(1) as usize);
        for _ in 0..nspins.max(1) {
            let occ = reader.read_fixed(nbands as usize * 8)?;
            let eig = reader.read_fixed(nbands as usize * 8)?;
            spins.push((occ, eig));
        }
        out.eigen.push(EigenKpt { kcoords, spins });
    }

    let found_den2 = parse_i32s(&reader.read_fixed(4)?)?;
    if found_den2.first().copied() != found_den.first().copied() {
        return Err(CheckError::Section(
            "the two ground-state density flags disagree".into(),
        ));
    }
    let grid = parse_i32s(&reader.read_fixed(12)?)?;
    out.fine_grid = [
        grid.first().copied().unwrap_or(0),
        grid.get(1).copied().unwrap_or(0),
        grid.get(2).copied().unwrap_or(0),
    ];
    if out.found_gs_den {
        let [ngx, ngy, ngz] = out.fine_grid;
        let per_col = 8 + 16 * ngz as usize * if nspins == 2 { 2 } else { 1 };
        for _ in 0..(ngx as usize) * (ngy as usize) {
            out.density_columns.push(reader.read_fixed(per_col)?);
        }
    }

    // Optional sections. A 10-byte record that trims to a known header
    // starts a new section. Any other 10-byte record is a value record.
    let mut current: Option<(String, Vec<Vec<u8>>)> = None;
    loop {
        let rec = reader.read_record()?;
        if rec.len() == 10 {
            let text = crate::record::char_str(crate::record::char_field(&rec));
            if text == "END" {
                break;
            }
            if KNOWN_HEADERS.contains(&text) {
                if let Some((h, values)) = current.take() {
                    out.optional.push(OptionalSection { header: h, values });
                }
                let mut values = Vec::new();
                if let Some(sizes) = header_value_size(text, counts) {
                    for size in sizes {
                        values.push(reader.read_fixed(size)?);
                    }
                }
                current = Some((text.to_string(), values));
                continue;
            }
            // An unknown 10-byte record starts a raw section.
            if current.is_none() {
                current = Some((text.to_string(), Vec::new()));
                continue;
            }
        }
        match &mut current {
            Some((_, values)) => values.push(rec),
            None => {
                return Err(CheckError::Section(
                    "a value record appears before any section header".into(),
                ))
            }
        }
    }
    if let Some((h, values)) = current {
        out.optional.push(OptionalSection { header: h, values });
    }

    Ok(out)
}

/// Reads the wavefunction block.
fn read_wave(reader: &mut RecordReader<&[u8]>) -> CResult<WaveSection> {
    let kind_rec = reader.read_fixed(4)?;
    let mut kind = [0u8; 4];
    kind.copy_from_slice(&kind_rec);
    let grid = parse_i32s(&reader.read_fixed(12)?)?;
    let dims = parse_i32s(&reader.read_fixed(16)?)?;
    let nbands_max = dims.get(1).copied().unwrap_or(0);
    let nkpts = dims.get(2).copied().unwrap_or(0);
    let nspins = dims.get(3).copied().unwrap_or(0);
    let mut blocks = Vec::with_capacity((nkpts * nspins).max(0) as usize);
    for _ in 0..(nkpts * nspins).max(0) {
        let kpt = reader.read_fixed(28)?;
        let waves = parse_i32s(&kpt[24..])?.first().copied().unwrap_or(0) as usize;
        let mut pw_grids = Vec::with_capacity(3);
        for _ in 0..3 {
            pw_grids.push(reader.read_fixed(4 * waves)?);
        }
        let mut coeffs = Vec::with_capacity(nbands_max as usize);
        for _ in 0..nbands_max {
            coeffs.push(reader.read_fixed(16 * waves)?);
        }
        blocks.push(WaveBlock {
            kpt,
            pw_grids,
            coeffs,
        });
    }
    Ok(WaveSection {
        kind,
        grid: [
            grid.first().copied().unwrap_or(0),
            grid.get(1).copied().unwrap_or(0),
            grid.get(2).copied().unwrap_or(0),
        ],
        dims: [
            dims.first().copied().unwrap_or(0),
            nbands_max,
            nkpts,
            nspins,
        ],
        blocks,
    })
}

/// Writes the electronic section in the target layout.
pub fn write_electronic<W: std::io::Write>(
    writer: &mut RecordWriter<W>,
    elec: &RawElectronic,
    is_check: bool,
) -> CResult<()> {
    writer.write_i32s(&[i32::from(elec.found_gs_wvfn)])?;
    writer.write_i32s(&[i32::from(elec.found_gs_den)])?;
    writer.write_f64s(&[elec.total_energy])?;
    writer.write_f64s(&[elec.fermi])?;
    writer.write_i32s(&[elec.nbands_max, elec.nspins])?;

    if is_check {
        write_wave(writer, elec.wave.as_ref().ok_or_else(|| {
            CheckError::Section("a .check file needs a wavefunction block".into())
        })?)?;
    }

    for kpt in &elec.eigen {
        writer.write_record(&kpt.kcoords)?;
        for (occ, eig) in &kpt.spins {
            writer.write_record(occ)?;
            writer.write_record(eig)?;
        }
    }

    writer.write_i32s(&[i32::from(elec.found_gs_den)])?;
    writer.write_i32s(&elec.fine_grid)?;
    for col in &elec.density_columns {
        writer.write_record(col)?;
    }

    for sec in &elec.optional {
        writer.write_char_padded(&sec.header, 10)?;
        for value in &sec.values {
            writer.write_record(value)?;
        }
    }
    writer.write_char_padded("END", 10)?;
    Ok(())
}

/// Writes the wavefunction block.
fn write_wave<W: std::io::Write>(writer: &mut RecordWriter<W>, wave: &WaveSection) -> CResult<()> {
    writer.write_record(&wave.kind)?;
    writer.write_i32s(&wave.grid)?;
    writer.write_i32s(&wave.dims)?;
    for block in &wave.blocks {
        writer.write_record(&block.kpt)?;
        for grid in &block.pw_grids {
            writer.write_record(grid)?;
        }
        for coeff in &block.coeffs {
            writer.write_record(coeff)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_header_sizes() {
        let c = Counts {
            ns: 3,
            mi: 10,
            ..Default::default()
        };
        assert_eq!(header_value_size("E_FERMI", &c).unwrap(), vec![8usize]);
        assert_eq!(header_value_size("FORCES", &c).unwrap(), vec![720usize]);
        assert_eq!(header_value_size("STRESS", &c).unwrap(), vec![48usize, 48]);
        assert_eq!(header_value_size("MD", &c).unwrap(), vec![4usize, 8]);
        assert!(header_value_size("MD_NOSE", &c).is_none());
    }

    #[test]
    fn optional_section_keeps_raw_values() {
        let sec = OptionalSection {
            header: "E_FERMI".into(),
            values: vec![vec![1u8; 8]],
        };
        assert_eq!(sec.header, "E_FERMI");
        assert_eq!(sec.values.len(), 1);
    }
}

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
        "MD" => Some(vec![4, 8]),
        // STRESS is consumed raw: some 6.11 builds write the strain
        // tensor as a 72-byte record (9 f64) instead of 48.
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
        assert_eq!(header_value_size("STRESS", &c), None); // raw
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


    fn i32_be(values: &[i32]) -> Vec<u8> {
        let mut out = Vec::with_capacity(values.len() * 4);
        for v in values {
            out.extend_from_slice(&v.to_be_bytes());
        }
        out
    }

    fn f64_be(values: &[f64]) -> Vec<u8> {
        let mut out = Vec::with_capacity(values.len() * 8);
        for v in values {
            out.extend_from_slice(&v.to_be_bytes());
        }
        out
    }

    /// One density column record: (nx, ny) header plus the charge values.
    fn column_record(
        nx: i32,
        ny: i32,
        up: &[f64],
        down: Option<&[f64]>,
    ) -> Vec<u8> {
        let mut rec = i32_be(&[nx, ny]);
        rec.extend_from_slice(&f64_be(up));
        if let Some(d) = down {
            rec.extend_from_slice(&f64_be(d));
        }
        rec
    }

    fn raw_electronic(nspins: i32, fine_grid: [i32; 3], columns: Vec<Vec<u8>>) -> RawElectronic {
        RawElectronic {
            found_gs_wvfn: false,
            found_gs_den: true,
            total_energy: 0.0,
            fermi: 0.0,
            nbands_max: 0,
            nspins,
            wave: None,
            eigen: vec![],
            fine_grid,
            density_columns: columns,
            optional: vec![],
        }
    }

    #[test]
    fn charge_decodes_fine_grid_c_order() {
        // grid (2, 1, 2), one spin: 1-based headers, each record holds
        // the z-stack of one (x, y) column. Output is C order for
        // shape (2, 1, 2): index (x * 1 + y) * 2 + z.
        let cols = vec![
            column_record(1, 1, &[1.0, 9.0, 3.0, 9.1], None),
            column_record(2, 1, &[4.0, 9.2, 5.0, 9.3], None),
        ];
        let out = raw_electronic(1, [2, 1, 2], cols).charge().unwrap();
        assert_eq!(out, vec![1.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn charge_sums_spin_columns() {
        let cols = vec![column_record(
            1,
            1,
            &[1.0, 0.0, 3.0, 0.0],
            Some(&[10.0, 0.0, 30.0, 0.0]),
        )];
        let out = raw_electronic(2, [1, 1, 2], cols).charge().unwrap();
        assert_eq!(out, vec![11.0, 33.0]);
    }

    #[test]
    fn charge_rejects_bad_column_count() {
        let cols = vec![column_record(1, 1, &[1.0, 0.0], None)];
        let e = raw_electronic(1, [2, 1, 1], cols).charge();
        assert!(e.is_err());
    }

    #[test]
    fn charge_rejects_out_of_range_index() {
        let cols = vec![column_record(3, 1, &[1.0, 0.0], None)];
        let e = raw_electronic(1, [2, 1, 1], cols).charge();
        assert!(e.is_err());
    }
}

// ---------------------------------------------------------------------------
// Fine-grid charge decoding
// ---------------------------------------------------------------------------

/// Decodes one big-endian f64 from `bytes` at `offset`.
fn be_f64_at(bytes: &[u8], offset: usize) -> f64 {
    f64::from_be_bytes(bytes[offset..offset + 8].try_into().unwrap())
}

impl RawElectronic {
    /// Total charge on the fine grid, C order for shape
    /// `(ngx, ngy, ngz)`: `x` slowest, `z` fastest,
    /// `ngx * ngy * ngz` values.
    ///
    /// Each `density_columns` record carries a big-endian `(nx, ny)`
    /// header (1-based, so `1 <= nx <= ngx`) and `ngz` complex charge
    /// values (real, imag pairs, big-endian). For `nspins == 2` a
    /// second `ngz` column of down-spin charge follows; the two are
    /// summed into the total charge.
    pub fn charge(&self) -> CResult<Vec<f64>> {
        let ngx = self.fine_grid[0].max(0) as usize;
        let ngy = self.fine_grid[1].max(0) as usize;
        let ngz = self.fine_grid[2].max(0) as usize;
        if self.density_columns.len() != ngx * ngy {
            return Err(CheckError::Section(format!(
                "expected {} density column records, found {}",
                ngx * ngy,
                self.density_columns.len()
            )));
        }
        let per = 8 + 16 * ngz;
        let mut out = vec![0.0f64; ngx * ngy * ngz];
        for rec in &self.density_columns {
            if rec.len() < per + if self.nspins == 2 { 16 * ngz } else { 0 } {
                return Err(CheckError::Section("short density column record".into()));
            }
            // 1-based headers: nx in 1..=ngx, ny in 1..=ngy.
            let nx = parse_i32s(&rec[0..4])?[0];
            let ny = parse_i32s(&rec[4..8])?[0];
            if nx < 1 || nx as usize > ngx || ny < 1 || ny as usize > ngy {
                return Err(CheckError::Section(format!(
                    "density column index out of range: nx={nx}, ny={ny} (grid {ngx}x{ngy})"
                )));
            }
            // 1-based headers: nx in 1..=ngx, ny in 1..=ngy.
            let nx = parse_i32s(&rec[0..4])?[0];
            let ny = parse_i32s(&rec[4..8])?[0];
            if nx < 1 || nx as usize > ngx || ny < 1 || ny as usize > ngy {
                return Err(CheckError::Section(format!(
                    "density column index out of range: nx={nx}, ny={ny} (grid {ngx}x{ngy})"
                )));
            }
            let (cx, cy) = (nx as usize - 1, ny as usize - 1);
            for z in 0..ngz {
                let mut v = be_f64_at(rec, 8 + 16 * z);
                if self.nspins == 2 {
                    v += be_f64_at(rec, 8 + 16 * ngz + 16 * z);
                }
                // C order for shape (ngx, ngy, ngz): x slowest, z fastest.
                out[(cx * ngy + cy) * ngz + z] = v;
            }
        }
        Ok(out)
    }
}
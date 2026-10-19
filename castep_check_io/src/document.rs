//! The top-level model file document.
//!
//! A `CheckDocument` is one parsed CASTEP model file: the parameters
//! section, the two cell sections, and the electronic section. The file
//! kind (`.check` or `.castep_bin`) is detected from the first record.
//! Writing always emits the target layout of the enabled release.

use crate::cell::{read_cell_section, write_cell_section, CELL_GLOBAL, RawCell, UNIT_CELL};
use crate::electronic::{read_electronic, write_electronic, RawElectronic};
use crate::error::{CResult, CheckError};
use crate::params::{read_params, write_params, RawParams};
use crate::record::{RecordReader, RecordWriter};
use crate::version::CheckKind;

/// The parsed contents of one model file.
#[derive(Debug)]
pub struct CheckDocument {
    /// Whether the file kind is `.check` or `.castep_bin`.
    pub kind: CheckKind,
    /// The parameters section.
    pub params: RawParams,
    /// The live cell section.
    pub cell: RawCell,
    /// The live cell global section.
    pub cell_global: RawCell,
    /// The original cell section.
    pub orig_cell: RawCell,
    /// The original cell global section.
    pub orig_cell_global: RawCell,
    /// The electronic section.
    pub electronic: RawElectronic,
}

impl CheckDocument {
    /// Parses a model file from bytes.
    ///
    /// The kind is detected from the first record: a 10-byte
    /// `CASTEP_BIN` magic means `.castep_bin`, otherwise `.check`.
    pub fn from_bytes(data: &[u8]) -> CResult<Self> {
        let mut reader = RecordReader::new(data);
        let first = reader.read_record()?;
        let kind = if first.len() == 10 && crate::record::char_str(crate::record::char_field(&first)) == "CASTEP_BIN" {
            CheckKind::CastepBin
        } else {
            CheckKind::Check
        };
        // The first record is the parameters tag. Re-read from the start.
        Self::from_recorded(data, kind)
    }

    fn from_recorded(data: &[u8], kind: CheckKind) -> CResult<Self> {
        let mut reader = RecordReader::new(data);
        let is_bin = kind == CheckKind::CastepBin;
        let params = read_params(&mut reader, is_bin)?;
        let zero = crate::cell::Counts::default();
        let cell = read_cell_section(&mut reader, UNIT_CELL, &zero)?;
        let cell_counts = cell.counts();
        let cell_global = read_cell_section(&mut reader, CELL_GLOBAL, &cell_counts)?;
        let orig_cell = read_cell_section(&mut reader, UNIT_CELL, &zero)?;
        let orig_cell_counts = orig_cell.counts();
        let orig_cell_global =
            read_cell_section(&mut reader, CELL_GLOBAL, &orig_cell_counts)?;
        let counts = cell_counts;
        let nkpts = cell_global.counts().nkpts;
        let electronic = read_electronic(&mut reader, !is_bin, nkpts, &counts)?;
        Ok(Self {
            kind,
            params,
            cell,
            cell_global,
            orig_cell,
            orig_cell_global,
            electronic,
        })
    }

    /// Parses a `.check` file from disk.
    pub fn from_check_file(path: impl AsRef<std::path::Path>) -> CResult<Self> {
        let data = std::fs::read(path.as_ref())
            .map_err(|e| CheckError::Io { path: path.as_ref().to_path_buf(), source: e })?;
        let doc = Self::from_bytes(&data)?;
        if doc.kind != CheckKind::Check {
            return Err(CheckError::Section(
                "expected a .check file, found a .castep_bin file".into(),
            ));
        }
        Ok(doc)
    }

    /// Parses a `.castep_bin` file from disk.
    pub fn from_castep_bin_file(path: impl AsRef<std::path::Path>) -> CResult<Self> {
        let data = std::fs::read(path.as_ref())
            .map_err(|e| CheckError::Io { path: path.as_ref().to_path_buf(), source: e })?;
        let doc = Self::from_bytes(&data)?;
        if doc.kind != CheckKind::CastepBin {
            return Err(CheckError::Section(
                "expected a .castep_bin file, found a .check file".into(),
            ));
        }
        Ok(doc)
    }

    /// Serializes the document as a `.check` file.
    pub fn to_check_bytes(&self) -> CResult<Vec<u8>> {
        let mut out = Vec::new();
        let mut writer = RecordWriter::new(&mut out);
        self.write_into(&mut writer, CheckKind::Check)?;
        Ok(out)
    }

    /// Serializes the document as a `.castep_bin` file.
    pub fn to_castep_bin_bytes(&self) -> CResult<Vec<u8>> {
        let mut out = Vec::new();
        let mut writer = RecordWriter::new(&mut out);
        self.write_into(&mut writer, CheckKind::CastepBin)?;
        Ok(out)
    }

    /// Writes the document to a `.check` file on disk.
    pub fn to_check_file(&self, path: impl AsRef<std::path::Path>) -> CResult<()> {
        let bytes = self.to_check_bytes()?;
        std::fs::write(path.as_ref(), bytes)
            .map_err(|e| CheckError::Io { path: path.as_ref().to_path_buf(), source: e })?;
        Ok(())
    }

    /// Writes the document to a `.castep_bin` file on disk.
    pub fn to_castep_bin_file(&self, path: impl AsRef<std::path::Path>) -> CResult<()> {
        let bytes = self.to_castep_bin_bytes()?;
        std::fs::write(path.as_ref(), bytes)
            .map_err(|e| CheckError::Io { path: path.as_ref().to_path_buf(), source: e })?;
        Ok(())
    }

    fn write_into<W: std::io::Write>(
        &self,
        writer: &mut RecordWriter<W>,
        kind: CheckKind,
    ) -> CResult<()> {
        let is_bin = kind == CheckKind::CastepBin;
        write_params(writer, &self.params, is_bin)?;
        let zero = crate::cell::Counts::default();
        let cell_counts = self.cell.counts();
        write_cell_section(writer, &self.cell, UNIT_CELL, &zero)?;
        write_cell_section(
            writer,
            &self.cell_global,
            CELL_GLOBAL,
            &cell_counts,
        )?;
        let orig_counts = self.orig_cell.counts();
        write_cell_section(writer, &self.orig_cell, UNIT_CELL, &zero)?;
        write_cell_section(
            writer,
            &self.orig_cell_global,
            CELL_GLOBAL,
            &orig_counts,
        )?;
        write_electronic(writer, &self.electronic, !is_bin)?;
        Ok(())
    }
}

impl Default for CheckDocument {
    fn default() -> Self {
        Self {
            kind: CheckKind::Check,
            params: RawParams {
                version: crate::version::target_file_version(),
                payloads: vec![],
            },
            cell: RawCell::new(),
            cell_global: RawCell::new(),
            orig_cell: RawCell::new(),
            orig_cell_global: RawCell::new(),
            electronic: RawElectronic {
                found_gs_wvfn: false,
                found_gs_den: false,
                total_energy: 0.0,
                fermi: 0.0,
                nbands_max: 0,
                nspins: 1,
                wave: None,
                eigen: Vec::new(),
                fine_grid: [0; 3],
                density_columns: Vec::new(),
                optional: Vec::new(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_document_has_target_version() {
        let doc = CheckDocument::default();
        assert_eq!(doc.params.version, crate::version::target_file_version());
        assert_eq!(doc.kind, CheckKind::Check);
        assert!(doc.electronic.wave.is_none());
    }
}

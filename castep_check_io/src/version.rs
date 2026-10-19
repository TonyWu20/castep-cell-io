//! Version handling for model files. See ADR-0005.
//!
//! The writer version is stored in the second record of the parameters
//! section: a `char(10)` written `f6.3` from the writer's version number,
//! e.g. ` 6.110` for CASTEP 6.11.
//!
//! Read policy: accept a file at or below the build target. Write policy:
//! always emit the target release layout and the target version string.

use crate::error::{CResult, CheckError, FileVersion};

/// The lowest file version the reader knows the layout of.
pub const MIN_SUPPORTED: FileVersion = FileVersion::new(3, 0);

/// The target file version for this build.
///
/// Mapped from the `CastepVersion` code (`major * 100 + minor`).
pub fn target_file_version() -> FileVersion {
    let code = castep_cell_fmt::CastepVersion::target().code();
    FileVersion::new((code / 100) as u8, (code % 100) as u8)
}

/// The target version record, 10 bytes, `f6.3` style, space-padded.
pub fn target_version_record() -> [u8; 10] {
    let t = target_file_version();
    let frac = format!("{:0<3}", t.minor.to_string().chars().take(3).collect::<String>());
    // The Fortran field is f6.3, a six-char right-justified field.
    let field = format!("{:>6}", format!("{}.{}", t.major, frac));
    // The f6.3 field sits at the left of the char(10) variable; the right
    // is space-padded.
    let mut out = vec![b' '; 10];
    out[..field.len()].copy_from_slice(field.as_bytes());
    out.try_into().unwrap()
}

/// Parses the 10-byte version record into a file version.
///
/// ` 6.110` gives 6.11, ` 4.100` gives 4.1, ` 3.000` gives 3.0. The
/// fractional part is read as up to three digits with trailing zeros
/// dropped: `110` is 11, `100` is 1, `000` is 0.
pub fn parse_version_record(payload: &[u8]) -> CResult<FileVersion> {
    if payload.len() != 10 {
        return Err(CheckError::BadRecordSize {
            offset: 0,
            expected: 10,
            actual: payload.len(),
        });
    }
    use crate::record::char_field;
    let raw = char_field(payload);
    let text = std::str::from_utf8(raw).map_err(|_| CheckError::MalformedVersion(payload.to_vec()))?;
    let text = text.trim();
    let (major_s, frac_s) = text
        .split_once('.')
        .ok_or_else(|| CheckError::MalformedVersion(payload.to_vec()))?;
    let major: u8 = major_s.parse().map_err(|_| CheckError::MalformedVersion(payload.to_vec()))?;
    let frac = frac_s.parse::<u32>().map_err(|_| CheckError::MalformedVersion(payload.to_vec()))?;
    let minor = strip_trailing_zeros(frac) as u8;
    Ok(FileVersion::new(major, minor))
}

fn strip_trailing_zeros(mut v: u32) -> u32 {
    while v > 0 && v.is_multiple_of(10) {
        v /= 10;
    }
    v
}

/// Checks a parsed file version against the build target.
///
/// Returns an error when the file is newer than the target or older than
/// the supported floor.
pub fn check_file_version(file: FileVersion) -> CResult<()> {
    let target = target_file_version();
    if file > target {
        return Err(CheckError::VersionTooNew { file, target });
    }
    if file < MIN_SUPPORTED {
        return Err(CheckError::VersionTooOld(file));
    }
    Ok(())
}

/// Whether a version-gated record, added at `gate`, exists in a file of
/// version `file`. CASTEP gates read records on `version >= gate`.
pub fn gate_open(file: FileVersion, gate: FileVersion) -> bool {
    file >= gate
}

/// The gate levels present in the 6.11 parameters layout.
pub mod gate {
    use super::FileVersion;

    pub const V3_0: FileVersion = FileVersion::new(3, 0);
    pub const V3_1: FileVersion = FileVersion::new(3, 1);
    pub const V3_2: FileVersion = FileVersion::new(3, 2);
    pub const V4_0: FileVersion = FileVersion::new(4, 0);
    pub const V4_1: FileVersion = FileVersion::new(4, 1);
    pub const V4_2: FileVersion = FileVersion::new(4, 2);
    pub const V4_3: FileVersion = FileVersion::new(4, 3);
    pub const V4_4: FileVersion = FileVersion::new(4, 4);
    pub const V5_0: FileVersion = FileVersion::new(5, 0);
    pub const V5_5: FileVersion = FileVersion::new(5, 5);
    pub const V6_0: FileVersion = FileVersion::new(6, 0);
    pub const V6_1: FileVersion = FileVersion::new(6, 1);
}

// A build must target at least one CASTEP release. The fmt dependency
// carries its own guard; this one names this crate.
#[cfg(not(any(feature = "castep-6-11", feature = "castep-23")))]
compile_error!(
    "castep-check-io targets no CASTEP release. \
     Enable at least one release feature, e.g. `castep-6-11` or `castep-23`."
);

#[cfg(test)]
mod tests {
    use super::*;

    fn padded(body: &str) -> [u8; 10] {
        let mut out = vec![b' '; 10];
        for (i, b) in body.bytes().enumerate() {
            out[i] = b;
        }
        out.try_into().unwrap()
    }

    #[test]
    fn parses_six_eleven() {
        let v = parse_version_record(&padded(" 6.110")).unwrap();
        assert_eq!(v, FileVersion::new(6, 11));
    }

    #[test]
    fn parses_four_point_one() {
        let v = parse_version_record(&padded(" 4.100")).unwrap();
        assert_eq!(v, FileVersion::new(4, 1));
    }

    #[test]
    fn parses_five_point_five() {
        let v = parse_version_record(&padded(" 5.500")).unwrap();
        assert_eq!(v, FileVersion::new(5, 5));
    }

    #[test]
    fn parses_three_point_zero() {
        let v = parse_version_record(&padded(" 3.000")).unwrap();
        assert_eq!(v, FileVersion::new(3, 0));
    }

    #[test]
    fn rejects_malformed_records() {
        assert!(matches!(
            parse_version_record(&padded("garbage")),
            Err(CheckError::MalformedVersion(_))
        ));
        assert!(matches!(
            parse_version_record(&padded("")),
            Err(CheckError::MalformedVersion(_))
        ));
        let short = [0u8; 8];
        assert!(parse_version_record(&short).is_err());
    }

    #[test]
    fn default_target_is_six_eleven() {
        let t = target_file_version();
        #[cfg(feature = "castep-6-11")]
        {
            assert_eq!(t, FileVersion::new(6, 11));
        }
    }

    #[test]
    fn target_version_record_matches_f6_3() {
        let rec = target_version_record();
        use crate::record::char_str;
        #[cfg(all(feature = "castep-6-11", not(feature = "castep-23")))]
        assert_eq!(char_str(&rec), " 6.110");
    }

    #[test]
    fn rejects_newer_files() {
        let err = check_file_version(FileVersion::new(6, 12)).unwrap_err();
        #[cfg(feature = "castep-6-11")]
        assert!(matches!(err, CheckError::VersionTooNew { .. }));
    }

    #[test]
    fn rejects_older_files() {
        let err = check_file_version(FileVersion::new(2, 4)).unwrap_err();
        assert!(matches!(err, CheckError::VersionTooOld(_)));
    }

    #[test]
    fn accepts_at_and_below_target() {
        check_file_version(FileVersion::new(3, 0)).unwrap();
        check_file_version(FileVersion::new(6, 0)).unwrap();
        check_file_version(FileVersion::new(6, 1)).unwrap();
    }

    #[test]
    fn gate_open_orders_lexicographically() {
        assert!(gate_open(FileVersion::new(6, 11), gate::V6_1));
        assert!(gate_open(FileVersion::new(4, 1), gate::V4_1));
        assert!(!gate_open(FileVersion::new(4, 0), gate::V4_1));
        assert!(gate_open(FileVersion::new(6, 9), gate::V5_5));
        assert!(gate_open(FileVersion::new(6, 9), gate::V4_4));
    }
}

/// The kind of a model file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckKind {
    /// A full model file, with the wavefunction section.
    Check,
    /// The analysis file: the same sections, without wavefunctions.
    CastepBin,
}

impl CheckKind {
    /// The filename extension of the kind.
    pub fn extension(self) -> &'static str {
        match self {
            CheckKind::Check => ".check",
            CheckKind::CastepBin => ".castep_bin",
        }
    }
}

//! The CASTEP release this build targets.
//!
//! Supported releases are declared as cargo features in `Cargo.toml`
//! (one feature per release, e.g. `castep-6-11`). A build may enable
//! several; it then accepts the union of their keyword sets and targets
//! the highest enabled release.

/// Releases enabled in this build, ascending.
///
/// One definition per enabled-release combination. Each lists only the
/// variants that exist for that combination, so the slice is always valid.
#[cfg(all(feature = "castep-6-11", feature = "castep-23"))]
static SUPPORTED: &[CastepVersion] = &[CastepVersion::V6_11, CastepVersion::V23];
#[cfg(all(feature = "castep-6-11", not(feature = "castep-23")))]
static SUPPORTED: &[CastepVersion] = &[CastepVersion::V6_11];
#[cfg(all(not(feature = "castep-6-11"), feature = "castep-23"))]
static SUPPORTED: &[CastepVersion] = &[CastepVersion::V23];

/// A CASTEP release the library can target.
///
/// One variant per enabled release feature. The discriminant encodes the
/// release as `major * 100 + minor`.
///
/// The type and its impls only exist when at least one release feature is
/// enabled; a build that enables none fails with the `compile_error!`
/// guard below, which stays the only diagnostic in that case.
#[cfg(any(feature = "castep-6-11", feature = "castep-23"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum CastepVersion {
    /// CASTEP 6.11. Keyword names, defaults, and validation as released in
    /// the 6.11 source distribution.
    #[cfg(feature = "castep-6-11")]
    V6_11 = 611,
    /// Placeholder for CASTEP releases after 6.11.
    ///
    /// Gates keyword types that exist in the crate for a not-yet-released
    /// release (feature `castep-23`). Encoded as a major `23` so it always
    /// sorts above any current 6.x release.
    #[cfg(feature = "castep-23")]
    V23 = 2300,
}

#[cfg(any(feature = "castep-6-11", feature = "castep-23"))]
impl CastepVersion {
    /// Every release enabled in this build, ascending.
    pub const fn supported() -> &'static [Self] {
        SUPPORTED
    }

    /// The release this build targets: the highest enabled release.
    ///
    /// Parsing accepts every keyword of every enabled release. Output is
    /// emitted in the target release's spelling.
    pub fn target() -> Self {
        Self::supported()
            .iter()
            .max()
            .copied()
            .expect("at least one CASTEP release feature must be enabled")
    }

    /// Whether this release is enabled in this build.
    pub fn is_supported(self) -> bool {
        Self::supported().contains(&self)
    }

    /// The release label, e.g. `"6.11"`.
    pub const fn label(self) -> &'static str {
        match self {
            #[cfg(feature = "castep-6-11")]
            Self::V6_11 => "6.11",
            #[cfg(feature = "castep-23")]
            Self::V23 => "23",
        }
    }

    /// The encoded release code (`major * 100 + minor`).
    pub const fn code(self) -> u16 {
        self as u16
    }
}

#[cfg(any(feature = "castep-6-11", feature = "castep-23"))]
impl Default for CastepVersion {
    /// The target release: the highest enabled release.
    fn default() -> Self {
        Self::target()
    }
}

#[cfg(any(feature = "castep-6-11", feature = "castep-23"))]
impl std::fmt::Display for CastepVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

// A build must target at least one CASTEP release.
#[cfg(not(any(feature = "castep-6-11", feature = "castep-23")))]
compile_error!(
    "castep-cell-fmt targets no CASTEP release. \
     Enable at least one release feature, e.g. `castep-6-11` or `castep-23`."
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_is_the_highest_supported_release() {
        let supported = CastepVersion::supported();
        assert!(!supported.is_empty());
        assert_eq!(CastepVersion::target(), *supported.iter().max().unwrap());
        assert_eq!(CastepVersion::default(), CastepVersion::target());
    }

    #[test]
    fn label_and_code() {
        for v in CastepVersion::supported() {
            assert!(!v.label().is_empty());
            assert!(v.code() > 0);
        }
    }

    #[cfg(feature = "castep-6-11")]
    #[test]
    fn v6_11() {
        assert_eq!(CastepVersion::V6_11.label(), "6.11");
        assert_eq!(CastepVersion::V6_11.code(), 611);
        assert!(CastepVersion::V6_11.is_supported());
        // Only when `castep-23` is off is 6.11 the target.
        #[cfg(not(feature = "castep-23"))]
        assert_eq!(CastepVersion::target(), CastepVersion::V6_11);
    }

    #[cfg(feature = "castep-23")]
    #[test]
    fn v23() {
        assert_eq!(CastepVersion::V23.label(), "23");
        assert_eq!(CastepVersion::V23.code(), 2300);
        assert!(CastepVersion::V23.is_supported());
        // `V23` encodes a major `23`, so it always sorts above `V6_11`.
        #[cfg(feature = "castep-6-11")]
        assert!(CastepVersion::V23 > CastepVersion::V6_11);
    }
}

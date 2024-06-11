use crate::InvLengthUnit;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
/// For backwards compatibility the keywords beginning
/// `BS_` and `OPTICS_` are synonyms for `SPECTRAL_KPOINT_` and similarly those beginning.
pub enum KPointKeywords {
    KPOINT_LIST,
    KPOINT_MP_GRID,
    KPOINT_MP_SPACING,
    KPOINT_MP_OFFSET,
    SPECTRAL_KPOINT_LIST,
    SPECTRAL_KPOINT_PATH,
    SPECTRAL_KPOINT_PATH_SPACING,
}

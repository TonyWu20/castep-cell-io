#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum PositionsKeywords {
    POSITIONS_FRAC,
    POSITIONS_ABS,
    #[cfg(feature = "TS")]
    POSITIONS_FRAC_INTERMEDIATE,
    #[cfg(feature = "TS")]
    POSITIONS_FRAC_PRODUCT,
    #[cfg(feature = "TS")]
    POSITIONS_ABS_INTERMEDIATE,
    #[cfg(feature = "TS")]
    POSITIONS_ABS_PRODUCT,
}

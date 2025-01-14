use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplay)]
#[keyword_display(specified_fields = true)]
pub enum BSExtraBands {
    #[keyword_display(field = "BS_NEXTRA_BANDS")]
    /// This keyword controls the number of extra bands at each k-point in addition to the number of occupied bands, when performing a band structure calculation.
    Nextra(u32),
    #[keyword_display(field = "BS_PERC_EXTRA_BANDS")]
    /// This keyword controls the number of extra bands at each k-point in addition to the number of occupied bands, when performing a band structure calculation.
    PercExtra(f64),
}

impl Default for BSExtraBands {
    fn default() -> Self {
        Self::PercExtra(0.0)
    }
}

#[cfg(test)]
mod test {
    use crate::param::KeywordDisplay;

    use super::BSExtraBands;

    #[test]
    fn bs_extra_bands() {
        let nextra = BSExtraBands::Nextra(64);
        assert_eq!(nextra.output(), "BS_NEXTRA_BANDS : 64");
        let perc_extra = BSExtraBands::PercExtra(72.0);
        assert_eq!(perc_extra.output(), "BS_PERC_EXTRA_BANDS : 72");
    }
}

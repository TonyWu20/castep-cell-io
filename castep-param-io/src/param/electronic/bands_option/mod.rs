use castep_param_derive::ParamDisplay;
use derive_builder::Builder;
pub use extra_bands::ExtraBands;
pub use nbands::Nbands;
use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

mod extra_bands;
/// This keyword determines the maximum number of bands at any k-point and spin.
/// There are three ways in which you can specify the maximum number of bands at any k-point and spin:
/// Directly, using `NBANDS`.
/// Indirectly, by specifying the number of extra bands in addition to the number of occupied bands using `NEXTRA_BANDS`.
/// This is the method used in the CASTEP interface.
/// Indirectly, by specifying the number of extra bands in addition to the number of occupied bands as a percentage of the latter value using `PERC_EXTRA_BANDS`.
/// It is not possible to have both the `NBANDS` keyword and either the `NEXTRA_BANDS` or `PERC_EXTRA_BANDS` keywords present in the same input file.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
    Builder,
    Default,
    ParamDisplay,
)]
#[builder(setter(into, strip_option), default)]
pub struct BandsOption {
    nbands: Option<Nbands>,
    extra_bands: Option<ExtraBands>,
}

mod nbands {

    use castep_param_derive::KeywordDisplay;
    use serde::{Deserialize, Serialize};

    use crate::param::{electronic::spin_polarised::SpinPolarised, ElectronicParam};

    use super::extra_bands::ExtraBands;

    #[derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        Serialize,
        Deserialize,
        KeywordDisplay,
    )]
    #[keyword_display(field = "NBANDS", from = u32, value=u32)]
    pub struct Nbands(u32);

    impl Nbands {
        /// If NEXTRA_BANDS is specified and SPIN_POLARIZED : FALSE:
        /// NBANDS : (NELECTRONS/2) + NEXTRA_BANDS
        /// Or, if SPIN_POLARIZED : TRUE:
        ///     NBANDS : max(NUP, NDOWN) + NEXTRA_BANDS.
        /// If PERC_EXTRA_BANDS is specified and SPIN_POLARIZED : FALSE:
        ///     NBANDS : (NELECTRONS/2) × [ 1 + (PERC_EXTRA_BANDS/100)]
        /// Or, if SPIN_POLARIZED : TRUE:
        ///     NBANDS : max(NUP, NDOWN) × [ 1 + (PERC_EXTRA_BANDS/100)]
        /// If NBANDS, NEXTRA_BANDS and PERC_EXTRA_BANDS are not specified and FIX_OCCUPANCY : TRUE, then, if SPIN_POLARIZED : FALSE:
        ///     NBANDS : NELECTRONS/2.
        /// Or, if SPIN_POLARIZED : TRUE:
        ///     NBANDS : max(NUP, NDOWN)
        /// If FIX_OCCUPANCY : FALSE, these default values of NBANDS will be multiplied by 1.2.
        pub fn default_value(
            electronic_param: ElectronicParam,
            spin_polarised: SpinPolarised,
            extra_bands: Option<ExtraBands>,
            // fix_occupancy: FixOccupancy,
        ) -> Self {
            if let (Some(ExtraBands::NextraBands(nextra)), SpinPolarised::False) =
                (extra_bands, spin_polarised)
            {
                let nelectrons = electronic_param
                    .nelectrons
                    .expect("Number of electrons of the system is not determined!");
                return Self(nelectrons.value() as u32 / 2 + nextra);
            }
            if let (Some(ExtraBands::NextraBands(nextra)), SpinPolarised::True) =
                (extra_bands, spin_polarised)
            {
                let nup = electronic_param
                    .nup
                    .expect("NUP of the system is not determined!");
                let ndown = electronic_param
                    .ndown
                    .expect("NDOWN of the system is not determined!");
                let max = if nup.value() > ndown.value() {
                    nup.value()
                } else {
                    ndown.value()
                };
                return Self(max as u32 + nextra);
            }
            todo!()
        }
    }
}

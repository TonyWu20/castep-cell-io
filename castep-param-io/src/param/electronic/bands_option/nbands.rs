use crate::parser::{data_type::PosInteger, Rule};
use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::{Parser, Span};
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::param::{ElectronicParam, SpinPolarised};

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
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field = "NBANDS", from = u64, value=u64)]
#[pest_ast(rule(Rule::nbands))]
#[pest_rule(rule=Rule::nbands,keyword="NBANDS")]
pub struct Nbands(
    #[pest_ast(inner(
        rule(Rule::pos_integer),
        with(PosInteger::new),
        with(u64::try_from),
        with(Result::unwrap)
    ))]
    u64,
);

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
            return Self(nelectrons.value() as u64 / 2 + nextra);
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
            return Self(max as u64 + nextra);
        }
        todo!()
    }
}

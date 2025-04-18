use crate::parser::{data_type::PosInteger, Rule};
use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::param::Nbands;
/// This keyword determines the number of bands at each k-point when performing
/// a band structure calculation.
/// There are three ways in which you can specify the number of bands at each
/// k-point:
/// 1. Directly, using BS_NBANDS.
/// 2. Indirectly, by specifying the number of extra bands in addition to the
///    number of occupied bands using BS_NEXTRA_BANDS.
/// 3. Indirectly, by specifying the number of extra bands in addition to
///    the number of occupied bands as a percentage of the latter value using
///    BS_PERC_EXTRA_BANDS.
/// # Note
/// It is not possible to have both the BS_NBANDS keyword and either the
/// BS_NEXTRA_BANDS or BS_PERC_EXTRA_BANDS keywords present in the same input file.
/// # Default
/// NBANDS + 5√NBANDS
/// # Example
/// `BS_NBANDS : 64`
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Hash,
    KeywordDisplay,
    Default,
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field="BS_NBANDS", from=u64,value=u64,)]
#[pest_ast(rule(Rule::bs_nbands))]
#[pest_rule(rule=Rule::bs_nbands,keyword="BS_NBANDS")]
pub struct BSNbands(
    #[pest_ast(inner(
        rule(Rule::pos_integer),
        with(PosInteger::new),
        with(u64::try_from),
        with(Result::unwrap)
    ))]
    u64,
);

impl BSNbands {
    fn default_value(nbands: &Nbands) -> Self {
        let value = (nbands.value() as f64 + 5.0 * (nbands.value() as f64).sqrt()) as u64;
        Self(value)
    }
}

#[cfg(test)]
mod test {
    use crate::param::KeywordDisplay;

    use super::BSNbands;

    #[test]
    fn bs_nbands() {
        let b = BSNbands::from(64);
        assert_eq!(64, b.value());
        assert_eq!("BS_NBANDS : 64", b.output());
    }
}

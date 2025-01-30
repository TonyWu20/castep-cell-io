use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::parser::data_type::Logical;
use crate::parser::Rule;

/// This keyword determines whether or not to update the estimate of the
/// Thomas-Fermi screening length in the screened exchange formalism before
/// the start of a band structure calculation.
/// # Note
/// This keyword is not relevant if RE_EST_K_SCRN : TRUE, since the reevaluation will happen automatically in this case.
/// # Default
/// FALSE
/// # Example
/// `BS_RE_EST_K_SCRN : TRUE`
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
#[keyword_display(field="BS_RE_EST_K_SCRN", from=bool, value=bool)]
#[pest_ast(rule(Rule::bs_re_est_k_scrn))]
#[pest_rule(rule=Rule::bs_re_est_k_scrn,keyword="BS_RE_EST_K_SCRN")]
pub struct BSReEstKScrn(
    #[pest_ast(inner(rule(Rule::logical), with(Logical::from), with(Logical::into)))] bool,
);
#[cfg(test)]
mod test {
    use crate::param::KeywordDisplay;

    use super::BSReEstKScrn;

    #[test]
    fn bs_re_est_k_scrn() {
        let p = BSReEstKScrn::default();
        assert_eq!("BS_RE_EST_K_SCRN : false", p.output());
        assert!(!p.value());
        let p_true = BSReEstKScrn::from(true);
        assert!(p_true.value());
    }
}

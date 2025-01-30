use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::parser::{data_type::PosInteger, Rule};

/// This keyword controls the maximum number of conjugate gradient steps taken
/// for each electronic band in the electronic minimizer before resetting to the
/// steepest descent direction, during a band structure calculation.
/// # Default
/// 4
/// # Example
/// `BS_MAX_CG_STEPS : 10`
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
    KeywordDisplay,
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field="BS_MAX_CG_STEPS", from=u64, value=u64, default_value=4)]
#[pest_ast(rule(Rule::bs_max_cg_steps))]
#[pest_rule(rule=Rule::bs_max_cg_steps,keyword="BS_MAX_CG_STEPS")]
pub struct BSMaxCgSteps(
    #[pest_ast(inner(
        rule(Rule::pos_integer),
        with(PosInteger::new),
        with(u64::try_from),
        with(Result::unwrap),
    ))]
    u64,
);

#[cfg(test)]
mod test {
    use crate::param::KeywordDisplay;

    use super::BSMaxCgSteps;

    #[test]
    fn bs_max_cg_steps() {
        let cg_steps = BSMaxCgSteps::default();
        assert_eq!(4, cg_steps.value());
        let cg_steps = BSMaxCgSteps::from(10);
        assert_eq!("BS_MAX_CG_STEPS : 10", cg_steps.output());
    }
}

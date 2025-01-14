use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

/// This keyword controls the maximum number of conjugate gradient steps taken
/// for each electronic band in the electronic minimizer before resetting to the
/// steepest descent direction, during a band structure calculation.
/// # Default
/// 4
/// # Example
/// `BS_MAX_CG_STEPS : 10`
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, KeywordDisplay,
)]
#[keyword_display(field="BS_MAX_CG_STEPS", from=u32, value=u32, default_value=4)]
pub struct BSMaxCgSteps(u32);

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

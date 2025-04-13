use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::parser::{data_type::PosInteger, Rule};
/// This keyword controls the maximum number of iterations to perform when
/// calculating band structure.
/// # Note
/// It might be necessary to increase this value if a low BS_MAX_CG_STEPS is used.
/// # Default
/// 60
/// # Example
/// `BS_MAX_ITER : 50`
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
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field="BS_MAX_ITER", from=u64,value=u64, default_value=60)]
#[pest_ast(rule(Rule::bs_max_iter))]
#[pest_rule(rule=Rule::bs_max_iter,keyword="BS_MAX_ITER")]
pub struct BSMaxIter(
    #[pest_ast(inner(
        rule(Rule::pos_integer),
        with(PosInteger::new),
        with(u64::try_from),
        with(Result::unwrap)
    ))]
    u64,
);

#[cfg(test)]
mod test {
    use crate::param::KeywordDisplay;

    use super::BSMaxIter;

    #[test]
    fn bs_max_iter() {
        let m = BSMaxIter::default();
        assert_eq!(60, m.value());
        let m = BSMaxIter::from(120);
        assert_eq!("BS_MAX_ITER : 120", m.output());
    }
}

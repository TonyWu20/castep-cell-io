use castep_param_derive::KeywordDisplayStruct;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::param::InvLengthUnit;

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplayStruct, Builder,
)]
#[keyword_display(field = "MIX_METRIC_Q", display_format = "{} {}", from=f64, default_value=-1.0)]
/// This keyword determines the weighting factor for the densities used in
/// the density mixing scheme.
/// CASTEP uses a weighting factor when evaluating scalar products of densities.
/// The factor depends on the wave vector q, according to:
/// `f(q) = (q2 + q12)/q2`
/// where q1 is the value of the MIX_METRIC_Q parameter.
/// CASTEP sets the value of q1 automatically if MIX_METRIC_Q is not specified.
/// # Default
/// -1 - CASTEP will automatically select the appropriate value
/// # Example
/// `MIX_METRIC_Q : 20.0 1/ang`
pub struct MixMetricQ {
    pub q: f64,
    #[keyword_display(is_option = true)]
    pub unit: Option<InvLengthUnit>,
}

#[cfg(test)]
mod test {
    use crate::param::KeywordDisplay;

    use super::MixMetricQ;

    #[test]
    fn mix_metric_q() {
        let q = MixMetricQ::default();
        println!("{}", q.output());
        let q = MixMetricQ::from(20.0);
        println!("{}", q.output());
    }
}

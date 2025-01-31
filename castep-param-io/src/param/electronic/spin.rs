use crate::parser::{data_type::Real, Rule};
use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
    Default,
    KeywordDisplay,
    FromPest,
    BuildFromPairs,
)]
#[keyword_display(field = "SPIN", from=f64, value=f64)]
#[pest_ast(rule(Rule::spin))]
#[pest_rule(rule=Rule::spin,keyword="SPIN")]
/// This keyword determines the initial value for the number of unpaired electrons in a spin-polarized calculation. This value may be optimized during the CASTEP calculation depending on the values of SPIN_FIX and FIX_OCCUPANCY.
/// The SPIN keyword cannot be used in conjunction with either of NUP or NDOWN keywords.
/// # Default
/// 0 when the total number of electrons in the system is even.
/// 1 when the total number of electrons in the system is odd.
pub struct Spin(#[pest_ast(inner(rule(Rule::real), with(Real::from_span), with(f64::from)))] f64);

#[cfg(test)]
mod test {
    use crate::param::KeywordDisplay;

    use super::Spin;

    #[test]
    fn spin() {
        let spin = Spin(2.0);
        println!("{}", spin.output())
    }
}

use crate::parser::{data_type::Real, Rule};
use castep_param_derive::{BuildFromPairs, KeywordDisplayStruct};
use derive_builder::Builder;
use from_pest::FromPest;
use pest::Parser;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::param::InvLengthUnit;

/// This keyword determines the maximum size of the g-vectors included in the fine grid.
/// The fine grid is seDefault
/// -1 a0-1 - this results in the fine and standard grids being identical
/// # Example
/// FINE_GMAX : 20 1/angt up such that all g-vectors with |g| less than or equal to FINE_GMAX are included.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    PartialOrd,
    Builder,
    KeywordDisplayStruct,
    FromPest,
    BuildFromPairs,
)]
#[builder(setter(into), default)]
#[keyword_display(field = "FINE_GMAX", display_format="{:20.15} {}", default_value=-1.0, from=f64)]
#[pest_ast(rule(Rule::fine_gmax))]
#[pest_rule(rule=Rule::fine_gmax,keyword="FINE_GMAX")]
pub struct FineGMax {
    #[pest_ast(inner(rule(Rule::real), with(Real::from_span), with(f64::from)))]
    pub max: f64,
    #[keyword_display(is_option = true)]
    #[pest_ast(inner(rule(Rule::fine_gmax_unit), with(InvLengthUnit::from_span)))]
    pub unit: Option<InvLengthUnit>,
}

#[cfg(test)]
mod test {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::parser::{ConsumeKVPairs, ParamFile, ParamParser, Rule};

    use super::FineGMax;

    #[test]
    fn fine_gmax_parse() {
        let input = ["fine_gmax : 2.0000", "fine_gmax : 2.0000 1/nm"];
        input.iter().for_each(|&input| {
            let mut parse = ParamParser::parse(Rule::param_file, input).unwrap();
            let file = ParamFile::from_pest(&mut parse).unwrap();
            dbg!(&file);
            let gmax = FineGMax::find_from_pairs(file.items());
            dbg!(gmax);
        });
    }
}

use castep_param_derive::KeywordDisplay;
use from_pest::FromPest;
use pest::{Parser, Span};
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::parser::{
    data_type::{PosInteger, Real},
    ConsumeKVPairs, ParamParser, Rule,
};

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplay, FromPest,
)]
/// This keyword controls the percentage of extra bands in addition to the number
/// of occupied bands. These extra bands are necessary for metals or finite
/// temperature insulators.
/// # Note
/// It is not possible to have both the NBANDS keyword and either the NEXTRA_BANDS
/// or PERC_EXTRA_BANDS keywords present in the same input file.
#[keyword_display(specified_fields = true)]
#[pest_ast(rule(Rule::extra_bands))]
pub enum ExtraBands {
    #[keyword_display(field = "NEXTRA_BANDS")]
    NextraBands(
        #[pest_ast(inner(
            rule(Rule::nextra_bands),
            with(NextraBands::from_span),
            with(NextraBands::get_u64_from_nextra)
        ))]
        u64,
    ),
    #[keyword_display(field = "PERC_EXTRA_BANDS")]
    PercExtraBands(
        #[pest_ast(inner(
            rule(Rule::perc_extra_bands),
            with(PercExtraBands::from_span),
            with(PercExtraBands::get_f64_from_nextra)
        ))]
        f64,
    ),
}

#[derive(Debug, Clone, Copy, FromPest)]
#[pest_ast(rule(Rule::nextra_bands))]
struct NextraBands<'a>(PosInteger<'a>);

impl<'a> NextraBands<'a> {
    fn from_span(span: Span<'a>) -> Self {
        let input = span.as_str();
        let mut parsed = ParamParser::parse(Rule::nextra_bands, input).unwrap();
        Self::from_pest(&mut parsed).unwrap()
    }
    fn get_u64_from_nextra(nextra: NextraBands<'_>) -> u64 {
        nextra.0.try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, FromPest)]
#[pest_ast(rule(Rule::perc_extra_bands))]
struct PercExtraBands<'a>(Real<'a>);

impl<'a> PercExtraBands<'a> {
    fn from_span(span: Span<'a>) -> Self {
        let input = span.as_str();
        let mut parsed = ParamParser::parse(Rule::perc_extra_bands, input).unwrap();
        Self::from_pest(&mut parsed).unwrap()
    }
    fn get_f64_from_nextra(perc_extra: PercExtraBands<'_>) -> f64 {
        perc_extra.0.into()
    }
}

impl<'a> ConsumeKVPairs<'a> for ExtraBands {
    type Item = ExtraBands;

    fn find_from_pairs(items: &'a [crate::parser::ParamItems<'a>]) -> Option<Self::Item> {
        {
            let nextra = items
                .iter()
                .position(|p| p.keyword().to_lowercase() == "nextra_bands");
            let perc_extra = items
                .iter()
                .position(|p| p.keyword().to_lowercase() == "perc_extra_bands");
            let item = match (nextra, perc_extra) {
                (None, None) => None,
                (None, Some(i)) | (Some(i), None) => Some(items[i]),
                (Some(n), Some(p)) => {
                    let to_use = if n > p { n } else { p };
                    Some(items[to_use])
                }
            };
            item.map(|i| {
                let input = i.to_string();
                let mut parsed =
                    ParamParser::parse(Rule::extra_bands, &input).expect("Always correct");
                ExtraBands::from_pest(&mut parsed).expect("Always correct")
            })
        }
    }
}

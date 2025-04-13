use crate::parser::{
    data_type::{PosInteger, Real},
    ConsumeKVPairs, ParamParser, Rule,
};
use castep_param_derive::KeywordDisplay;
use from_pest::FromPest;
use pest::{Parser, Span};
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, KeywordDisplay, FromPest,
)]
#[keyword_display(specified_fields = true)]
#[pest_ast(rule(Rule::bs_extra_bands))]
pub enum BSExtraBands {
    #[keyword_display(field = "BS_NEXTRA_BANDS")]
    /// This keyword controls the number of extra bands at each k-point in addition to the number of occupied bands, when performing a band structure calculation.
    Nextra(
        #[pest_ast(inner(
            rule(Rule::bs_nextra_bands),
            with(BSNextraBands::from_span),
            with(BSNextraBands::get_u64_from_nextra)
        ))]
        u64,
    ),
    #[keyword_display(field = "BS_PERC_EXTRA_BANDS")]
    /// This keyword controls the number of extra bands at each k-point in addition to the number of occupied bands, when performing a band structure calculation.
    PercExtra(
        #[pest_ast(inner(
            rule(Rule::bs_perc_extra_bands),
            with(BSPercExtraBands::from_span),
            with(BSPercExtraBands::get_f64_from_nextra)
        ))]
        f64,
    ),
}

impl Default for BSExtraBands {
    fn default() -> Self {
        Self::PercExtra(0.0)
    }
}

#[derive(Debug, Clone, Copy, FromPest)]
#[pest_ast(rule(Rule::bs_nextra_bands))]
struct BSNextraBands<'a>(PosInteger<'a>);

impl<'a> BSNextraBands<'a> {
    fn from_span(span: Span<'a>) -> Self {
        let input = span.as_str();
        let mut parsed = ParamParser::parse(Rule::bs_nextra_bands, input).unwrap();
        Self::from_pest(&mut parsed).unwrap()
    }
    fn get_u64_from_nextra(nextra: BSNextraBands<'_>) -> u64 {
        nextra.0.try_into().unwrap()
    }
}

#[derive(Debug, Clone, Copy, FromPest)]
#[pest_ast(rule(Rule::bs_perc_extra_bands))]
struct BSPercExtraBands<'a>(Real<'a>);

impl<'a> BSPercExtraBands<'a> {
    fn from_span(span: Span<'a>) -> Self {
        let input = span.as_str();
        let mut parsed = ParamParser::parse(Rule::bs_perc_extra_bands, input).unwrap();
        Self::from_pest(&mut parsed).unwrap()
    }
    fn get_f64_from_nextra(perc_extra: BSPercExtraBands<'_>) -> f64 {
        perc_extra.0.into()
    }
}

impl<'a> ConsumeKVPairs<'a> for BSExtraBands {
    type Item = BSExtraBands;

    fn find_from_pairs(items: &'a [crate::parser::ParamItems<'a>]) -> Option<Self::Item> {
        {
            let nextra = items
                .iter()
                .position(|p| p.keyword().to_lowercase() == "bs_nextra_bands");
            let perc_extra = items
                .iter()
                .position(|p| p.keyword().to_lowercase() == "bs_perc_extra_bands");
            match (nextra, perc_extra) {
                (None, None) => None,
                (None, Some(i)) | (Some(i), None) => {
                    let item = items[i];
                    let input = item.to_string();
                    let mut parsed =
                        ParamParser::parse(Rule::bs_extra_bands, &input).expect("Always correct");
                    Some(BSExtraBands::from_pest(&mut parsed).expect("Always correct"))
                }
                (Some(n), Some(p)) => {
                    let to_use = if n > p { n } else { p };
                    let item = items[to_use];
                    let input = item.to_string();
                    let mut parsed =
                        ParamParser::parse(Rule::bs_extra_bands, &input).expect("Always correct");
                    Some(BSExtraBands::from_pest(&mut parsed).expect("Always correct"))
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use from_pest::FromPest;
    use pest::Parser;

    use crate::{
        param::KeywordDisplay,
        parser::{ConsumeKVPairs, ParamFile, ParamParser, Rule},
    };

    use super::BSExtraBands;

    #[test]
    fn bs_extra_bands() {
        let nextra = BSExtraBands::Nextra(64);
        assert_eq!(nextra.output(), "BS_NEXTRA_BANDS : 64");
        let perc_extra = BSExtraBands::PercExtra(72.0);
        assert_eq!(perc_extra.output(), "BS_PERC_EXTRA_BANDS : 72");
        let input = perc_extra.output();
        let mut parsed = ParamParser::parse(Rule::param_file, &input).unwrap();
        let file = ParamFile::from_pest(&mut parsed).unwrap();
        dbg!(&file);
        let found = BSExtraBands::find_from_pairs(file.items());
        dbg!(found);
        let mut parsed = ParamParser::parse(Rule::bs_extra_bands, &input).unwrap();
        let extra = BSExtraBands::from_pest(&mut parsed).expect("Correct");
        dbg!(extra);
    }
}

use castep_param_derive::{BuildFromPairs, KeywordDisplay};
use from_pest::FromPest;
use pest::{Parser, Span};
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

use crate::parser::Rule;

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    Hash,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    KeywordDisplay,
    FromPest,
    BuildFromPairs,
)]
/// This keyword determines the parallelization strategy used. Available options are:
/// `Kpoint` - only k-point parallelization will be used (best scalability)
/// `Gvector` - only g-vector parallelization will be used (worst scalability)
/// `Mixed` - a combination of k-point and g-vector parallelization will be used
/// `Default` - the optimal parallelization strategy for the architecture will be used
/// The Default setting is appropriate in most situations.
/// The Kpoint option is available only when the number of processors is a factor of the number of k-points.
/// The Gvector option is appropriate for calculations involving large systems where often only one k-point is used.
/// The Mixed option is available when the number of processors and the number of k-points have a common factor (for example 6 k-points and 4 processors).
/// The differences in the scaling properties of the different schemes dictate the number of processors that should be used. For example, a calculation that uses 16 k-points is likely to finish faster on 4 processors (using Kpoint distribution) than on 6 processors (using Mixed distribution) and most likely faster than on 9 processors (using Gvector distribution).
/// # Default
/// `Default`
/// # Example
/// `DATA_DISTRIBUTION : Gvector`
#[keyword_display(field = "DATA_DISTRIBUTION")]
#[pest_ast(rule(Rule::data_distribution))]
#[pest_rule(rule=Rule::data_distribution, keyword="DATA_DISTRIBUTION")]
pub enum DataDistribution {
    #[pest_ast(inner(rule(Rule::data_distribution_value), with(span_into_data_distribution)))]
    Kpoint,
    Gvector,
    Mixed,
    #[default]
    Default,
}

fn span_into_data_distribution(span: Span<'_>) -> DataDistribution {
    let value = span.as_str().to_lowercase();
    let matched = match value.as_str() {
        "kpoint" => Some(DataDistribution::Kpoint),
        "gvector" => Some(DataDistribution::Gvector),
        "mixed" => Some(DataDistribution::Mixed),
        "default" => Some(DataDistribution::Default),
        _ => None,
    };
    matched.expect("Always correct from parsed result.")
}

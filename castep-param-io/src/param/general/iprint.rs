use std::fmt::Display;

use crate::parser::Rule;
use castep_param_derive::KeywordDisplay;
use pest::Span;
use pest_ast::FromPest;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    KeywordDisplay,
    Serialize,
    Deserialize,
    FromPest,
)]
#[keyword_display(field = "IPRINT", direct_display = false)]
#[pest_ast(rule(Rule::iprint))]
/// This keyword controls the level of verbosity of the output. Possible
///  values range from 0, which produces minimal output, to 3,
/// which corresponds to full debugging output.
/// # Default
/// 1
/// # Example
/// `IPRINT : 1`
pub enum IPrint {
    #[pest_ast(inner(rule(Rule::iprint_value), with(span_into_iprint)))]
    Minimal,
    #[default]
    Default,
    Verbose,
    Full,
}

fn span_into_iprint(span: Span<'_>) -> IPrint {
    let value = span.as_str();
    let matched = match value {
        "0" => Some(IPrint::Minimal),
        "1" => Some(IPrint::Default),
        "2" => Some(IPrint::Verbose),
        "3" => Some(IPrint::Full),
        _ => None,
    };
    matched.expect("Always correct from parsed value.")
}

impl Display for IPrint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as usize)
    }
}

#[cfg(test)]
mod test {
    use crate::param::KeywordDisplay;

    use super::IPrint;

    #[test]
    fn iprint() {
        let iprint = IPrint::default();
        assert_eq!("IPRINT : 1", iprint.output());
    }
}

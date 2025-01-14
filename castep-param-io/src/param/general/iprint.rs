use std::fmt::Display;

use castep_param_derive::KeywordDisplay;
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
)]
#[keyword_display(field = "IPRINT", direct_display = false)]
/// This keyword controls the level of verbosity of the output. Possible
///  values range from 0, which produces minimal output, to 3,
/// which corresponds to full debugging output.
/// # Default
/// 1
/// # Example
/// `IPRINT : 1`
pub enum IPrint {
    Minimal,
    #[default]
    Default,
    Verbose,
    Full,
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

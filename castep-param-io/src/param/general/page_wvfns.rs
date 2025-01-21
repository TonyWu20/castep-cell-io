
use castep_param_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};


///This keyword controls the paging of wavefunctions to disk in order to save memory. Available options are:
/// - > 0 - all wavefunctions requiring more memory than this value in megabytes will be paged to disk.
/// - 0 - no paging will be performed.
/// - < 0 - all wavefunctions will be paged to disk.
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    KeywordDisplay,
)]
#[keyword_display(field="PAGE_WVFNS",from=i64,value=i64)]
pub struct PageWvfns(i64);

impl From<i32> for PageWvfns {
    fn from(value: i32) -> Self {
        Self(value as i64)
    }
}

impl From<i16> for PageWvfns {
    fn from(value: i16) -> Self {
        Self(value as i64)
    }
}

impl From<i8> for PageWvfns {
    fn from(value: i8) -> Self {
        Self(value as i64)
    }
}

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord, Default,
)]
/// This keyword specifies the units in which time will be reported.
/// # Example
/// `TIME_UNIT : aut`
pub enum TimeUnit {
    AtomicUnitOfTime,
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
    #[default]
    Picosecond,
    Femtosecond,
}

impl Display for TimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeUnit::AtomicUnitOfTime => f.write_str("aut"),
            TimeUnit::Second => f.write_str("s"),
            TimeUnit::Millisecond => f.write_str("ms"),
            TimeUnit::Microsecond => f.write_str("mus"),
            TimeUnit::Nanosecond => f.write_str("ns"),
            TimeUnit::Picosecond => f.write_str("ps"),
            TimeUnit::Femtosecond => f.write_str("fs"),
        }
    }
}

impl KeywordDisplay for TimeUnit {
    fn field(&self) -> String {
        "TIME_UNIT".to_string()
    }
}

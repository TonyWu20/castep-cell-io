use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::FromCellValue;
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Specifies the units in which time will be reported.
///
/// Keyword type: String
///
/// Default: TimeUnit::Picosecond
///
/// Example:
/// TIME_UNIT : aut
#[derive(
    Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename = "TIME_UNIT")]
pub enum TimeUnit {
    /// Atomic unit of time
    #[serde(alias = "AUT", alias = "aut")]
    AtomicUnitOfTime,
    /// Second
    #[serde(alias = "S", alias = "s")]
    Second,
    /// Millisecond
    #[serde(alias = "MS", alias = "ms")]
    Millisecond,
    /// Microsecond
    #[serde(alias = "MUS", alias = "mus")]
    Microsecond,
    /// Nanosecond
    #[serde(alias = "NS", alias = "ns")]
    Nanosecond,
    /// Picosecond
    #[serde(alias = "PS", alias = "ps")]
    #[default]
    Picosecond,
    /// Femtosecond
    #[serde(alias = "FS", alias = "fs")]
    Femtosecond,
}

impl FromCellValue for TimeUnit {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "aut" => Ok(Self::AtomicUnitOfTime),
            "s" => Ok(Self::Second),
            "ms" => Ok(Self::Millisecond),
            "mus" => Ok(Self::Microsecond),
            "ns" => Ok(Self::Nanosecond),
            "ps" => Ok(Self::Picosecond),
            "fs" => Ok(Self::Femtosecond),
            other => Err(Error::Message(format!(
                "unknown TimeUnit: {other}"
            ))),
        }
    }
}

impl ToCell for TimeUnit {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("TIME_UNIT", self.to_cell_value())
    }
}

impl ToCellValue for TimeUnit {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                TimeUnit::AtomicUnitOfTime => "aut",
                TimeUnit::Second => "s",
                TimeUnit::Millisecond => "ms",
                TimeUnit::Microsecond => "mus",
                TimeUnit::Nanosecond => "ns",
                TimeUnit::Picosecond => "ps",
                TimeUnit::Femtosecond => "fs",
            }
            .to_string(),
        )
    }
}



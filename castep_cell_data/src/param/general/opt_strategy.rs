use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::query::value_as_str;
use castep_cell_io::Error;

/// Determines the optimization strategy used when there are multiple strategies
/// available for the selected algorithm.
///
/// Keyword type: String
///
/// Default: OptStrategy::Default
///
/// Example:
/// OPT_STRATEGY : Memory
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OptStrategy {
    /// Maximizes performance at the cost of additional memory usage.
    Speed,
    /// Balances performance and memory usage.
    #[default]
    Default,
    /// Minimizes memory usage at a cost of decreased performance.
    Memory,
}

impl FromCellValue for OptStrategy {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "speed" => Ok(Self::Speed),
            "default" => Ok(Self::Default),
            "memory" => Ok(Self::Memory),
            other => Err(Error::Message(format!("unknown OptStrategy: {other}"))),
        }
    }
}

impl FromKeyValue for OptStrategy {
    const KEY_NAME: &'static str = "OPT_STRATEGY";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for OptStrategy {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("OPT_STRATEGY", self.to_cell_value())
    }
}

impl ToCellValue for OptStrategy {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                OptStrategy::Speed => "Speed",
                OptStrategy::Default => "Default",
                OptStrategy::Memory => "Memory",
            }
            .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_io::CellValue;

    #[test]
    fn test_case_insensitive() {
        assert_eq!(OptStrategy::from_cell_value(&CellValue::Str("speed")).unwrap(), OptStrategy::Speed);
        assert_eq!(OptStrategy::from_cell_value(&CellValue::Str("SPEED")).unwrap(), OptStrategy::Speed);
        assert_eq!(OptStrategy::from_cell_value(&CellValue::Str("memory")).unwrap(), OptStrategy::Memory);
        assert_eq!(OptStrategy::from_cell_value(&CellValue::Str("MEMORY")).unwrap(), OptStrategy::Memory);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(OptStrategy::from_cell_value(&CellValue::Str("default")).unwrap(), OptStrategy::Default);
    }

    #[test]
    fn test_invalid() {
        assert!(OptStrategy::from_cell_value(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(OptStrategy::KEY_NAME, "OPT_STRATEGY");
    }
}


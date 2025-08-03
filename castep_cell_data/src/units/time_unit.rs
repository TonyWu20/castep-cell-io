use castep_cell_serde::{Cell, CellValue, ToCell, ToCellValue};
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

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_serde::{ToCell, from_str, to_string};
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_time_unit_serde() {
        let test_cases = [
            ("TIME_UNIT : aut", TimeUnit::AtomicUnitOfTime),
            ("TIME_UNIT : s", TimeUnit::Second),
            ("TIME_UNIT : ps", TimeUnit::Picosecond),
            ("TIME_UNIT : fs", TimeUnit::Femtosecond),
        ];

        for (input_str, expected_unit) in test_cases {
            #[derive(Debug, Deserialize, Serialize)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            struct CellFileWithTimeUnit {
                time_unit: TimeUnit,
            }

            let cell_file_result: Result<CellFileWithTimeUnit, _> = from_str(input_str);
            assert!(
                cell_file_result.is_ok(),
                "Deserialization failed for '{}': {:?}",
                input_str,
                cell_file_result.err()
            );
            let cell_file = cell_file_result.unwrap();
            assert_eq!(
                cell_file.time_unit, expected_unit,
                "Failed for input: {input_str}"
            );
        }

        let time_unit_instance = TimeUnit::AtomicUnitOfTime;
        let serialized_result = to_string(&time_unit_instance.to_cell());
        assert!(
            serialized_result.is_ok(),
            "Serialization failed: {:?}",
            serialized_result.err()
        );
        let serialized_string = serialized_result.unwrap();
        println!("Serialized TIME_UNIT (aut): {serialized_string}");
        assert!(serialized_string.contains("TIME_UNIT"));
        assert!(serialized_string.contains("aut"));

        assert_eq!(TimeUnit::default(), TimeUnit::Picosecond);
    }
}

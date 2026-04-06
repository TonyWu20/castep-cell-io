use castep_cell_fmt::query::value_as_str;
use castep_cell_fmt::{CResult, Cell, CellValue, FromKeyValue, ToCell, ToCellValue};

/// Controls the method used to minimize electronic states.
///
/// Keyword type: String
///
/// Default: ElectronicMinimizer::Cg
///
/// Example:
/// ELECTRONIC_MINIMIZER : SD
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[non_exhaustive]
#[derive(Default)]
pub enum ElectronicMinimizer {
    /// Minimizer takes up to 10 SD steps
    Sd,
    /// Minimizer takes one SD step followed by up to 4 CG steps
    #[default]
    Cg,
    #[doc(hidden)]
    RmmDiis,
}


impl FromKeyValue for ElectronicMinimizer {
    const KEY_NAME: &'static str = "ELECTRONIC_MINIMIZER";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "sd" => Ok(Self::Sd),
            "cg" => Ok(Self::Cg),
            "rmm/diis" => Ok(Self::RmmDiis),
            other => Err(castep_cell_fmt::Error::Message(format!(
                "unknown ElectronicMinimizer: {other}"
            ))),
        }
    }
}

impl ToCell for ElectronicMinimizer {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("ELECTRONIC_MINIMIZER", self.to_cell_value())
    }
}

impl ToCellValue for ElectronicMinimizer {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                ElectronicMinimizer::Sd => "SD",
                ElectronicMinimizer::Cg => "CG",
                ElectronicMinimizer::RmmDiis => "RMM/DIIS",
            }
            .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_fmt::CellValue;

    #[test]
    fn test_case_insensitive() {
        assert_eq!(
            ElectronicMinimizer::from_cell_value_kv(&CellValue::Str("sd")).unwrap(),
            ElectronicMinimizer::Sd
        );
        assert_eq!(
            ElectronicMinimizer::from_cell_value_kv(&CellValue::Str("SD")).unwrap(),
            ElectronicMinimizer::Sd
        );
        assert_eq!(
            ElectronicMinimizer::from_cell_value_kv(&CellValue::Str("cg")).unwrap(),
            ElectronicMinimizer::Cg
        );
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(
            ElectronicMinimizer::from_cell_value_kv(&CellValue::Str("rmm/diis")).unwrap(),
            ElectronicMinimizer::RmmDiis
        );
    }

    #[test]
    fn test_invalid() {
        assert!(ElectronicMinimizer::from_cell_value_kv(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(ElectronicMinimizer::KEY_NAME, "ELECTRONIC_MINIMIZER");
    }
}

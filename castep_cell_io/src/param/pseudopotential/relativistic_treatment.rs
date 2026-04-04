use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;

/// Selects the method used to treat relativistic effects.
///
/// Keyword type: String
///
/// Default: RelativisticTreatment::KoellingHarmon
///
/// Example:
/// RELATIVISTIC_TREATMENT : ZORA
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RelativisticTreatment {
    /// Completely non-relativistic pseudopotentials
    Schroedinger,
    /// Scalar relativistic treatment (ZORA)
    Zora,
    /// Scalar relativistic treatment (Koelling-Harmon)
    #[default]
    KoellingHarmon,
    /// Fully relativistic treatment
    Dirac,
}

impl FromCellValue for RelativisticTreatment {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "schroedinger" => Ok(Self::Schroedinger),
            "zora" => Ok(Self::Zora),
            "koelling-harmon" => Ok(Self::KoellingHarmon),
            "dirac" => Ok(Self::Dirac),
            other => Err(Error::Message(format!("unknown RelativisticTreatment: {other}"))),
        }
    }
}

impl FromKeyValue for RelativisticTreatment {
    const KEY_NAME: &'static str = "RELATIVISTIC_TREATMENT";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for RelativisticTreatment {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("RELATIVISTIC_TREATMENT", self.to_cell_value())
    }
}

impl ToCellValue for RelativisticTreatment {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                RelativisticTreatment::Schroedinger => "SCHROEDINGER",
                RelativisticTreatment::Zora => "ZORA",
                RelativisticTreatment::KoellingHarmon => "KOELLING-HARMON",
                RelativisticTreatment::Dirac => "DIRAC",
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
        assert_eq!(RelativisticTreatment::from_cell_value(&CellValue::Str("schroedinger")).unwrap(), RelativisticTreatment::Schroedinger);
        assert_eq!(RelativisticTreatment::from_cell_value(&CellValue::Str("SCHROEDINGER")).unwrap(), RelativisticTreatment::Schroedinger);
        assert_eq!(RelativisticTreatment::from_cell_value(&CellValue::Str("zora")).unwrap(), RelativisticTreatment::Zora);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(RelativisticTreatment::from_cell_value(&CellValue::Str("koelling-harmon")).unwrap(), RelativisticTreatment::KoellingHarmon);
        assert_eq!(RelativisticTreatment::from_cell_value(&CellValue::Str("dirac")).unwrap(), RelativisticTreatment::Dirac);
    }

    #[test]
    fn test_invalid() {
        assert!(RelativisticTreatment::from_cell_value(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(RelativisticTreatment::KEY_NAME, "RELATIVISTIC_TREATMENT");
    }
}

use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, CResult, Error};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Defines the type of NMR calculation to be performed.
///
/// Keyword type: String
///
/// Default: MagresTask::Shielding
///
/// Example:
/// MAGRES_TASK : NMR
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MAGRES_TASK")]
pub enum MagresTask {
    /// Performs a calculation of the NMR shielding tensor for all atoms
    #[serde(alias = "shielding", alias = "SHIELDING")]
    Shielding,
    /// Performs a calculation of the electric field gradient tensor for all atoms
    #[serde(alias = "efg", alias = "EFG")]
    Efg,
    /// Performs a calculation of both the NMR shielding tensor and the EFG tensor
    #[serde(alias = "nmr", alias = "NMR")]
    Nmr,
}

impl Default for MagresTask {
    fn default() -> Self {
        Self::Shielding // Default is Shielding
    }
}

impl FromCellValue for MagresTask {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "shielding" => Ok(Self::Shielding),
            "efg" => Ok(Self::Efg),
            "nmr" => Ok(Self::Nmr),
            other => Err(Error::Message(format!("unknown MagresTask: {other}"))),
        }
    }
}

impl FromKeyValue for MagresTask {
    const KEY_NAME: &'static str = "MAGRES_TASK";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MagresTask {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("MAGRES_TASK", self.to_cell_value())
    }
}

impl ToCellValue for MagresTask {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                MagresTask::Shielding => "Shielding",
                MagresTask::Efg => "EFG",
                MagresTask::Nmr => "NMR",
            }
            .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_fmt::CellValue;
    use castep_cell_fmt::parse::FromCellValue;

    #[test]
    fn test_case_insensitive() {
        assert_eq!(MagresTask::from_cell_value(&CellValue::Str("shielding")).unwrap(), MagresTask::Shielding);
        assert_eq!(MagresTask::from_cell_value(&CellValue::Str("SHIELDING")).unwrap(), MagresTask::Shielding);
        assert_eq!(MagresTask::from_cell_value(&CellValue::Str("efg")).unwrap(), MagresTask::Efg);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(MagresTask::from_cell_value(&CellValue::Str("nmr")).unwrap(), MagresTask::Nmr);
    }

    #[test]
    fn test_invalid() {
        assert!(MagresTask::from_cell_value(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(MagresTask::KEY_NAME, "MAGRES_TASK");
    }
}


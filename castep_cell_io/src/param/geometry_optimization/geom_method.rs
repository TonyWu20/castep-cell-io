use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Determines the method used for geometry optimization.
///
/// Keyword type: String
///
/// Default: GeomMethod::Bfgs
///
/// Example:
/// GEOM_METHOD : DampedMD
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "GEOM_METHOD")]
#[derive(Default)]
pub enum GeomMethod {
    /// BFGS minimization
    #[serde(alias = "bfgs", alias = "BFGS")]
    #[default]
    Bfgs,
    /// Low-memory BFGS minimization
    #[serde(alias = "lbfgs", alias = "LBFGS")]
    Lbfgs,
    /// BFGS minimization using delocalized internal coordinates
    #[serde(rename = "Delocalized", alias = "delocalized", alias = "DELOCALIZED")]
    #[serde(alias = "Delocalised", alias = "delocalised", alias = "DELOCALISED")]
    // Alternative spelling
    Delocalized,
    /// Damped molecular dynamics
    #[serde(alias = "dampedmd", alias = "DAMPEDMD", alias = "DampedMD")]
    DampedMd,
    /// Two-point steepest descent
    #[serde(alias = "tpsd", alias = "TPSD")]
    Tpsd,
}

impl FromCellValue for GeomMethod {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "bfgs" => Ok(Self::Bfgs),
            "lbfgs" => Ok(Self::Lbfgs),
            "delocalized" | "delocalised" => Ok(Self::Delocalized),
            "dampedmd" => Ok(Self::DampedMd),
            "tpsd" => Ok(Self::Tpsd),
            other => Err(Error::Message(format!("unknown GeomMethod: {other}"))),
        }
    }
}

impl FromKeyValue for GeomMethod {
    const KEY_NAME: &'static str = "GEOM_METHOD";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}


impl ToCell for GeomMethod {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("GEOM_METHOD", self.to_cell_value())
    }
}

impl ToCellValue for GeomMethod {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                GeomMethod::Bfgs => "BFGS",
                GeomMethod::Lbfgs => "LBFGS",
                GeomMethod::Delocalized => "Delocalized",
                GeomMethod::DampedMd => "DampedMD",
                GeomMethod::Tpsd => "TPSD",
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
        assert_eq!(GeomMethod::from_cell_value(&CellValue::Str("bfgs")).unwrap(), GeomMethod::Bfgs);
        assert_eq!(GeomMethod::from_cell_value(&CellValue::Str("BFGS")).unwrap(), GeomMethod::Bfgs);
        assert_eq!(GeomMethod::from_cell_value(&CellValue::Str("lbfgs")).unwrap(), GeomMethod::Lbfgs);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(GeomMethod::from_cell_value(&CellValue::Str("delocalized")).unwrap(), GeomMethod::Delocalized);
        assert_eq!(GeomMethod::from_cell_value(&CellValue::Str("delocalised")).unwrap(), GeomMethod::Delocalized);
        assert_eq!(GeomMethod::from_cell_value(&CellValue::Str("dampedmd")).unwrap(), GeomMethod::DampedMd);
        assert_eq!(GeomMethod::from_cell_value(&CellValue::Str("tpsd")).unwrap(), GeomMethod::Tpsd);
    }

    #[test]
    fn test_invalid() {
        assert!(GeomMethod::from_cell_value(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(GeomMethod::KEY_NAME, "GEOM_METHOD");
    }
}


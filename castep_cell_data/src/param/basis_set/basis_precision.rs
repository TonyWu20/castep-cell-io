use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Specifies the precision of the basis set by choosing the level of convergence
/// of atomic energies with respect to the plane wave cutoff energy.
///
/// Keyword type: String
///
/// Default: BasisPrecision::Fine
///
/// Example:
/// BASIS_PRECISION : MEDIUM
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "BASIS_PRECISION")]
pub enum BasisPrecision {
    /// Convergence of about 1 eV/atom
    #[serde(alias = "coarse", alias = "COARSE")]
    Coarse,
    /// Convergence of about 0.3 eV/atom
    #[serde(alias = "medium", alias = "MEDIUM")]
    Medium,
    /// Convergence of about 0.1 eV/atom
    #[serde(alias = "fine", alias = "FINE")]
    Fine,
    /// 1.2 × FINE cutoff energy
    #[serde(alias = "precise", alias = "PRECISE")]
    Precise,
    /// 1.6 × FINE cutoff energy, convergence of about 0.01 eV/atom
    #[serde(alias = "extreme", alias = "EXTREME")]
    Extreme,
}

impl Default for BasisPrecision {
    fn default() -> Self {
        Self::Fine
    }
}

impl FromCellValue for BasisPrecision {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "coarse" => Ok(Self::Coarse),
            "medium" => Ok(Self::Medium),
            "fine" => Ok(Self::Fine),
            "precise" => Ok(Self::Precise),
            "extreme" => Ok(Self::Extreme),
            other => Err(Error::Message(format!("unknown BasisPrecision: {other}"))),
        }
    }
}

impl FromKeyValue for BasisPrecision {
    const KEY_NAME: &'static str = "BASIS_PRECISION";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for BasisPrecision {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("BASIS_PRECISION", self.to_cell_value())
    }
}

impl ToCellValue for BasisPrecision {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                BasisPrecision::Coarse => "COARSE",
                BasisPrecision::Medium => "MEDIUM",
                BasisPrecision::Fine => "FINE",
                BasisPrecision::Precise => "PRECISE",
                BasisPrecision::Extreme => "EXTREME",
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
        assert_eq!(BasisPrecision::from_cell_value(&CellValue::Str("coarse")).unwrap(), BasisPrecision::Coarse);
        assert_eq!(BasisPrecision::from_cell_value(&CellValue::Str("COARSE")).unwrap(), BasisPrecision::Coarse);
        assert_eq!(BasisPrecision::from_cell_value(&CellValue::Str("fine")).unwrap(), BasisPrecision::Fine);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(BasisPrecision::from_cell_value(&CellValue::Str("medium")).unwrap(), BasisPrecision::Medium);
        assert_eq!(BasisPrecision::from_cell_value(&CellValue::Str("precise")).unwrap(), BasisPrecision::Precise);
        assert_eq!(BasisPrecision::from_cell_value(&CellValue::Str("extreme")).unwrap(), BasisPrecision::Extreme);
    }

    #[test]
    fn test_invalid() {
        assert!(BasisPrecision::from_cell_value(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(BasisPrecision::KEY_NAME, "BASIS_PRECISION");
    }
}


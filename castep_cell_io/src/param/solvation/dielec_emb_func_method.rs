use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_string;

/// Determines the type of dielectric function used in electrostatic embedding for implicit solvation.
///
/// Keyword type: String
///
/// Default: DielecEmbFuncMethod::Fg (if TASK is AUTOSOLVATION), DielecEmbFuncMethod::Uniform otherwise
///
/// Example:
/// DIELEC_EMB_FUNC_METHOD : FG
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DielecEmbFuncMethod {
    /// Fattebert-Gygi density-dependent dielectric (default for AUTOSOLVATION).
    Fg,
    /// Uniform dielectric (vacuum).
    Uniform,
}

impl FromCellValue for DielecEmbFuncMethod {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_string(value)?.to_ascii_lowercase().as_str() {
            "fg" => Ok(Self::Fg),
            "uniform" => Ok(Self::Uniform),
            other => Err(Error::Message(format!("unknown DielecEmbFuncMethod: {other}"))),
        }
    }
}

impl FromKeyValue for DielecEmbFuncMethod {
    const KEY_NAME: &'static str = "DIELEC_EMB_FUNC_METHOD";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for DielecEmbFuncMethod {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("DIELEC_EMB_FUNC_METHOD", self.to_cell_value())
    }
}

impl ToCellValue for DielecEmbFuncMethod {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                DielecEmbFuncMethod::Fg => "FG",
                DielecEmbFuncMethod::Uniform => "Uniform",
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
    fn test_case_insensitive_parsing() {
        let val = CellValue::Str("fg");
        assert_eq!(DielecEmbFuncMethod::from_cell_value(&val).unwrap(), DielecEmbFuncMethod::Fg);

        let val = CellValue::Str("FG");
        assert_eq!(DielecEmbFuncMethod::from_cell_value(&val).unwrap(), DielecEmbFuncMethod::Fg);

        let val = CellValue::Str("Fg");
        assert_eq!(DielecEmbFuncMethod::from_cell_value(&val).unwrap(), DielecEmbFuncMethod::Fg);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(
            DielecEmbFuncMethod::from_cell_value(&CellValue::Str("fg")).unwrap(),
            DielecEmbFuncMethod::Fg
        );
        assert_eq!(
            DielecEmbFuncMethod::from_cell_value(&CellValue::Str("uniform")).unwrap(),
            DielecEmbFuncMethod::Uniform
        );
    }

    #[test]
    fn test_invalid_variant() {
        let val = CellValue::Str("invalid");
        assert!(DielecEmbFuncMethod::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(DielecEmbFuncMethod::KEY_NAME, "DIELEC_EMB_FUNC_METHOD");
    }

    #[test]
    fn test_round_trip() {
        let variants = [DielecEmbFuncMethod::Fg, DielecEmbFuncMethod::Uniform];
        for variant in variants {
            let cell_value = variant.to_cell_value();
            let parsed = DielecEmbFuncMethod::from_cell_value(&cell_value).unwrap();
            assert_eq!(parsed, variant);
        }
    }
}

use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_string;

/// Determines appropriate boundary conditions for electrostatic embedding.
///
/// Keyword type: String
///
/// Default: BoundaryType::Periodic
///
/// Example:
/// BOUNDARY_TYPE : OPEN
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum BoundaryType {
    /// Periodic boundary conditions (default).
    #[default]
    Periodic,
    /// Open boundary conditions for embedding or solvation.
    Open,
    /// Zero boundary conditions (metallic).
    Zero,
}

impl FromCellValue for BoundaryType {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_string(value)?.to_ascii_lowercase().as_str() {
            "periodic" => Ok(Self::Periodic),
            "open" => Ok(Self::Open),
            "zero" => Ok(Self::Zero),
            other => Err(Error::Message(format!("unknown BoundaryType: {other}"))),
        }
    }
}

impl FromKeyValue for BoundaryType {
    const KEY_NAME: &'static str = "BOUNDARY_TYPE";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for BoundaryType {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("BOUNDARY_TYPE", self.to_cell_value())
    }
}

impl ToCellValue for BoundaryType {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                BoundaryType::Periodic => "Periodic",
                BoundaryType::Open => "Open",
                BoundaryType::Zero => "Zero",
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
        let val = CellValue::Str("periodic");
        assert_eq!(BoundaryType::from_cell_value(&val).unwrap(), BoundaryType::Periodic);

        let val = CellValue::Str("PERIODIC");
        assert_eq!(BoundaryType::from_cell_value(&val).unwrap(), BoundaryType::Periodic);

        let val = CellValue::Str("Periodic");
        assert_eq!(BoundaryType::from_cell_value(&val).unwrap(), BoundaryType::Periodic);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(
            BoundaryType::from_cell_value(&CellValue::Str("periodic")).unwrap(),
            BoundaryType::Periodic
        );
        assert_eq!(
            BoundaryType::from_cell_value(&CellValue::Str("open")).unwrap(),
            BoundaryType::Open
        );
        assert_eq!(
            BoundaryType::from_cell_value(&CellValue::Str("zero")).unwrap(),
            BoundaryType::Zero
        );
    }

    #[test]
    fn test_invalid_variant() {
        let val = CellValue::Str("invalid");
        assert!(BoundaryType::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(BoundaryType::KEY_NAME, "BOUNDARY_TYPE");
    }

    #[test]
    fn test_default() {
        assert_eq!(BoundaryType::default(), BoundaryType::Periodic);
    }

    #[test]
    fn test_round_trip() {
        let variants = [BoundaryType::Periodic, BoundaryType::Open, BoundaryType::Zero];
        for variant in variants {
            let cell_value = variant.to_cell_value();
            let parsed = BoundaryType::from_cell_value(&cell_value).unwrap();
            assert_eq!(parsed, variant);
        }
    }
}

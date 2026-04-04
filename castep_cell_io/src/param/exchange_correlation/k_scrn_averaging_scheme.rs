use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;

/// Determines the averaging scheme for estimating the Thomas-Fermi screening length.
///
/// Keyword type: String
///
/// Default: KScrnAveragingScheme::AveDen
///
/// Example:
/// K_SCRN_AVERAGING_SCHEME : SWA_DEN
///
/// Note: NLXC_K_SCRN_AVERAGING_SCHEME is the latest form; K_SCRN_AVERAGING_SCHEME is supported for backward compatibility.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum KScrnAveragingScheme {
    /// Use averaged charge density
    AveDen,
    /// Self-weighted average charge density
    SwaDen,
    /// Self-weighted average of screened-exchange energy density
    SwaEsx,
}

impl Default for KScrnAveragingScheme {
    fn default() -> Self {
        Self::AveDen
    }
}

impl FromCellValue for KScrnAveragingScheme {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "ave_den" => Ok(Self::AveDen),
            "swa_den" => Ok(Self::SwaDen),
            "swa_esx" => Ok(Self::SwaEsx),
            other => Err(Error::Message(format!("unknown KScrnAveragingScheme: {other}"))),
        }
    }
}

impl FromKeyValue for KScrnAveragingScheme {
    const KEY_NAME: &'static str = "K_SCRN_AVERAGING_SCHEME";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for KScrnAveragingScheme {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("K_SCRN_AVERAGING_SCHEME", self.to_cell_value())
    }
}

impl ToCellValue for KScrnAveragingScheme {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                KScrnAveragingScheme::AveDen => "AVE_DEN",
                KScrnAveragingScheme::SwaDen => "SWA_DEN",
                KScrnAveragingScheme::SwaEsx => "SWA_ESX",
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
        let val = CellValue::Str("ave_den");
        assert_eq!(KScrnAveragingScheme::from_cell_value(&val).unwrap(), KScrnAveragingScheme::AveDen);

        let val = CellValue::Str("AVE_DEN");
        assert_eq!(KScrnAveragingScheme::from_cell_value(&val).unwrap(), KScrnAveragingScheme::AveDen);

        let val = CellValue::Str("Ave_Den");
        assert_eq!(KScrnAveragingScheme::from_cell_value(&val).unwrap(), KScrnAveragingScheme::AveDen);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(
            KScrnAveragingScheme::from_cell_value(&CellValue::Str("swa_den")).unwrap(),
            KScrnAveragingScheme::SwaDen
        );
        assert_eq!(
            KScrnAveragingScheme::from_cell_value(&CellValue::Str("swa_esx")).unwrap(),
            KScrnAveragingScheme::SwaEsx
        );
    }

    #[test]
    fn test_invalid_variant() {
        let val = CellValue::Str("invalid");
        assert!(KScrnAveragingScheme::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(KScrnAveragingScheme::KEY_NAME, "K_SCRN_AVERAGING_SCHEME");
    }

    #[test]
    fn test_default() {
        assert_eq!(KScrnAveragingScheme::default(), KScrnAveragingScheme::AveDen);
    }

    #[test]
    fn test_to_cell_value() {
        assert_eq!(
            KScrnAveragingScheme::AveDen.to_cell_value(),
            CellValue::String("AVE_DEN".to_string())
        );
        assert_eq!(
            KScrnAveragingScheme::SwaDen.to_cell_value(),
            CellValue::String("SWA_DEN".to_string())
        );
        assert_eq!(
            KScrnAveragingScheme::SwaEsx.to_cell_value(),
            CellValue::String("SWA_ESX".to_string())
        );
    }

    #[test]
    fn test_to_cell() {
        let cell = KScrnAveragingScheme::AveDen.to_cell();
        match cell {
            Cell::KeyValue(key, _) => assert_eq!(key, "K_SCRN_AVERAGING_SCHEME"),
            _ => panic!("Expected KeyValue cell"),
        }
    }
}

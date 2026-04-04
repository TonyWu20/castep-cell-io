use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_string;

/// Specifies the semi-empirical dispersion correction scheme to use.
///
/// Keyword type: String
///
/// Default: SedcScheme::Ts
///
/// Example:
/// SEDC_SCHEME : G06
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum SedcScheme {
    /// Tkatchenko-Scheffler dispersion correction (default).
    #[default]
    Ts,
    /// Ortmann, Bechstedt, and Schmidt dispersion correction.
    Obs,
    /// Grimme 2006 dispersion correction.
    G06,
    /// Jurecka et al. dispersion correction.
    Jchs,
    /// Many-body dispersion correction.
    Mbd,
}

impl FromCellValue for SedcScheme {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_string(value)?.to_ascii_lowercase().as_str() {
            "ts" => Ok(Self::Ts),
            "obs" => Ok(Self::Obs),
            "g06" => Ok(Self::G06),
            "jchs" => Ok(Self::Jchs),
            "mbd" | "mbd*" => Ok(Self::Mbd),
            other => Err(Error::Message(format!("unknown SedcScheme: {other}"))),
        }
    }
}

impl FromKeyValue for SedcScheme {
    const KEY_NAME: &'static str = "SEDC_SCHEME";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for SedcScheme {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SEDC_SCHEME", self.to_cell_value())
    }
}

impl ToCellValue for SedcScheme {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                SedcScheme::Ts => "TS",
                SedcScheme::Obs => "OBS",
                SedcScheme::G06 => "G06",
                SedcScheme::Jchs => "JCHS",
                SedcScheme::Mbd => "MBD*",
            }
            .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_insensitive_parsing() {
        let val = CellValue::Str("ts");
        assert_eq!(SedcScheme::from_cell_value(&val).unwrap(), SedcScheme::Ts);

        let val = CellValue::Str("TS");
        assert_eq!(SedcScheme::from_cell_value(&val).unwrap(), SedcScheme::Ts);

        let val = CellValue::Str("Ts");
        assert_eq!(SedcScheme::from_cell_value(&val).unwrap(), SedcScheme::Ts);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(
            SedcScheme::from_cell_value(&CellValue::Str("ts")).unwrap(),
            SedcScheme::Ts
        );
        assert_eq!(
            SedcScheme::from_cell_value(&CellValue::Str("obs")).unwrap(),
            SedcScheme::Obs
        );
        assert_eq!(
            SedcScheme::from_cell_value(&CellValue::Str("g06")).unwrap(),
            SedcScheme::G06
        );
        assert_eq!(
            SedcScheme::from_cell_value(&CellValue::Str("jchs")).unwrap(),
            SedcScheme::Jchs
        );
        assert_eq!(
            SedcScheme::from_cell_value(&CellValue::Str("mbd")).unwrap(),
            SedcScheme::Mbd
        );
        assert_eq!(
            SedcScheme::from_cell_value(&CellValue::Str("mbd*")).unwrap(),
            SedcScheme::Mbd
        );
    }

    #[test]
    fn test_invalid_variant() {
        let val = CellValue::Str("invalid");
        assert!(SedcScheme::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(SedcScheme::KEY_NAME, "SEDC_SCHEME");
    }

    #[test]
    fn test_default() {
        assert_eq!(SedcScheme::default(), SedcScheme::Ts);
    }

    #[test]
    fn test_round_trip_serialization() {
        let variants = [
            SedcScheme::Ts,
            SedcScheme::Obs,
            SedcScheme::G06,
            SedcScheme::Jchs,
            SedcScheme::Mbd,
        ];

        for variant in &variants {
            let cell_value = variant.to_cell_value();
            let parsed = SedcScheme::from_cell_value(&cell_value).unwrap();
            assert_eq!(parsed, *variant);
        }
    }

    #[test]
    fn test_to_cell() {
        let scheme = SedcScheme::G06;
        let cell = scheme.to_cell();
        match cell {
            Cell::KeyValue(key, _) => assert_eq!(key, "SEDC_SCHEME"),
            _ => panic!("Expected KeyValue cell"),
        }
    }
}

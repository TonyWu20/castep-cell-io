// Note: This file handles both SECONDD_METHOD and the obsolete PHONON_METHOD.
// We'll define the enum and logic for SECONDD_METHOD primarily.
use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_string;
use serde::{Deserialize, Serialize};

/// Alias to `enum SeconddMethod`
pub type PhononMethod = SeconddMethod;

/// Specifies which calculation method will be used for phonons (SECONDD_METHOD).
/// Obsolete PHONON_METHOD is handled by aliasing the keyword name.
/// You might specify #[serde(alias = "PHONON_METHOD", alias="phonon_method")]
/// for backward compatibility
///
/// Keyword type: String
///
/// Default: SecondMethod::LinearResponse
///
/// Example:
/// SECONDD_METHOD : FINITEDISPLACEMENT
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "SECONDD_METHOD")]
// Handle the obsolete PHONON_METHOD name
// Note: Serde typically handles one primary name. Handling aliases for the *keyword itself*
// (not enum variants) usually requires custom deserializer logic or multiple struct fields.
// For simplicity here, we define it for SECONDD_METHOD. A wrapper struct or custom logic
// in the parent deserializer might be needed to fully support PHONON_METHOD alias.
#[derive(Default)]
pub enum SeconddMethod {
    /// Linear response method (or density functional perturbation theory)
    #[serde(alias = "linearresponse", alias = "LINEARRESPONSE")]
    #[serde(alias = "DFPT", alias = "dfpt")] // Alternative name
    #[default]
    LinearResponse,
    /// Finite displacement method
    #[serde(alias = "finitedisplacement", alias = "FINITEDISPLACEMENT")]
    FiniteDisplacement,
}

impl FromCellValue for SeconddMethod {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_string(value)?.to_ascii_lowercase().as_str() {
            "linearresponse" | "dfpt" => Ok(Self::LinearResponse),
            "finitedisplacement" => Ok(Self::FiniteDisplacement),
            other => Err(Error::Message(format!("unknown SeconddMethod: {other}"))),
        }
    }
}

impl FromKeyValue for SeconddMethod {
    const KEY_NAME: &'static str = "SECONDD_METHOD";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}


impl ToCell for SeconddMethod {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("SECONDD_METHOD", self.to_cell_value())
    }
}

impl ToCellValue for SeconddMethod {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                SeconddMethod::LinearResponse => "LINEARRESPONSE",
                SeconddMethod::FiniteDisplacement => "FINITEDISPLACEMENT",
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
        assert_eq!(SeconddMethod::from_cell_value(&CellValue::Str("linearresponse")).unwrap(), SeconddMethod::LinearResponse);
        assert_eq!(SeconddMethod::from_cell_value(&CellValue::Str("LINEARRESPONSE")).unwrap(), SeconddMethod::LinearResponse);
        assert_eq!(SeconddMethod::from_cell_value(&CellValue::Str("dfpt")).unwrap(), SeconddMethod::LinearResponse);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(SeconddMethod::from_cell_value(&CellValue::Str("finitedisplacement")).unwrap(), SeconddMethod::FiniteDisplacement);
    }

    #[test]
    fn test_invalid() {
        assert!(SeconddMethod::from_cell_value(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(SeconddMethod::KEY_NAME, "SECONDD_METHOD");
    }

    #[test]
    fn test_round_trip_serialization() {
        let method = SeconddMethod::LinearResponse;
        let cell_value = method.to_cell_value();
        let parsed = SeconddMethod::from_cell_value(&cell_value).unwrap();
        assert_eq!(parsed, method);

        let method = SeconddMethod::FiniteDisplacement;
        let cell_value = method.to_cell_value();
        let parsed = SeconddMethod::from_cell_value(&cell_value).unwrap();
        assert_eq!(parsed, method);
    }

    #[test]
    fn test_default_is_linear_response() {
        assert_eq!(SeconddMethod::default(), SeconddMethod::LinearResponse);
    }
}


// Note: This file handles both SECONDD_METHOD and the obsolete PHONON_METHOD.
// We'll define the enum and logic for SECONDD_METHOD primarily.
use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;
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
pub enum SeconddMethod {
    /// Linear response method (or density functional perturbation theory)
    #[serde(alias = "linearresponse", alias = "LINEARRESPONSE")]
    #[serde(alias = "DFPT", alias = "dfpt")] // Alternative name
    LinearResponse,
    /// Finite displacement method
    #[serde(alias = "finitedisplacement", alias = "FINITEDISPLACEMENT")]
    FiniteDisplacement,
}

impl FromCellValue for SeconddMethod {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
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

impl Default for SeconddMethod {
    fn default() -> Self {
        Self::LinearResponse // Default is LINEARRESPONSE
    }
}

impl ToCell for SeconddMethod {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SECONDD_METHOD", self.to_cell_value())
    }
}

impl ToCellValue for SeconddMethod {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                SeconddMethod::LinearResponse => "LINEARRESPONSE", // Or "DFPT"
                SeconddMethod::FiniteDisplacement => "FINITEDISPLACEMENT",
            }
            .to_string(),
        )
    }
}



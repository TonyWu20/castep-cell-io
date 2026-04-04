use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_fmt::query::value_as_str;

/// Determines the electronic minimization method used in the self-consistent calculation.
///
/// Keyword type: String
///
/// Default: MetalsMethod::Edft (or MetalsMethod::Dm if FIX_OCCUPANCY is FALSE)
///
/// Example:
/// METALS_METHOD : dm
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MetalsMethod {
    /// System treated by density mixing
    Dm,
    /// System treated by ensemble density functional method
    Edft,
    /// Currently not used
    None,
}

impl Default for MetalsMethod {
    fn default() -> Self {
        Self::Edft
    }
}

impl FromKeyValue for MetalsMethod {
    const KEY_NAME: &'static str = "METALS_METHOD";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "dm" => Ok(Self::Dm),
            "edft" => Ok(Self::Edft),
            "none" => Ok(Self::None),
            other => Err(castep_cell_fmt::Error::Message(format!("unknown MetalsMethod: {other}"))),
        }
    }
}

impl ToCell for MetalsMethod {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("METALS_METHOD", self.to_cell_value())
    }
}

impl ToCellValue for MetalsMethod {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MetalsMethod::Dm => "DM",
                MetalsMethod::Edft => "EDFT",
                MetalsMethod::None => "NONE",
            }
            .to_string(),
        )
    }
}



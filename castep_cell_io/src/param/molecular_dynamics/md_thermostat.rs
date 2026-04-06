use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;

/// Determines the thermostat used for a molecular dynamics calculation (NVT ensemble).
///
/// Keyword type: String
///
/// Default: MdThermostat::NoseHoover
///
/// Example:
/// MD_THERMOSTAT : Langevin
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[derive(Default)]
pub enum MdThermostat {
    /// Nosé-Hoover thermostat
    #[default]
    NoseHoover,
    /// Langevin thermostat
    Langevin,
}


impl FromCellValue for MdThermostat {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "nosé-hoover" | "nose-hoover" => Ok(Self::NoseHoover),
            "langevin" => Ok(Self::Langevin),
            other => Err(Error::Message(format!("unknown MdThermostat: {other}"))),
        }
    }
}

impl FromKeyValue for MdThermostat {
    const KEY_NAME: &'static str = "MD_THERMOSTAT";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdThermostat {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("MD_THERMOSTAT", self.to_cell_value())
    }
}

impl ToCellValue for MdThermostat {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                MdThermostat::NoseHoover => "Nosé-Hoover",
                MdThermostat::Langevin => "Langevin",
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
        assert_eq!(MdThermostat::from_cell_value(&CellValue::Str("nose-hoover")).unwrap(), MdThermostat::NoseHoover);
        assert_eq!(MdThermostat::from_cell_value(&CellValue::Str("NOSE-HOOVER")).unwrap(), MdThermostat::NoseHoover);
        assert_eq!(MdThermostat::from_cell_value(&CellValue::Str("langevin")).unwrap(), MdThermostat::Langevin);
    }

    #[test]
    fn test_unicode_variant() {
        assert_eq!(MdThermostat::from_cell_value(&CellValue::Str("nosé-hoover")).unwrap(), MdThermostat::NoseHoover);
    }

    #[test]
    fn test_invalid() {
        assert!(MdThermostat::from_cell_value(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(MdThermostat::KEY_NAME, "MD_THERMOSTAT");
    }
}


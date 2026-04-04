use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;

/// Determines the barostat method used for molecular dynamics with variable cell volume.
///
/// Keyword type: String
///
/// Default: MdBarostat::AndersenHoover
///
/// Example:
/// MD_BAROSTAT : Andersen-Hoover
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MdBarostat {
    /// Andersen-Hoover barostat
    AndersenHoover,
    /// Parrinello-Rahman barostat
    ParrinelloRahman,
}

impl Default for MdBarostat {
    fn default() -> Self {
        Self::AndersenHoover // Default is Andersen-Hoover
    }
}

impl FromCellValue for MdBarostat {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "andersen-hoover" => Ok(Self::AndersenHoover),
            "parrinello-rahman" => Ok(Self::ParrinelloRahman),
            other => Err(Error::Message(format!("unknown MdBarostat: {other}"))),
        }
    }
}

impl FromKeyValue for MdBarostat {
    const KEY_NAME: &'static str = "MD_BAROSTAT";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for MdBarostat {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("MD_BAROSTAT", self.to_cell_value())
    }
}

impl ToCellValue for MdBarostat {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                MdBarostat::AndersenHoover => "Andersen-Hoover",
                MdBarostat::ParrinelloRahman => "Parrinello-Rahman",
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
        assert_eq!(MdBarostat::from_cell_value(&CellValue::Str("andersen-hoover")).unwrap(), MdBarostat::AndersenHoover);
        assert_eq!(MdBarostat::from_cell_value(&CellValue::Str("ANDERSEN-HOOVER")).unwrap(), MdBarostat::AndersenHoover);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(MdBarostat::from_cell_value(&CellValue::Str("parrinello-rahman")).unwrap(), MdBarostat::ParrinelloRahman);
    }

    #[test]
    fn test_invalid() {
        assert!(MdBarostat::from_cell_value(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(MdBarostat::KEY_NAME, "MD_BAROSTAT");
    }
}


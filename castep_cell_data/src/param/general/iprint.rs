use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::query::value_as_u32;
use castep_cell_io::Error;

/// Controls the level of verbosity of the output.
///
/// Keyword type: Integer
///
/// Default: Iprint::Level1
///
/// Example:
/// IPRINT : 1
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Iprint {
    /// Minimal output (0)
    Level0,
    /// Standard output (1)
    #[default]
    Level1,
    /// Detailed output (2)
    Level2,
    /// Full debugging output (3)
    Level3,
}

impl FromCellValue for Iprint {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_u32(value)? {
            0 => Ok(Self::Level0),
            1 => Ok(Self::Level1),
            2 => Ok(Self::Level2),
            3 => Ok(Self::Level3),
            _ => Err(Error::Message(
                "value of `iprint` exceeds maximum of 3".to_string(),
            )),
        }
    }
}

impl FromKeyValue for Iprint {
    const KEY_NAME: &'static str = "IPRINT";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for Iprint {
    fn to_cell(&self) -> Cell {
        let value = match self {
            Iprint::Level0 => 0,
            Iprint::Level1 => 1,
            Iprint::Level2 => 2,
            Iprint::Level3 => 3,
        };
        Cell::KeyValue("IPRINT", CellValue::Int(value))
    }
}

impl ToCellValue for Iprint {
    fn to_cell_value(&self) -> CellValue {
        let value = match self {
            Iprint::Level0 => 0,
            Iprint::Level1 => 1,
            Iprint::Level2 => 2,
            Iprint::Level3 => 3,
        };
        CellValue::Int(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_io::CellValue;

    #[test]
    fn test_all_levels() {
        assert_eq!(Iprint::from_cell_value(&CellValue::UInt(0)).unwrap(), Iprint::Level0);
        assert_eq!(Iprint::from_cell_value(&CellValue::UInt(1)).unwrap(), Iprint::Level1);
        assert_eq!(Iprint::from_cell_value(&CellValue::UInt(2)).unwrap(), Iprint::Level2);
        assert_eq!(Iprint::from_cell_value(&CellValue::UInt(3)).unwrap(), Iprint::Level3);
    }

    #[test]
    fn test_invalid() {
        assert!(Iprint::from_cell_value(&CellValue::UInt(4)).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(Iprint::KEY_NAME, "IPRINT");
    }
}


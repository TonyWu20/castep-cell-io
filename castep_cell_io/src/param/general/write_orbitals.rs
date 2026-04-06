use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, CResult};
use castep_cell_fmt::parse::FromKeyValue;
use castep_cell_fmt::query::value_as_bool;

/// Specifies whether or not to write seedname.orbitals binary file in a band structure calculation.
///
/// Keyword type: Logical
///
/// Default: FALSE unless TASK is BANDSTRUCTURE.
///
/// Example:
/// WRITE_ORBITALS : TRUE
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct WriteOrbitals(pub bool);

// Note: The default logic ("FALSE unless TASK is BANDSTRUCTURE") is context-dependent
// and cannot be easily encoded in the struct itself without access to the TASK value.
// The `Default` implementation here provides a base default of `false`.

impl FromKeyValue for WriteOrbitals {
    const KEY_NAME: &'static str = "WRITE_ORBITALS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl ToCell for WriteOrbitals {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("WRITE_ORBITALS", CellValue::Bool(self.0))
    }
}

impl ToCellValue for WriteOrbitals {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Bool(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value_true() {
        let val = CellValue::Bool(true);
        assert_eq!(WriteOrbitals::from_cell_value_kv(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(WriteOrbitals::from_cell_value_kv(&val).unwrap().0, false);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(WriteOrbitals::KEY_NAME, "WRITE_ORBITALS");
    }
}


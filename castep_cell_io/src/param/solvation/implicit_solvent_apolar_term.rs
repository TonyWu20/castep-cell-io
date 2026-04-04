use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_fmt::query::value_as_bool;

/// Specifies whether to include the apolar contribution to the solvation free energy.
///
/// Keyword type: Logical
///
/// Default: false
///
/// Example:
/// IMPLICIT_SOLVENT_APOLAR_TERM : TRUE
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImplicitSolventApolarTerm(pub bool);

impl FromKeyValue for ImplicitSolventApolarTerm {
    const KEY_NAME: &'static str = "IMPLICIT_SOLVENT_APOLAR_TERM";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl ToCell for ImplicitSolventApolarTerm {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("IMPLICIT_SOLVENT_APOLAR_TERM", CellValue::Bool(self.0))
    }
}

impl ToCellValue for ImplicitSolventApolarTerm {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Bool(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cell_value_true() {
        let val = CellValue::Bool(true);
        assert_eq!(ImplicitSolventApolarTerm::from_cell_value_kv(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(ImplicitSolventApolarTerm::from_cell_value_kv(&val).unwrap().0, false);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(ImplicitSolventApolarTerm::KEY_NAME, "IMPLICIT_SOLVENT_APOLAR_TERM");
    }

    #[test]
    fn test_default() {
        let default = ImplicitSolventApolarTerm::default();
        assert_eq!(default.0, false);
    }

    #[test]
    fn test_round_trip() {
        for value in [true, false] {
            let original = ImplicitSolventApolarTerm(value);
            let cell_value = original.to_cell_value();
            let parsed = ImplicitSolventApolarTerm::from_cell_value_kv(&cell_value).unwrap();
            assert_eq!(parsed.0, original.0);
        }
    }
}

use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_fmt::query::value_as_bool;

/// Specifies whether to use smeared ionic charges in the solvation model.
///
/// Keyword type: Logical
///
/// Default: false
///
/// Example:
/// USE_SMEARED_IONS : TRUE
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct UseSmearediIons(pub bool);

impl FromKeyValue for UseSmearediIons {
    const KEY_NAME: &'static str = "USE_SMEARED_IONS";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(value_as_bool(value)?))
    }
}

impl ToCell for UseSmearediIons {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("USE_SMEARED_IONS", CellValue::Bool(self.0))
    }
}

impl ToCellValue for UseSmearediIons {
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
        assert_eq!(UseSmearediIons::from_cell_value_kv(&val).unwrap().0, true);
    }

    #[test]
    fn test_from_cell_value_false() {
        let val = CellValue::Bool(false);
        assert_eq!(UseSmearediIons::from_cell_value_kv(&val).unwrap().0, false);
    }

    #[test]
    fn test_key_name() {
        assert_eq!(UseSmearediIons::KEY_NAME, "USE_SMEARED_IONS");
    }

    #[test]
    fn test_default() {
        let default = UseSmearediIons::default();
        assert_eq!(default.0, false);
    }

    #[test]
    fn test_round_trip() {
        for value in [true, false] {
            let original = UseSmearediIons(value);
            let cell_value = original.to_cell_value();
            let parsed = UseSmearediIons::from_cell_value_kv(&cell_value).unwrap();
            assert_eq!(parsed.0, original.0);
        }
    }
}

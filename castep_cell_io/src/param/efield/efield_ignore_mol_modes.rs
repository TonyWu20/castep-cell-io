use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;

/// Specifies how many of the lowest lying modes to ignore for ionic permittivity/polarizability.
///
/// Keyword type: String
///
/// Default: EfieldIgnoreMolModes::Crystal
///
/// Example:
/// EFIELD_IGNORE_MOL_MODES : Molecule
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EfieldIgnoreMolModes {
    /// Ignore the three lowest lying modes
    Crystal,
    /// Ignore the six lowest lying modes
    Molecule,
    /// Ignore the five lowest lying modes
    LinearMolecule,
}

impl Default for EfieldIgnoreMolModes {
    fn default() -> Self {
        Self::Crystal
    }
}

impl FromCellValue for EfieldIgnoreMolModes {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "crystal" => Ok(Self::Crystal),
            "molecule" => Ok(Self::Molecule),
            "linear_molecule" => Ok(Self::LinearMolecule),
            other => Err(Error::Message(format!("unknown EfieldIgnoreMolModes: {other}"))),
        }
    }
}

impl FromKeyValue for EfieldIgnoreMolModes {
    const KEY_NAME: &'static str = "EFIELD_IGNORE_MOL_MODES";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for EfieldIgnoreMolModes {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("EFIELD_IGNORE_MOL_MODES", self.to_cell_value())
    }
}

impl ToCellValue for EfieldIgnoreMolModes {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                EfieldIgnoreMolModes::Crystal => "Crystal",
                EfieldIgnoreMolModes::Molecule => "Molecule",
                EfieldIgnoreMolModes::LinearMolecule => "Linear_molecule",
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
        assert_eq!(EfieldIgnoreMolModes::from_cell_value(&CellValue::Str("crystal")).unwrap(), EfieldIgnoreMolModes::Crystal);
        assert_eq!(EfieldIgnoreMolModes::from_cell_value(&CellValue::Str("CRYSTAL")).unwrap(), EfieldIgnoreMolModes::Crystal);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(EfieldIgnoreMolModes::from_cell_value(&CellValue::Str("molecule")).unwrap(), EfieldIgnoreMolModes::Molecule);
        assert_eq!(EfieldIgnoreMolModes::from_cell_value(&CellValue::Str("linear_molecule")).unwrap(), EfieldIgnoreMolModes::LinearMolecule);
    }

    #[test]
    fn test_invalid() {
        assert!(EfieldIgnoreMolModes::from_cell_value(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(EfieldIgnoreMolModes::KEY_NAME, "EFIELD_IGNORE_MOL_MODES");
    }
}

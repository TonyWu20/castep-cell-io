use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;

/// Specifies how many of the lowest lying modes to ignore for ionic permittivity/polarizability.
///
/// Keyword type: String
///
/// Default: EfieldIgnoreMolecModes::Crystal
///
/// Example:
/// EFIELD_IGNORE_MOLEC_MODES : Molecule
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[derive(Default)]
pub enum EfieldIgnoreMolecModes {
    /// Ignore the three lowest lying modes
    #[default]
    Crystal,
    /// Ignore the six lowest lying modes
    Molecule,
    /// Ignore the five lowest lying modes
    LinearMolecule,
}


impl FromCellValue for EfieldIgnoreMolecModes {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "crystal" => Ok(Self::Crystal),
            "molecule" => Ok(Self::Molecule),
            "linear_molecule" => Ok(Self::LinearMolecule),
            other => Err(Error::Message(format!("unknown EfieldIgnoreMolecModes: {other}"))),
        }
    }
}

impl FromKeyValue for EfieldIgnoreMolecModes {
    const KEY_NAME: &'static str = "EFIELD_IGNORE_MOLEC_MODES";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for EfieldIgnoreMolecModes {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("EFIELD_IGNORE_MOLEC_MODES", self.to_cell_value())
    }
}

impl ToCellValue for EfieldIgnoreMolecModes {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                EfieldIgnoreMolecModes::Crystal => "Crystal",
                EfieldIgnoreMolecModes::Molecule => "Molecule",
                EfieldIgnoreMolecModes::LinearMolecule => "Linear_molecule",
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
        assert_eq!(EfieldIgnoreMolecModes::from_cell_value(&CellValue::Str("crystal")).unwrap(), EfieldIgnoreMolecModes::Crystal);
        assert_eq!(EfieldIgnoreMolecModes::from_cell_value(&CellValue::Str("CRYSTAL")).unwrap(), EfieldIgnoreMolecModes::Crystal);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(EfieldIgnoreMolecModes::from_cell_value(&CellValue::Str("molecule")).unwrap(), EfieldIgnoreMolecModes::Molecule);
        assert_eq!(EfieldIgnoreMolecModes::from_cell_value(&CellValue::Str("linear_molecule")).unwrap(), EfieldIgnoreMolecModes::LinearMolecule);
    }

    #[test]
    fn test_invalid() {
        assert!(EfieldIgnoreMolecModes::from_cell_value(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(EfieldIgnoreMolecModes::KEY_NAME, "EFIELD_IGNORE_MOLEC_MODES");
    }
}

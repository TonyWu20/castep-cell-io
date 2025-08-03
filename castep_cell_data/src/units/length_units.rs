use castep_cell_serde::{CellValue, ToCellValue};
use serde::{Deserialize, Serialize};
#[derive(
    Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default,
)]
/// This keyword specifies the units in which lengths will be reported.
///
/// Keyword type: String
///
/// Default: ang
///
/// Example:
/// LENGTH_UNIT : bohr
pub enum LengthUnit {
    #[serde(alias = "BOHR", alias = "bohr")]
    Bohr,
    #[serde(alias = "a0", alias = "A0")]
    BohrA0,
    #[serde(alias = "M", alias = "m")]
    Meter,
    #[serde(alias = "CM", alias = "cm")]
    Centimeter,
    #[serde(alias = "NM", alias = "nm")]
    Nanometer,
    #[default]
    #[serde(alias = "ANG", alias = "ang")]
    Ang,
}

impl ToCellValue for LengthUnit {
    fn to_cell_value(&self) -> CellValue {
        match self {
            LengthUnit::Bohr => CellValue::String("bohr".into()),
            LengthUnit::BohrA0 => CellValue::String("a0".into()),
            LengthUnit::Meter => CellValue::String("m".into()),
            LengthUnit::Centimeter => CellValue::String("cm".into()),
            LengthUnit::Nanometer => CellValue::String("nm".into()),
            LengthUnit::Ang => CellValue::String("ang".into()),
        }
    }
}

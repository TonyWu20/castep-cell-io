use castep_cell_serde::{CellValue, ToCellValue};
use serde::{Deserialize, Serialize};
#[derive(
    Debug, Clone, Copy, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default,
)]
#[serde(rename_all = "lowercase")]
pub enum LengthUnit {
    Bohr,
    #[serde(alias = "a0")]
    BohrA0,
    #[serde(rename = "m")]
    Meter,
    #[serde(rename = "cm")]
    Centimeter,
    #[serde(rename = "nm")]
    Nanometer,
    #[default]
    #[serde(rename = "ang")]
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

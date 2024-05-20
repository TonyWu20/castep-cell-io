use std::fmt::Display;

#[derive(Debug, Default, Clone, Copy)]
pub enum LengthUnit {
    Bohr,
    Meter,
    Centimeter,
    Nanometer,
    #[default]
    Ang,
}

impl Display for LengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthUnit::Bohr => f.write_str("bohr"),
            LengthUnit::Meter => f.write_str("m"),
            LengthUnit::Centimeter => f.write_str("cm"),
            LengthUnit::Nanometer => f.write_str("nm"),
            LengthUnit::Ang => f.write_str("ang"),
        }
    }
}

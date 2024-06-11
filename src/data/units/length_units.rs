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

#[derive(Debug, Default, Clone, Copy)]
pub enum InvLengthUnit {
    Bohr,
    Meter,
    Nanometer,
    #[default]
    Ang,
}

impl Display for InvLengthUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvLengthUnit::Bohr => f.write_str("1/"),
            InvLengthUnit::Meter => f.write_str("1/m"),
            InvLengthUnit::Nanometer => f.write_str("1/nm"),
            InvLengthUnit::Ang => f.write_str("1/ang"),
        }
    }
}

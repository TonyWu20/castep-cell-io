#[derive(Debug, Default, Clone, Copy)]
pub enum LengthUnit {
    Bohr,
    Meter,
    Centimeter,
    Nanometer,
    #[default]
    Ang,
}

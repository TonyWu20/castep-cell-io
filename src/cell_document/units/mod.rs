use crate::CellParseError;

pub mod charge_units;
pub mod efield_units;
pub mod force_units;
pub mod length_units;
pub mod pressure_units;

#[derive(Debug, Clone, Copy)]
pub struct Degrees(f64);

impl Degrees {
    pub fn new(degrees: f64) -> Self {
        Degrees(degrees)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Radians(f64);

impl Radians {
    pub fn new(radians: f64) -> Self {
        Radians(radians)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

pub trait ParsableUnit<Output = Self> {
    fn parse_from_str(input: &str) -> Result<Output, CellParseError>;
}

use std::fmt::Display;

use crate::formatting::BlockDisplay;

use super::units::{efield_units::EFieldUnit, pressure_units::PressureUnits};

#[derive(Debug, Clone, Copy, Default)]
pub struct ExtEFieldBlock {
    unit: EFieldUnit,
    x: f64,
    y: f64,
    z: f64,
}

impl BlockDisplay for ExtEFieldBlock {
    fn block_tag(&self) -> String {
        "EXTERNAL_EFIELD".to_string()
    }

    fn entries(&self) -> String {
        let unit_line = if self.unit == EFieldUnit::default() {
            String::new()
        } else {
            format!("{}\n", self.unit)
        };
        format!(
            "{unit_line}{:16.10}{:16.10}{:16.10}",
            self.x, self.y, self.z
        )
    }
}

impl Display for ExtEFieldBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ExtPressureBlock {
    unit: PressureUnits,
    components: [f64; 6],
}

impl BlockDisplay for ExtPressureBlock {
    fn block_tag(&self) -> String {
        "EXTERNAL_PRESSURE".to_string()
    }

    fn entries(&self) -> String {
        let unit_line = if self.unit == PressureUnits::default() {
            String::new()
        } else {
            format!("{}\n", self.unit)
        };
        let [rxx, rxy, rxz, ryy, ryz, rzz] = self.components;
        let pressures = format!(
            r#"{:16.10}{:16.10}{:16.10}
                {:16.10}{:16.10}
                                {:16.10}"#,
            rxx, rxy, rxz, ryy, ryz, rzz
        );
        [unit_line, pressures].concat()
    }
}

impl Display for ExtPressureBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
    }
}

#[cfg(test)]
mod test {
    use crate::data::external_fields::ExtPressureBlock;

    use super::ExtEFieldBlock;

    #[test]
    fn ext_efield() {
        let item = ExtEFieldBlock::default();
        println!("{item}");
        let pressure = ExtPressureBlock::default();
        println!("{pressure}");
    }
}

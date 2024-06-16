use std::fmt::Display;

use crate::cell_document::params::OptionDisplay;

#[derive(Debug, Clone, Copy)]
pub enum MetalsMethod {
    Dm(DensityMixing),
    Edft(Edft),
}

impl OptionDisplay for MetalsMethod {
    fn tag(&self) -> String {
        "metals_method".to_string()
    }

    fn value(&self) -> String {
        match self {
            MetalsMethod::Dm(_) => "dm".to_string(),
            MetalsMethod::Edft(_) => "edft".to_string(),
        }
    }
}

impl Display for MetalsMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            MetalsMethod::Dm(dm) => [self.value(), format!("{dm}")].join("\n"),
            MetalsMethod::Edft(edft) => [self.value(), format!("{edft}")].join("\n"),
        };
        write!(f, "{content}")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DensityMixing {
    scheme: DensityMixingScheme,
    mix_charge_amp: f64,
    mix_spin_amp: f64,
    mix_charge_gmax: f64,
    mix_spin_gmax: f64,
    mix_history_length: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Edft {
    num_occ_cycles: u32,
}

impl Edft {
    pub fn new(num_occ_cycles: u32) -> Self {
        Self { num_occ_cycles }
    }
}

impl Display for Edft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "num_occ_cycles : {}", self.num_occ_cycles)
    }
}

impl Default for Edft {
    fn default() -> Self {
        Self { num_occ_cycles: 6 }
    }
}

impl DensityMixing {
    pub fn new(
        scheme: DensityMixingScheme,
        mix_charge_amp: f64,
        mix_spin_amp: f64,
        mix_charge_gmax: f64,
        mix_spin_gmax: f64,
        mix_history_length: u32,
    ) -> Self {
        Self {
            scheme,
            mix_charge_amp,
            mix_spin_amp,
            mix_charge_gmax,
            mix_spin_gmax,
            mix_history_length,
        }
    }
}

impl Default for DensityMixing {
    fn default() -> Self {
        Self {
            scheme: DensityMixingScheme::default(),
            mix_charge_amp: 0.5,
            mix_spin_amp: 2.0,
            mix_charge_gmax: 1.5,
            mix_spin_gmax: 1.5,
            mix_history_length: 20,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum DensityMixingScheme {
    Linear,
    Kerker,
    Broyden,
    #[default]
    Pulay,
}

impl OptionDisplay for DensityMixingScheme {
    fn tag(&self) -> String {
        "mixing_scheme".to_string()
    }

    fn value(&self) -> String {
        match self {
            DensityMixingScheme::Linear => "Linear".to_string(),
            DensityMixingScheme::Kerker => "Kerker".to_string(),
            DensityMixingScheme::Broyden => "Broyden".to_string(),
            DensityMixingScheme::Pulay => "Pulay".to_string(),
        }
    }
}

impl Display for DensityMixingScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.output())
    }
}

impl Display for DensityMixing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scheme = format!("{}", self.scheme);
        let params = match self.scheme {
            DensityMixingScheme::Linear => [
                format!("mix_charge_amp :{:>26.15}", self.mix_charge_amp),
                format!("mix_spin_amp : {:>26.15}", self.mix_spin_amp),
            ]
            .join("\n"),
            DensityMixingScheme::Kerker => [
                format!("mix_charge_amp :{:>26.15}", self.mix_charge_amp),
                format!("mix_spin_amp :{:>26.15}", self.mix_spin_amp),
                format!("mix_charge_gmax :{:>26.15}", self.mix_charge_gmax),
                format!("mix_spin_gmax :{:>26.15}", self.mix_spin_gmax),
            ]
            .join("\n"),

            DensityMixingScheme::Broyden | DensityMixingScheme::Pulay => [
                format!("mix_charge_amp :{:>26.15}", self.mix_charge_amp),
                format!("mix_spin_amp :{:>26.15}", self.mix_spin_amp),
                format!("mix_charge_gmax :{:>26.15}", self.mix_charge_gmax),
                format!("mix_spin_gmax :{:>26.15}", self.mix_spin_gmax),
                format!("mix_history_length :{:>10}", self.mix_history_length),
            ]
            .join("\n"),
        };
        write!(f, "{}", [scheme, params].join("\n"))
    }
}

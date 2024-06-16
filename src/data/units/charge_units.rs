use std::fmt::Display;

#[derive(Debug, Clone, Copy, Default)]
pub enum ChargeUnits {
    #[default]
    ElementaryCharge,
    Coulomb,
}

impl Display for ChargeUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChargeUnits::ElementaryCharge => f.write_str("e"),
            ChargeUnits::Coulomb => f.write_str("c"),
        }
    }
}

use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::FromCellValue;
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Represents the unit for the per-species nuclear gyromagnetic ratio
/// (the `SPECIES_GAMMA` block).
///
/// Keyword type: String
///
/// Default: rad/s/T (`radsectesla`) — CASTEP 6.11 default
/// (`Fundamental/cell.f90`, `units_gamma='radsectesla'`).
#[derive(
    Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum GammaUnit {
    /// Radians per second per tesla — the CASTEP default.
    #[default]
    #[serde(alias = "RADSECTESLA", alias = "radsectesla")]
    RadPerSecTesla,
    /// Megahertz per tesla.
    #[serde(alias = "MHZTESLA", alias = "mhztesla")]
    MegaHertzPerTesla,
}

impl FromCellValue for GammaUnit {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "radsectesla" => Ok(Self::RadPerSecTesla),
            "mhztesla" => Ok(Self::MegaHertzPerTesla),
            other => Err(Error::Message(format!(
                "unknown GammaUnit: {other} (expected radsectesla or mhztesla)"
            ))),
        }
    }
}

impl ToCellValue for GammaUnit {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                GammaUnit::RadPerSecTesla => "radsectesla",
                GammaUnit::MegaHertzPerTesla => "mhztesla",
            }
            .to_string(),
        )
    }
}

impl ToCell for GammaUnit {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("GAMMA_UNIT", self.to_cell_value())
    }
}

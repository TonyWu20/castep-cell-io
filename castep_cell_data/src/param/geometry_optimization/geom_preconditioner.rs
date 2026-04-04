use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;
use serde::{Deserialize, Serialize};

/// Selects the preconditioner used for LBFGS geometry optimization.
///
/// Keyword type: String
///
/// Default: GeomPreconditioner::Id
///
/// Example:
/// GEOM_PRECONDITIONER : EXP
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "GEOM_PRECONDITIONER")]
pub enum GeomPreconditioner {
    /// Identity; LBFGS is used without a preconditioner
    #[serde(alias = "id", alias = "ID")]
    Id,
    /// Exponential preconditioner
    #[serde(alias = "exp", alias = "EXP")]
    Exp,
    /// Forcefield based preconditioner using the scheme of Lindh et al. (1995)
    #[serde(alias = "ff", alias = "FF")]
    Ff,
}

impl FromCellValue for GeomPreconditioner {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "id" => Ok(Self::Id),
            "exp" => Ok(Self::Exp),
            "ff" => Ok(Self::Ff),
            other => Err(Error::Message(format!("unknown GeomPreconditioner: {other}"))),
        }
    }
}

impl FromKeyValue for GeomPreconditioner {
    const KEY_NAME: &'static str = "GEOM_PRECONDITIONER";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl Default for GeomPreconditioner {
    fn default() -> Self {
        Self::Id // Default is ID
    }
}

impl ToCell for GeomPreconditioner {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("GEOM_PRECONDITIONER", self.to_cell_value())
    }
}

impl ToCellValue for GeomPreconditioner {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                GeomPreconditioner::Id => "ID",
                GeomPreconditioner::Exp => "EXP",
                GeomPreconditioner::Ff => "FF",
            }
            .to_string(),
        )
    }
}



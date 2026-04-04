use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;

/// Controls which functional is used to calculate the exchange-correlation potential.
///
/// Keyword type: String
///
/// Default: XcFunctional::Lda
///
/// Example:
/// XC_FUNCTIONAL : PW91
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum XcFunctional {
    /// Local Density Approximation
    #[default]
    Lda,
    /// Perdew Wang '91 GGA
    Pw91,
    /// Perdew Burke Ernzerhof
    Pbe,
    /// Revised Perdew Burke Ernzerhof
    Rpbe,
    /// Wu-Cohen
    Wc,
    /// PBEsol, PBE functional for solids
    Pbesol,
    /// Becke Lee Young Parr
    Blyp,
    /// Exact exchange, no correlation
    Hf,
    /// Exact exchange, LDA correlation
    HfLda,
    /// Screened exchange, no correlation
    SX,
    /// Screened exchange, LDA correlation
    SXlda,
    /// PBE0 hybrid functional
    Pbe0,
    /// B3LYP hybrid functional
    B3lyp,
    /// HSE03 hybrid functional
    Hse03,
    /// HSE06 hybrid functional
    Hse06,
    /// Regularized SCAN meta-GGA functional
    Rscan,
}

impl FromCellValue for XcFunctional {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "lda" => Ok(Self::Lda),
            "pw91" => Ok(Self::Pw91),
            "pbe" => Ok(Self::Pbe),
            "rpbe" => Ok(Self::Rpbe),
            "wc" => Ok(Self::Wc),
            "pbesol" => Ok(Self::Pbesol),
            "blyp" => Ok(Self::Blyp),
            "hf" => Ok(Self::Hf),
            "hf-lda" => Ok(Self::HfLda),
            "sx" => Ok(Self::SX),
            "sx-lda" => Ok(Self::SXlda),
            "pbe0" => Ok(Self::Pbe0),
            "b3lyp" => Ok(Self::B3lyp),
            "hse03" => Ok(Self::Hse03),
            "hse06" => Ok(Self::Hse06),
            "rscan" => Ok(Self::Rscan),
            other => Err(Error::Message(format!("unknown XcFunctional: {other}"))),
        }
    }
}

impl FromKeyValue for XcFunctional {
    const KEY_NAME: &'static str = "XC_FUNCTIONAL";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for XcFunctional {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("XC_FUNCTIONAL", self.to_cell_value())
    }
}

impl ToCellValue for XcFunctional {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                XcFunctional::Lda => "LDA",
                XcFunctional::Pw91 => "PW91",
                XcFunctional::Pbe => "PBE",
                XcFunctional::Rpbe => "RPBE",
                XcFunctional::Wc => "WC",
                XcFunctional::Pbesol => "PBESOL",
                XcFunctional::Blyp => "BLYP",
                XcFunctional::Hf => "HF",
                XcFunctional::HfLda => "HF-LDA",
                XcFunctional::SX => "SX",
                XcFunctional::SXlda => "SX-LDA",
                XcFunctional::Pbe0 => "PBE0",
                XcFunctional::B3lyp => "B3LYP",
                XcFunctional::Hse03 => "HSE03",
                XcFunctional::Hse06 => "HSE06",
                XcFunctional::Rscan => "RSCAN",
            }
            .to_string(),
        )
    }
}


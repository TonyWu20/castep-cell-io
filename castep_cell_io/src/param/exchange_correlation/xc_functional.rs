use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_str;

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
    /// Local Density Approximation, exchange only
    LdaX,
    /// Local Density Approximation, correlation only
    LdaC,
    /// Perdew Wang '91 GGA
    Pw91,
    /// Perdew Burke Ernzerhof
    Pbe,
    /// Perdew Burke Ernzerhof GGA functional, exchange only
    PbeX,
    /// Perdew Burke Ernzerhof GGA functional, correlation only
    PbeC,
    /// Revised Perdew Burke Ernzerhof
    Rpbe,
    /// Wu-Cohen
    Wc,
    /// PBEsol, PBE functional for solids
    Pbesol,
    /// PBEsol, PBE functional for solids, exchange only
    PbesolX,
    /// PBEsol, PBE functional for solids, correlation only
    PbesolC,
    /// Becke (1988) GGA functional, exchange only
    B88X,
    /// Lee-Yang-Parr (1988) GGA functional, correlation only
    LypC,
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
            "lda-x" => Ok(Self::LdaX),
            "lda-c" => Ok(Self::LdaC),
            "pw91" => Ok(Self::Pw91),
            "pbe" => Ok(Self::Pbe),
            "pbe_x" => Ok(Self::PbeX),
            "pbe_c" => Ok(Self::PbeC),
            "rpbe" => Ok(Self::Rpbe),
            "wc" => Ok(Self::Wc),
            "pbesol" => Ok(Self::Pbesol),
            "pbesol_x" => Ok(Self::PbesolX),
            "pbesol_c" => Ok(Self::PbesolC),
            "b88_x" => Ok(Self::B88X),
            "lyp_c" => Ok(Self::LypC),
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
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue("XC_FUNCTIONAL", self.to_cell_value())
    }
}

impl ToCellValue for XcFunctional {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::String(
            match self {
                XcFunctional::Lda => "LDA",
                XcFunctional::LdaX => "LDA-X",
                XcFunctional::LdaC => "LDA-C",
                XcFunctional::Pw91 => "PW91",
                XcFunctional::Pbe => "PBE",
                XcFunctional::PbeX => "PBE_X",
                XcFunctional::PbeC => "PBE_C",
                XcFunctional::Rpbe => "RPBE",
                XcFunctional::Wc => "WC",
                XcFunctional::Pbesol => "PBESOL",
                XcFunctional::PbesolX => "PBESOL_X",
                XcFunctional::PbesolC => "PBESOL_C",
                XcFunctional::B88X => "B88_X",
                XcFunctional::LypC => "LYP_C",
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

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_fmt::CellValue;

    #[test]
    fn test_case_insensitive() {
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("lda")).unwrap(), XcFunctional::Lda);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("LDA")).unwrap(), XcFunctional::Lda);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("pbe")).unwrap(), XcFunctional::Pbe);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("PBE")).unwrap(), XcFunctional::Pbe);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("pw91")).unwrap(), XcFunctional::Pw91);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("rpbe")).unwrap(), XcFunctional::Rpbe);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("wc")).unwrap(), XcFunctional::Wc);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("pbesol")).unwrap(), XcFunctional::Pbesol);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("blyp")).unwrap(), XcFunctional::Blyp);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("hf")).unwrap(), XcFunctional::Hf);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("hf-lda")).unwrap(), XcFunctional::HfLda);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("sx")).unwrap(), XcFunctional::SX);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("sx-lda")).unwrap(), XcFunctional::SXlda);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("pbe0")).unwrap(), XcFunctional::Pbe0);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("b3lyp")).unwrap(), XcFunctional::B3lyp);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("hse03")).unwrap(), XcFunctional::Hse03);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("hse06")).unwrap(), XcFunctional::Hse06);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("rscan")).unwrap(), XcFunctional::Rscan);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("lda-x")).unwrap(), XcFunctional::LdaX);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("lda-c")).unwrap(), XcFunctional::LdaC);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("pbe_x")).unwrap(), XcFunctional::PbeX);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("pbe_c")).unwrap(), XcFunctional::PbeC);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("pbesol_x")).unwrap(), XcFunctional::PbesolX);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("pbesol_c")).unwrap(), XcFunctional::PbesolC);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("b88_x")).unwrap(), XcFunctional::B88X);
        assert_eq!(XcFunctional::from_cell_value(&CellValue::Str("lyp_c")).unwrap(), XcFunctional::LypC);
    }

    #[test]
    fn test_invalid() {
        assert!(XcFunctional::from_cell_value(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(XcFunctional::KEY_NAME, "XC_FUNCTIONAL");
    }
}

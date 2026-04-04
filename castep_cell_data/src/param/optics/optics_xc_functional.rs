use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_str;

/// Controls which functional is used to calculate the exchange-correlation potential during an optics run.
///
/// Keyword type: String
///
/// Default: OpticXcFunctional::Lda (derived from XC_FUNCTIONAL)
///
/// Example:
/// OPTICS_XC_FUNCTIONAL : PW91
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OpticXcFunctional {
    /// Local Density Approximation
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

impl Default for OpticXcFunctional {
    fn default() -> Self {
        Self::Lda
    }
}

impl FromCellValue for OpticXcFunctional {
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
            other => Err(Error::Message(format!("unknown OpticXcFunctional: {other}"))),
        }
    }
}

impl FromKeyValue for OpticXcFunctional {
    const KEY_NAME: &'static str = "OPTICS_XC_FUNCTIONAL";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for OpticXcFunctional {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("OPTICS_XC_FUNCTIONAL", self.to_cell_value())
    }
}

impl ToCellValue for OpticXcFunctional {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                OpticXcFunctional::Lda => "LDA",
                OpticXcFunctional::Pw91 => "PW91",
                OpticXcFunctional::Pbe => "PBE",
                OpticXcFunctional::Rpbe => "RPBE",
                OpticXcFunctional::Wc => "WC",
                OpticXcFunctional::Pbesol => "PBESOL",
                OpticXcFunctional::Blyp => "BLYP",
                OpticXcFunctional::Hf => "HF",
                OpticXcFunctional::HfLda => "HF-LDA",
                OpticXcFunctional::SX => "SX",
                OpticXcFunctional::SXlda => "SX-LDA",
                OpticXcFunctional::Pbe0 => "PBE0",
                OpticXcFunctional::B3lyp => "B3LYP",
                OpticXcFunctional::Hse03 => "HSE03",
                OpticXcFunctional::Hse06 => "HSE06",
                OpticXcFunctional::Rscan => "RSCAN",
            }
            .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_io::CellValue;

    #[test]
    fn test_case_insensitive_parsing() {
        let val = CellValue::Str("lda");
        assert_eq!(OpticXcFunctional::from_cell_value(&val).unwrap(), OpticXcFunctional::Lda);

        let val = CellValue::Str("LDA");
        assert_eq!(OpticXcFunctional::from_cell_value(&val).unwrap(), OpticXcFunctional::Lda);

        let val = CellValue::Str("pbe");
        assert_eq!(OpticXcFunctional::from_cell_value(&val).unwrap(), OpticXcFunctional::Pbe);

        let val = CellValue::Str("PBE");
        assert_eq!(OpticXcFunctional::from_cell_value(&val).unwrap(), OpticXcFunctional::Pbe);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(OpticXcFunctional::from_cell_value(&CellValue::Str("pw91")).unwrap(), OpticXcFunctional::Pw91);
        assert_eq!(OpticXcFunctional::from_cell_value(&CellValue::Str("rpbe")).unwrap(), OpticXcFunctional::Rpbe);
        assert_eq!(OpticXcFunctional::from_cell_value(&CellValue::Str("wc")).unwrap(), OpticXcFunctional::Wc);
        assert_eq!(OpticXcFunctional::from_cell_value(&CellValue::Str("pbesol")).unwrap(), OpticXcFunctional::Pbesol);
        assert_eq!(OpticXcFunctional::from_cell_value(&CellValue::Str("blyp")).unwrap(), OpticXcFunctional::Blyp);
        assert_eq!(OpticXcFunctional::from_cell_value(&CellValue::Str("hf")).unwrap(), OpticXcFunctional::Hf);
        assert_eq!(OpticXcFunctional::from_cell_value(&CellValue::Str("hf-lda")).unwrap(), OpticXcFunctional::HfLda);
        assert_eq!(OpticXcFunctional::from_cell_value(&CellValue::Str("sx")).unwrap(), OpticXcFunctional::SX);
        assert_eq!(OpticXcFunctional::from_cell_value(&CellValue::Str("sx-lda")).unwrap(), OpticXcFunctional::SXlda);
        assert_eq!(OpticXcFunctional::from_cell_value(&CellValue::Str("pbe0")).unwrap(), OpticXcFunctional::Pbe0);
        assert_eq!(OpticXcFunctional::from_cell_value(&CellValue::Str("b3lyp")).unwrap(), OpticXcFunctional::B3lyp);
        assert_eq!(OpticXcFunctional::from_cell_value(&CellValue::Str("hse03")).unwrap(), OpticXcFunctional::Hse03);
        assert_eq!(OpticXcFunctional::from_cell_value(&CellValue::Str("hse06")).unwrap(), OpticXcFunctional::Hse06);
        assert_eq!(OpticXcFunctional::from_cell_value(&CellValue::Str("rscan")).unwrap(), OpticXcFunctional::Rscan);
    }

    #[test]
    fn test_invalid() {
        assert!(OpticXcFunctional::from_cell_value(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(OpticXcFunctional::KEY_NAME, "OPTICS_XC_FUNCTIONAL");
    }

    #[test]
    fn test_default() {
        assert_eq!(OpticXcFunctional::default(), OpticXcFunctional::Lda);
    }

    #[test]
    fn test_to_cell_value() {
        assert_eq!(
            OpticXcFunctional::Lda.to_cell_value(),
            CellValue::String("LDA".to_string())
        );
        assert_eq!(
            OpticXcFunctional::Pw91.to_cell_value(),
            CellValue::String("PW91".to_string())
        );
    }

    #[test]
    fn test_to_cell() {
        let cell = OpticXcFunctional::Lda.to_cell();
        match cell {
            Cell::KeyValue(key, _) => assert_eq!(key, "OPTICS_XC_FUNCTIONAL"),
            _ => panic!("Expected KeyValue cell"),
        }
    }
}

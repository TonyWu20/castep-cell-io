use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromKeyValue, CResult};
use castep_cell_io::query::value_as_str;

/// Determines the Fermi-surface smearing scheme.
///
/// Keyword type: String
///
/// Default: SmearingScheme::Gaussian
///
/// Example:
/// SMEARING_SCHEME : ColdSmearing
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SmearingScheme {
    /// Gaussian smearing
    #[default]
    Gaussian,
    /// Gaussian splines smearing
    GaussianSplines,
    /// Fermi-Dirac smearing
    FermiDirac,
    /// Hermite polynomials smearing
    HermitePolynomials,
    /// Cold smearing
    ColdSmearing,
}

impl FromKeyValue for SmearingScheme {
    const KEY_NAME: &'static str = "SMEARING_SCHEME";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "gaussian" => Ok(Self::Gaussian),
            "gaussiansplines" => Ok(Self::GaussianSplines),
            "fermidirac" => Ok(Self::FermiDirac),
            "hermitepolynomials" => Ok(Self::HermitePolynomials),
            "coldsmearing" => Ok(Self::ColdSmearing),
            other => Err(castep_cell_io::Error::Message(format!("unknown SmearingScheme: {other}"))),
        }
    }
}

impl ToCell for SmearingScheme {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SMEARING_SCHEME", self.to_cell_value())
    }
}

impl ToCellValue for SmearingScheme {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                SmearingScheme::Gaussian => "Gaussian",
                SmearingScheme::GaussianSplines => "GaussianSplines",
                SmearingScheme::FermiDirac => "FermiDirac",
                SmearingScheme::HermitePolynomials => "HermitePolynomials",
                SmearingScheme::ColdSmearing => "ColdSmearing",
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
    fn test_case_insensitive() {
        assert_eq!(SmearingScheme::from_cell_value_kv(&CellValue::Str("gaussian")).unwrap(), SmearingScheme::Gaussian);
        assert_eq!(SmearingScheme::from_cell_value_kv(&CellValue::Str("GAUSSIAN")).unwrap(), SmearingScheme::Gaussian);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(SmearingScheme::from_cell_value_kv(&CellValue::Str("gaussiansplines")).unwrap(), SmearingScheme::GaussianSplines);
        assert_eq!(SmearingScheme::from_cell_value_kv(&CellValue::Str("fermidirac")).unwrap(), SmearingScheme::FermiDirac);
        assert_eq!(SmearingScheme::from_cell_value_kv(&CellValue::Str("hermitepolynomials")).unwrap(), SmearingScheme::HermitePolynomials);
        assert_eq!(SmearingScheme::from_cell_value_kv(&CellValue::Str("coldsmearing")).unwrap(), SmearingScheme::ColdSmearing);
    }

    #[test]
    fn test_invalid() {
        assert!(SmearingScheme::from_cell_value_kv(&CellValue::Str("invalid")).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(SmearingScheme::KEY_NAME, "SMEARING_SCHEME");
    }
}


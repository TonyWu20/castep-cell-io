use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_io::parse::{FromCellValue, FromKeyValue};
use castep_cell_io::{CResult, Error};
use castep_cell_io::query::value_as_string;
use serde::{Deserialize, Serialize};

/// Defines the type of electronic spectroscopy calculation to perform.
///
/// Keyword type: String
///
/// Default: SpectralTask::BandStructure
///
/// Example:
/// SPECTRAL_TASK : optics
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename = "SPECTRAL_TASK")]
pub enum SpectralTask {
    /// Calculates band structure.
    #[serde(alias = "BANDSTRUCTURE", alias = "bandstructure")]
    #[default]
    BandStructure,
    /// Calculates density of states.
    #[serde(alias = "DOS", alias = "dos")]
    Dos,
    /// Performs a TD-DFT calculation of excitation energies and optical spectra.
    #[serde(alias = "OPTICS", alias = "optics")]
    Optics,
    /// Performs core level spectroscopy calculation.
    #[serde(alias = "CORELOSS", alias = "coreloss")]
    CoreLoss,
    /// Performs all of the above calculations.
    #[serde(alias = "ALL", alias = "all")]
    All,
}

impl FromCellValue for SpectralTask {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_string(value)?.to_ascii_lowercase().as_str() {
            "bandstructure" => Ok(Self::BandStructure),
            "dos" => Ok(Self::Dos),
            "optics" => Ok(Self::Optics),
            "coreloss" => Ok(Self::CoreLoss),
            "all" => Ok(Self::All),
            other => Err(Error::Message(format!("unknown SpectralTask: {other}"))),
        }
    }
}

impl FromKeyValue for SpectralTask {
    const KEY_NAME: &'static str = "SPECTRAL_TASK";

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for SpectralTask {
    fn to_cell(&self) -> Cell {
        Cell::KeyValue("SPECTRAL_TASK", self.to_cell_value())
    }
}

impl ToCellValue for SpectralTask {
    fn to_cell_value(&self) -> CellValue {
        CellValue::String(
            match self {
                SpectralTask::BandStructure => "BandStructure",
                SpectralTask::Dos => "Dos",
                SpectralTask::Optics => "Optics",
                SpectralTask::CoreLoss => "CoreLoss",
                SpectralTask::All => "All",
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
        let val = CellValue::Str("bandstructure");
        assert_eq!(SpectralTask::from_cell_value(&val).unwrap(), SpectralTask::BandStructure);

        let val = CellValue::Str("BANDSTRUCTURE");
        assert_eq!(SpectralTask::from_cell_value(&val).unwrap(), SpectralTask::BandStructure);

        let val = CellValue::Str("BandStructure");
        assert_eq!(SpectralTask::from_cell_value(&val).unwrap(), SpectralTask::BandStructure);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(SpectralTask::from_cell_value(&CellValue::Str("bandstructure")).unwrap(), SpectralTask::BandStructure);
        assert_eq!(SpectralTask::from_cell_value(&CellValue::Str("dos")).unwrap(), SpectralTask::Dos);
        assert_eq!(SpectralTask::from_cell_value(&CellValue::Str("optics")).unwrap(), SpectralTask::Optics);
        assert_eq!(SpectralTask::from_cell_value(&CellValue::Str("coreloss")).unwrap(), SpectralTask::CoreLoss);
        assert_eq!(SpectralTask::from_cell_value(&CellValue::Str("all")).unwrap(), SpectralTask::All);
    }

    #[test]
    fn test_invalid_variant() {
        let val = CellValue::Str("invalid");
        assert!(SpectralTask::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_key_name() {
        assert_eq!(SpectralTask::KEY_NAME, "SPECTRAL_TASK");
    }

    #[test]
    fn test_default() {
        assert_eq!(SpectralTask::default(), SpectralTask::BandStructure);
    }

    #[test]
    fn test_round_trip() {
        let variants = [
            SpectralTask::BandStructure,
            SpectralTask::Dos,
            SpectralTask::Optics,
            SpectralTask::CoreLoss,
            SpectralTask::All,
        ];

        for variant in &variants {
            let cell_value = variant.to_cell_value();
            let parsed = SpectralTask::from_cell_value(&cell_value).unwrap();
            assert_eq!(parsed, *variant);
        }
    }
}

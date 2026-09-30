use castep_cell_fmt::{
    Cell, CellValue, ToCell, ToCellValue,
    parse::{FromBlock, FromCellValue},
    CResult, Error, query::value_as_f64,
};
use super::Species;
use crate::units::GammaUnit;

/// Represents a single entry within the SPECIES_GAMMA block,
/// linking a species to its nuclear gyromagnetic ratio (gamma).
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct SpeciesGammaEntry {
    /// The species (symbol or atomic number).
    pub species: Species,
    /// The gyromagnetic ratio value for this species.
    pub gamma: f64,
}

impl FromCellValue for SpeciesGammaEntry {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 2 => {
                Ok(SpeciesGammaEntry {
                    species: Species::from_cell_value(&arr[0])?,
                    gamma: value_as_f64(&arr[1])?,
                })
            }
            _ => Err(Error::Message(
                "SpeciesGammaEntry must be an array of [species, gamma]".into(),
            )),
        }
    }
}

impl ToCellValue for SpeciesGammaEntry {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Array(vec![
            self.species.to_cell_value(),
            CellValue::Float(self.gamma),
        ])
    }
}

/// Represents the SPECIES_GAMMA block.
///
/// Sets the nuclear gyromagnetic ratio for each atomic species.
/// Format:
/// %BLOCK SPECIES_GAMMA
/// [UNITS]
/// CCC1/I1 R1
/// CCC2/I2 R2
/// ...
/// %ENDBLOCK SPECIES_GAMMA
///
/// 6.11: the optional units line is a gamma unit; when omitted,
/// `radsectesla` (rad/s/tesla) is the CASTEP default.
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct SpeciesGamma {
    /// The gamma unit. If `None`, the default (`radsectesla`) is used.
    pub unit: Option<GammaUnit>,
    /// The list of species and their gamma values.
    #[builder(default)]
    pub entries: Vec<SpeciesGammaEntry>,
}

impl FromBlock for SpeciesGamma {
    const BLOCK_NAME: &'static str = "SPECIES_GAMMA";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        if rows.is_empty() {
            return Ok(Self {
                unit: None,
                entries: Vec::new(),
            });
        }

        let (unit, data_start) = if let CellValue::Array(arr) = &rows[0] {
            if arr.len() == 1 {
                if let Ok(u) = GammaUnit::from_cell_value(&arr[0]) {
                    (Some(u), 1)
                } else {
                    (None, 0)
                }
            } else {
                (None, 0)
            }
        } else {
            (None, 0)
        };

        let entries = rows[data_start..]
            .iter()
            .map(SpeciesGammaEntry::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;

        Ok(Self { unit, entries })
    }
}

impl ToCell for SpeciesGamma {
    fn to_cell(&self) -> Cell<'_> {
        let mut block_content = Vec::new();

        if let Some(ref u) = self.unit {
            block_content.push(CellValue::Array(vec![u.to_cell_value()]));
        }

        block_content.extend(
            self.entries
                .iter()
                .map(|entry| entry.to_cell_value()),
        );

        Cell::Block("SPECIES_GAMMA", block_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_species_gamma_entry_from_cell_value() {
        let val = CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::Float(0.107),
        ]);
        let entry = SpeciesGammaEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.species, Species::Symbol("Fe".to_string()));
        assert_eq!(entry.gamma, 0.107);
    }

    #[test]
    fn test_species_gamma_entry_insufficient_elements() {
        let val = CellValue::Array(vec![CellValue::Str("Fe")]);
        assert!(SpeciesGammaEntry::from_cell_value(&val).is_err());
    }

    #[test]
    fn test_species_gamma_empty() {
        let result = SpeciesGamma::from_block_rows(&[]).unwrap();
        assert!(result.unit.is_none());
        assert_eq!(result.entries.len(), 0);
    }

    #[test]
    fn test_species_gamma_with_unit() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("mhztesla")]),
            CellValue::Array(vec![CellValue::Str("Fe"), CellValue::Float(42.0)]),
        ];
        let result = SpeciesGamma::from_block_rows(&rows).unwrap();
        assert_eq!(result.unit, Some(GammaUnit::MegaHertzPerTesla));
        assert_eq!(result.entries.len(), 1);
    }

    #[test]
    fn test_species_gamma_without_unit() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("O"),
                CellValue::Float(0.0),
            ]),
        ];
        let result = SpeciesGamma::from_block_rows(&rows).unwrap();
        assert!(result.unit.is_none());
        assert_eq!(result.entries.len(), 1);
    }

    #[test]
    fn test_block_name() {
        assert_eq!(SpeciesGamma::BLOCK_NAME, "SPECIES_GAMMA");
    }

    #[test]
    fn test_species_gamma_builder() {
        let entry = SpeciesGammaEntry::builder()
            .species(Species::Symbol("Fe".to_string()))
            .gamma(0.107)
            .build();

        let sg = SpeciesGamma::builder()
            .unit(GammaUnit::RadPerSecTesla)
            .entries(vec![entry])
            .build();

        assert_eq!(sg.unit, Some(GammaUnit::RadPerSecTesla));
        assert_eq!(sg.entries.len(), 1);
        assert_eq!(sg.entries[0].gamma, 0.107);
    }
}

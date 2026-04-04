use castep_cell_io::{Cell, CellValue, ToCell, ToCellValue, FromCellValue, FromBlock, CResult, query::value_as_u32};

use super::Species;

/// Represents a single entry within the SPECIES_LCAO_STATES block,
/// linking a species to the number of LCAO states for it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpeciesLcaoState {
    /// The species (symbol or atomic number).
    pub species: Species,
    /// The number of angular momentum channels (LCAO states) for this species.
    pub num_states: u32,
}

impl FromCellValue for SpeciesLcaoState {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 2 => {
                Ok(SpeciesLcaoState {
                    species: Species::from_cell_value(&arr[0])?,
                    num_states: value_as_u32(&arr[1])?,
                })
            }
            _ => Err(castep_cell_io::Error::Message(
                "SpeciesLcaoState must be an array of [species, num_states]".into(),
            )),
        }
    }
}

impl ToCellValue for SpeciesLcaoState {
    fn to_cell_value(&self) -> CellValue {
        CellValue::Array(vec![
            self.species.to_cell_value(),
            CellValue::UInt(self.num_states),
        ])
    }
}

/// Represents the SPECIES_LCAO_STATES block.
///
/// Defines the size of the LCAO basis set used for population analysis.
/// Format:
/// %BLOCK SPECIES_LCAO_STATES
/// CCC1/I1 IB1
/// CCC2/I2 IB2
/// ...
/// %ENDBLOCK SPECIES_LCAO_STATES
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpeciesLcaoStates {
    /// The list of species and their corresponding LCAO state counts.
    pub states: Vec<SpeciesLcaoState>,
}

impl FromBlock for SpeciesLcaoStates {
    const BLOCK_NAME: &'static str = "SPECIES_LCAO_STATES";

    fn from_block_rows(rows: &[CellValue<'_>]) -> CResult<Self> {
        let states = rows
            .iter()
            .map(SpeciesLcaoState::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;
        Ok(SpeciesLcaoStates { states })
    }
}

impl ToCell for SpeciesLcaoStates {
    fn to_cell(&self) -> Cell {
        Cell::Block(
            "SPECIES_LCAO_STATES",
            self.states
                .iter()
                .map(|state_entry| state_entry.to_cell_value())
                .collect::<Vec<CellValue>>(),
        )
    }
}



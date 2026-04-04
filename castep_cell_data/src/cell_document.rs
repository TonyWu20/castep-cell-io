use derive_builder::Builder;
use castep_cell_io::{
    parse::{FromCellFile, FromBlock},
    Cell, CellValue, ToCell, ToCellFile, CResult, Error, query::find_block,
};

use crate::cell::{
    lattice_param::LatticeCart,
    positions::PositionsFrac,
    bz_sampling_kpoints::KpointsList,
    symmetry::SymmetryOps,
    constraints::FixCOM,
    constraints::IonicConstraints,
    external_fields::ExternalEfield,
    species::{SpeciesMass, SpeciesPot, SpeciesLcaoStates},
};

#[derive(Debug, Clone)]
pub enum Lattice {
    Cart(LatticeCart),
}

impl ToCell for Lattice {
    fn to_cell(&self) -> Cell {
        match self {
            Lattice::Cart(cart) => cart.to_cell(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Positions {
    Frac(PositionsFrac),
}

impl ToCell for Positions {
    fn to_cell(&self) -> Cell {
        match self {
            Positions::Frac(frac) => frac.to_cell(),
        }
    }
}

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct CellDocument {
    pub lattice: Lattice,
    pub positions: Positions,
    #[builder(default)]
    pub kpoints_list: Option<KpointsList>,
    #[builder(default)]
    pub symmetry_ops: Option<SymmetryOps>,
    #[builder(default)]
    pub fix_com: Option<FixCOM>,
    #[builder(default)]
    pub ionic_constraints: Option<IonicConstraints>,
    #[builder(default)]
    pub external_efield: Option<ExternalEfield>,
    #[builder(default)]
    pub species_mass: Option<SpeciesMass>,
    #[builder(default)]
    pub species_pot: Option<SpeciesPot>,
    #[builder(default)]
    pub species_lcao_states: Option<SpeciesLcaoStates>,
}

impl CellDocumentBuilder {
    pub fn validate(&self) -> CResult<()> {
        if self.lattice.is_none() {
            return Err(Error::Message("lattice is required".into()));
        }
        if self.positions.is_none() {
            return Err(Error::Message("positions is required".into()));
        }
        Ok(())
    }
}

impl FromCellFile for CellDocument {
    fn from_cell_file(cells: &[Cell<'_>]) -> CResult<Self> {
        let lattice_rows = find_block(cells, "LATTICE_CART")?;
        let lattice = Lattice::Cart(LatticeCart::from_block_rows(lattice_rows)?);

        let positions_rows = find_block(cells, "POSITIONS_FRAC")?;
        let positions = Positions::Frac(PositionsFrac::from_block_rows(positions_rows)?);

        let kpoints_list = find_block(cells, "KPOINTS_LIST")
            .ok()
            .map(|rows| KpointsList::from_block_rows(rows))
            .transpose()?;

        let symmetry_ops = find_block(cells, "SYMMETRY_OPS")
            .ok()
            .map(|rows| SymmetryOps::from_block_rows(rows))
            .transpose()?;

        let fix_com = cells
            .iter()
            .find_map(|c| {
                if let Cell::KeyValue(k, _v) = c {
                    if k.eq_ignore_ascii_case("FIX_COM") {
                        return Some(FixCOM(true));
                    }
                }
                None
            });

        let ionic_constraints = find_block(cells, "IONIC_CONSTRAINTS")
            .ok()
            .map(|rows| IonicConstraints::from_block_rows(rows))
            .transpose()?;

        let external_efield = find_block(cells, "EXTERNAL_EFIELD")
            .ok()
            .map(|rows| ExternalEfield::from_block_rows(rows))
            .transpose()?;

        let species_mass = find_block(cells, "SPECIES_MASS")
            .ok()
            .map(|rows| SpeciesMass::from_block_rows(rows))
            .transpose()?;

        let species_pot = find_block(cells, "SPECIES_POT")
            .ok()
            .map(|rows| SpeciesPot::from_block_rows(rows))
            .transpose()?;

        let species_lcao_states = find_block(cells, "SPECIES_LCAO_STATES")
            .ok()
            .map(|rows| SpeciesLcaoStates::from_block_rows(rows))
            .transpose()?;

        Ok(CellDocument {
            lattice,
            positions,
            kpoints_list,
            symmetry_ops,
            fix_com,
            ionic_constraints,
            external_efield,
            species_mass,
            species_pot,
            species_lcao_states,
        })
    }
}

impl ToCellFile for CellDocument {
    fn to_cell_file(&self) -> Vec<Cell> {
        let mut cells = vec![self.lattice.to_cell(), self.positions.to_cell()];

        if let Some(kp) = &self.kpoints_list {
            cells.push(kp.to_cell());
        }
        if let Some(sym) = &self.symmetry_ops {
            cells.push(sym.to_cell());
        }
        if let Some(_fc) = &self.fix_com {
            cells.push(Cell::Flag("FIX_COM"));
        }
        if let Some(ic) = &self.ionic_constraints {
            cells.push(ic.to_cell());
        }
        if let Some(ef) = &self.external_efield {
            cells.push(ef.to_cell());
        }
        if let Some(sm) = &self.species_mass {
            cells.push(sm.to_cell());
        }
        if let Some(sp) = &self.species_pot {
            cells.push(sp.to_cell());
        }
        if let Some(sl) = &self.species_lcao_states {
            cells.push(sl.to_cell());
        }

        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mg2sio4_cell() {
        let input = include_str!("../../Mg2SiO4_Cr_1.cell");
        let doc = castep_cell_io::parse::<CellDocument>(input).expect("Failed to parse");

        assert!(matches!(doc.lattice, Lattice::Cart(_)));
        assert!(matches!(doc.positions, Positions::Frac(_)));
        assert!(doc.kpoints_list.is_some());
        assert!(doc.symmetry_ops.is_some());
        assert!(doc.fix_com.is_some());
        assert!(doc.ionic_constraints.is_some());
        assert!(doc.external_efield.is_some());
        assert!(doc.species_mass.is_some());
        assert!(doc.species_pot.is_some());
        assert!(doc.species_lcao_states.is_some());
    }
}

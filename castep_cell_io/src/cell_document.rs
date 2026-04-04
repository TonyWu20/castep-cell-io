use derive_builder::Builder;
use castep_cell_fmt::{
    parse::{FromCellFile, FromBlock},
    Cell, CellValue, ToCell, ToCellFile, CResult, Error, query::find_block,
};

use crate::cell::{
    lattice_param::LatticeCart,
    positions::{PositionsFrac, PositionsAbs},
    bz_sampling_kpoints::{
        KpointsList, BsKpointPath, BSKpointList, OpticsKpointsList, MagresKpointsList,
    },
    symmetry::SymmetryOps,
    constraints::{FixCOM, IonicConstraints, NonlinearConstraints, FixAllIons, FixAllCell},
    external_fields::{ExternalEfield, ExternalPressure},
    species::{SpeciesMass, SpeciesPot, SpeciesLcaoStates, SpeciesQ, HubbardU, SedcCustomParams},
    phonon::{PhononKpointList, PhononKpointPath, PhononGammaDirections, PhononFineKpointList, PhononSupercellMatrix, SupercellKpointListCastep},
    velocities::IonicVelocities,
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
    Abs(PositionsAbs),
}

impl ToCell for Positions {
    fn to_cell(&self) -> Cell {
        match self {
            Positions::Frac(frac) => frac.to_cell(),
            Positions::Abs(abs) => abs.to_cell(),
        }
    }
}

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct CellDocument {
    pub lattice: Lattice,
    pub positions: Positions,
    #[builder(default, setter(strip_option))]
    pub kpoints_list: Option<KpointsList>,
    #[builder(default, setter(strip_option))]
    pub bs_kpoint_path: Option<BsKpointPath>,
    #[builder(default, setter(strip_option))]
    pub bs_kpoints_list: Option<BSKpointList>,
    #[builder(default, setter(strip_option))]
    pub optics_kpoints_list: Option<OpticsKpointsList>,
    #[builder(default, setter(strip_option))]
    pub magres_kpoints_list: Option<MagresKpointsList>,
    #[builder(default, setter(strip_option))]
    pub symmetry_ops: Option<SymmetryOps>,
    #[builder(default, setter(strip_option))]
    pub fix_com: Option<FixCOM>,
    #[builder(default, setter(strip_option))]
    pub ionic_constraints: Option<IonicConstraints>,
    #[builder(default, setter(strip_option))]
    pub nonlinear_constraints: Option<NonlinearConstraints>,
    #[builder(default, setter(strip_option))]
    pub fix_all_ions: Option<FixAllIons>,
    #[builder(default, setter(strip_option))]
    pub fix_all_cell: Option<FixAllCell>,
    #[builder(default, setter(strip_option))]
    pub external_efield: Option<ExternalEfield>,
    #[builder(default, setter(strip_option))]
    pub external_pressure: Option<ExternalPressure>,
    #[builder(default, setter(strip_option))]
    pub species_mass: Option<SpeciesMass>,
    #[builder(default, setter(strip_option))]
    pub species_pot: Option<SpeciesPot>,
    #[builder(default, setter(strip_option))]
    pub species_lcao_states: Option<SpeciesLcaoStates>,
    #[builder(default, setter(strip_option))]
    pub species_q: Option<SpeciesQ>,
    #[builder(default, setter(strip_option))]
    pub hubbard_u: Option<HubbardU>,
    #[builder(default, setter(strip_option))]
    pub sedc_custom_params: Option<SedcCustomParams>,
    #[builder(default, setter(strip_option))]
    pub phonon_kpoint_list: Option<PhononKpointList>,
    #[builder(default, setter(strip_option))]
    pub phonon_kpoint_path: Option<PhononKpointPath>,
    #[builder(default, setter(strip_option))]
    pub phonon_gamma_directions: Option<PhononGammaDirections>,
    #[builder(default, setter(strip_option))]
    pub phonon_fine_kpoint_list: Option<PhononFineKpointList>,
    #[builder(default, setter(strip_option))]
    pub phonon_supercell_matrix: Option<PhononSupercellMatrix>,
    #[builder(default, setter(strip_option))]
    pub supercell_kpoint_list: Option<SupercellKpointListCastep>,
    #[builder(default, setter(strip_option))]
    pub ionic_velocities: Option<IonicVelocities>,
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

        let positions = if find_block(cells, "POSITIONS_FRAC").is_ok() {
            Positions::Frac(PositionsFrac::from_block_rows(find_block(cells, "POSITIONS_FRAC")?)?)
        } else {
            Positions::Abs(PositionsAbs::from_block_rows(find_block(cells, "POSITIONS_ABS")?)?)
        };

        let kpoints_list = find_block(cells, "KPOINTS_LIST")
            .ok()
            .map(|rows| KpointsList::from_block_rows(rows))
            .transpose()?;

        let bs_kpoint_path = find_block(cells, "BS_KPOINT_PATH")
            .ok()
            .map(|rows| BsKpointPath::from_block_rows(rows))
            .transpose()?;

        let bs_kpoints_list = find_block(cells, "BS_KPOINTS_LIST")
            .ok()
            .map(|rows| BSKpointList::from_block_rows(rows))
            .transpose()?;

        let optics_kpoints_list = find_block(cells, "OPTICS_KPOINTS_LIST")
            .ok()
            .map(|rows| OpticsKpointsList::from_block_rows(rows))
            .transpose()?;

        let magres_kpoints_list = find_block(cells, "MAGRES_KPOINTS_LIST")
            .ok()
            .map(|rows| MagresKpointsList::from_block_rows(rows))
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

        let nonlinear_constraints = find_block(cells, "NONLINEAR_CONSTRAINTS")
            .ok()
            .map(|rows| NonlinearConstraints::from_block_rows(rows))
            .transpose()?;

        let fix_all_ions = cells
            .iter()
            .find_map(|c| {
                if let Cell::KeyValue(k, _v) = c {
                    if k.eq_ignore_ascii_case("FIX_ALL_IONS") {
                        return Some(FixAllIons(true));
                    }
                }
                None
            });

        let fix_all_cell = cells
            .iter()
            .find_map(|c| {
                if let Cell::KeyValue(k, _v) = c {
                    if k.eq_ignore_ascii_case("FIX_ALL_CELL") {
                        return Some(FixAllCell(true));
                    }
                }
                None
            });

        let external_efield = find_block(cells, "EXTERNAL_EFIELD")
            .ok()
            .map(|rows| ExternalEfield::from_block_rows(rows))
            .transpose()?;

        let external_pressure = find_block(cells, "EXTERNAL_PRESSURE")
            .ok()
            .map(|rows| ExternalPressure::from_block_rows(rows))
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

        let species_q = find_block(cells, "SPECIES_Q")
            .ok()
            .map(|rows| SpeciesQ::from_block_rows(rows))
            .transpose()?;

        let hubbard_u = find_block(cells, "HUBBARD_U")
            .ok()
            .map(|rows| HubbardU::from_block_rows(rows))
            .transpose()?;

        let sedc_custom_params = find_block(cells, "SEDC_CUSTOM_PARAMS")
            .ok()
            .map(|rows| SedcCustomParams::from_block_rows(rows))
            .transpose()?;

        let phonon_kpoint_list = find_block(cells, "PHONON_KPOINT_LIST")
            .ok()
            .map(|rows| PhononKpointList::from_block_rows(rows))
            .transpose()?;

        let phonon_kpoint_path = find_block(cells, "PHONON_KPOINT_PATH")
            .ok()
            .map(|rows| PhononKpointPath::from_block_rows(rows))
            .transpose()?;

        let phonon_gamma_directions = find_block(cells, "PHONON_GAMMA_DIRECTIONS")
            .ok()
            .map(|rows| PhononGammaDirections::from_block_rows(rows))
            .transpose()?;

        let phonon_fine_kpoint_list = find_block(cells, "PHONON_FINE_KPOINT_LIST")
            .ok()
            .map(|rows| PhononFineKpointList::from_block_rows(rows))
            .transpose()?;

        let phonon_supercell_matrix = find_block(cells, "PHONON_SUPERCELL_MATRIX")
            .ok()
            .map(|rows| PhononSupercellMatrix::from_block_rows(rows))
            .transpose()?;

        let supercell_kpoint_list = find_block(cells, "SUPERCELL_KPOINT_LIST")
            .ok()
            .map(|rows| SupercellKpointListCastep::from_block_rows(rows))
            .transpose()?;

        let ionic_velocities = find_block(cells, "IONIC_VELOCITIES")
            .ok()
            .map(|rows| IonicVelocities::from_block_rows(rows))
            .transpose()?;

        Ok(CellDocument {
            lattice,
            positions,
            kpoints_list,
            bs_kpoint_path,
            bs_kpoints_list,
            optics_kpoints_list,
            magres_kpoints_list,
            symmetry_ops,
            fix_com,
            ionic_constraints,
            nonlinear_constraints,
            fix_all_ions,
            fix_all_cell,
            external_efield,
            external_pressure,
            species_mass,
            species_pot,
            species_lcao_states,
            species_q,
            hubbard_u,
            sedc_custom_params,
            phonon_kpoint_list,
            phonon_kpoint_path,
            phonon_gamma_directions,
            phonon_fine_kpoint_list,
            phonon_supercell_matrix,
            supercell_kpoint_list,
            ionic_velocities,
        })
    }
}

impl ToCellFile for CellDocument {
    fn to_cell_file(&self) -> Vec<Cell> {
        let mut cells = vec![self.lattice.to_cell(), self.positions.to_cell()];

        if let Some(kp) = &self.kpoints_list {
            cells.push(kp.to_cell());
        }
        if let Some(bp) = &self.bs_kpoint_path {
            cells.push(bp.to_cell());
        }
        if let Some(bk) = &self.bs_kpoints_list {
            cells.push(bk.to_cell());
        }
        if let Some(ok) = &self.optics_kpoints_list {
            cells.push(ok.to_cell());
        }
        if let Some(mk) = &self.magres_kpoints_list {
            cells.push(mk.to_cell());
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
        if let Some(nc) = &self.nonlinear_constraints {
            cells.push(nc.to_cell());
        }
        if let Some(_fi) = &self.fix_all_ions {
            cells.push(Cell::Flag("FIX_ALL_IONS"));
        }
        if let Some(_fc) = &self.fix_all_cell {
            cells.push(Cell::Flag("FIX_ALL_CELL"));
        }
        if let Some(ef) = &self.external_efield {
            cells.push(ef.to_cell());
        }
        if let Some(ep) = &self.external_pressure {
            cells.push(ep.to_cell());
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
        if let Some(sq) = &self.species_q {
            cells.push(sq.to_cell());
        }
        if let Some(hu) = &self.hubbard_u {
            cells.push(hu.to_cell());
        }
        if let Some(sc) = &self.sedc_custom_params {
            cells.push(sc.to_cell());
        }
        if let Some(pk) = &self.phonon_kpoint_list {
            cells.push(pk.to_cell());
        }
        if let Some(pp) = &self.phonon_kpoint_path {
            cells.push(pp.to_cell());
        }
        if let Some(pg) = &self.phonon_gamma_directions {
            cells.push(pg.to_cell());
        }
        if let Some(pf) = &self.phonon_fine_kpoint_list {
            cells.push(pf.to_cell());
        }
        if let Some(pm) = &self.phonon_supercell_matrix {
            cells.push(pm.to_cell());
        }
        if let Some(sk) = &self.supercell_kpoint_list {
            cells.push(sk.to_cell());
        }
        if let Some(iv) = &self.ionic_velocities {
            cells.push(iv.to_cell());
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
        let doc = castep_cell_fmt::parse::<CellDocument>(input).expect("Failed to parse");

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

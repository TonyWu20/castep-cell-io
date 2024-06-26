use std::fmt::Display;

use castep_periodic_table::element::ElementSymbol;
use chemrust_core::data::atom::CoreAtomData;
use chemrust_core::data::geom::coordinates::CoordData;
use chemrust_core::data::lattice::{CellConstants, UnitCellParameters};
use nalgebra::{Matrix3, Point3};

use crate::{IonicPositionBlock, LatticeABC, LatticeParamBlock};

use self::constraints::{FixAllCell, FixAllIons, FixCom, IonicConstraintsBlock};
use self::symmetry_ops::SymmetryOpsBlock;

use super::{
    ExtEFieldBlock, ExtPressureBlock, KpointSettings, NCKpointSettings, SpeciesLCAOStatesBlock,
    SpeciesMassBlock, SpeciesPotBlock,
};

pub mod constraints;
pub mod external_fields;
pub mod ionic_positions;
pub mod kpoint_settings;
pub mod lattice_parameters;
pub mod species_characters;
pub mod symmetry_ops;

#[derive(Debug, Clone)]
pub struct CellEssentials {
    lattice_block: LatticeParamBlock,
    ionic_pos_block: IonicPositionBlock,
}

impl UnitCellParameters for LatticeParamBlock {
    fn lattice_bases(&self) -> Matrix3<f64> {
        match self.parameter() {
            crate::LatticeParam::LatticeCart(lat_cart) => {
                Matrix3::from_column_slice(&[lat_cart.a(), lat_cart.b(), lat_cart.c()].concat())
            }
            crate::LatticeParam::LatticeABC(lat_abc) => {
                let LatticeABC {
                    a,
                    b,
                    c,
                    alpha,
                    beta,
                    gamma,
                } = lat_abc;
                CellConstants::new(a, b, c, alpha.value(), beta.value(), gamma.value())
                    .lattice_bases()
            }
        }
    }
}

impl CellEssentials {
    pub fn new(lattice_block: LatticeParamBlock, ionic_pos_block: IonicPositionBlock) -> Self {
        Self {
            lattice_block,
            ionic_pos_block,
        }
    }

    pub fn lattice_block(&self) -> &LatticeParamBlock {
        &self.lattice_block
    }

    pub fn ionic_pos_block(&self) -> &IonicPositionBlock {
        &self.ionic_pos_block
    }

    pub fn lattice_block_mut(&mut self) -> &mut LatticeParamBlock {
        &mut self.lattice_block
    }

    pub fn ionic_pos_block_mut(&mut self) -> &mut IonicPositionBlock {
        &mut self.ionic_pos_block
    }
}

impl CoreAtomData for IonicPositionBlock {
    fn indices_repr(&self) -> Vec<usize> {
        (0..self.positions().len()).collect()
    }

    fn symbols_repr(&self) -> Vec<ElementSymbol> {
        self.positions().iter().map(|pos| pos.symbol()).collect()
    }

    fn coords_repr(&self) -> Vec<CoordData> {
        self.positions()
            .iter()
            .map(|pos| CoordData::Fractional(Point3::from_slice(&pos.coordinate())))
            .collect()
    }

    fn labels_repr(&self) -> Vec<Option<String>> {
        vec![None; self.positions().len()]
    }
}

#[derive(Debug, Clone)]
pub enum CellEntries {
    KpointSettings(KpointSettings),
    NCKpointSettings(NCKpointSettings),
    /// This keyword controls whether or not all of the lattice parameters remain fixed during relaxation or molecular dynamics.
    FixAllCell(FixAllCell),
    FixAllIons(FixAllIons),
    /// This keyword controls whether or not the center of mass of the ions remains fixed during relaxation or molecular dynamics.
    FixCom(FixCom),
    IonicConstraints(IonicConstraintsBlock),
    ExtEfield(ExtEFieldBlock),
    ExtPressure(ExtPressureBlock),
    SpeciesMass(SpeciesMassBlock),
    SpeciesPot(SpeciesPotBlock),
    SpeciesLCAOStates(SpeciesLCAOStatesBlock),
    SymmetryOps(SymmetryOpsBlock),
}

impl Display for CellEntries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            CellEntries::KpointSettings(v) => format!("{v}"),
            CellEntries::NCKpointSettings(v) => format!("{v}"),
            CellEntries::FixAllCell(v) => format!("{v}"),
            CellEntries::FixAllIons(v) => format!("{v}"),
            CellEntries::FixCom(v) => format!("{v}"),
            CellEntries::IonicConstraints(v) => format!("{v}"),
            CellEntries::ExtEfield(v) => format!("{v}"),
            CellEntries::ExtPressure(v) => format!("{v}"),
            CellEntries::SpeciesMass(v) => format!("{v}"),
            CellEntries::SpeciesPot(v) => format!("{v}"),
            CellEntries::SpeciesLCAOStates(v) => format!("{v}"),
            CellEntries::SymmetryOps(v) => format!("{v}"),
        };
        write!(f, "{content}")
    }
}

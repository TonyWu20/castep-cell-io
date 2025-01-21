use chemrust_core::data::atom::CoreAtomData;
use chemrust_core::data::lattice::CrystalModel;
use chemrust_core::data::lattice::UnitCellParameters;

use crate::{
    cell_document::{
        sections::lattice_parameters::LatticeParam, CellDocument, IonicPosition,
        IonicPositionBlock, LatticeParamBlock,
    },
    LengthUnit,
    PositionsKeywords::POSITIONS_FRAC,
};

use super::CellEssentials;
use super::LatticeCart;

pub fn to_cell_document<T: CrystalModel>(model: &T) -> CellDocument {
    let lattice_param_block = LatticeParamBlock::from(model.get_cell_parameters());
    let ionic_pos_block = IonicPositionBlock::from_atom_data(
        model.get_atom_data(),
        &model.get_cell_parameters().lattice_bases(),
    );
    let model_description = CellEssentials::new(lattice_param_block, ionic_pos_block);
    CellDocument::new(model_description)
}

impl<T> From<&T> for LatticeParamBlock
where
    T: UnitCellParameters,
{
    fn from(value: &T) -> Self {
        let lattice_bases = value.lattice_bases();
        let (a, b, c) = (
            lattice_bases.column(0).into(),
            lattice_bases.column(1).into(),
            lattice_bases.column(2).into(),
        );
        let lattice_cart = LatticeCart::new(a, b, c);
        let lattice_param = LatticeParam::LatticeCart(lattice_cart);
        LatticeParamBlock::new(LengthUnit::Ang, lattice_param)
    }
}

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
    let lattice_bases = model.get_cell_parameters().lattice_bases();
    let (a, b, c) = (
        lattice_bases.column(0).into(),
        lattice_bases.column(1).into(),
        lattice_bases.column(2).into(),
    );
    let lattice_cart = LatticeCart::new(a, b, c);
    let lattice_param = LatticeParam::LatticeCart(lattice_cart);
    let lattice_param_block = LatticeParamBlock::new(LengthUnit::Ang, lattice_param);
    let atom_data = model.get_atom_data();
    let symbols = atom_data.symbols_repr();
    let coords = atom_data
        .coords_repr()
        .iter()
        .map(|cd| cd.cart_to_frac(&lattice_bases).raw_data().into())
        .collect::<Vec<[f64; 3]>>();
    let ionic_positions = symbols
        .iter()
        .zip(coords.iter())
        .map(|(&symbol, &coord)| IonicPosition::new(symbol, coord, None))
        .collect::<Vec<IonicPosition>>();
    let ionic_pos_block =
        IonicPositionBlock::new(LengthUnit::Ang, ionic_positions, POSITIONS_FRAC, true);
    let model_description = CellEssentials::new(lattice_param_block, ionic_pos_block);
    CellDocument::new(model_description)
}

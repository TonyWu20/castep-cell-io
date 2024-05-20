use winnow::{ascii::Caseless, combinator::alt, PResult, Parser};

use crate::{
    data::{LatticeABC, LatticeCart, LatticeParam, LatticeParamBlock},
    keywords::{DocumentSections, LatticeBlockType},
    parsing::{helpers::block::get_block_data, CellParseError},
};

use super::length_unit;

fn assign_lattice_cart<'s>(input: &mut &'s str) -> PResult<DocumentSections<'s>> {
    Caseless("lattice_cart")
        .map(|_| DocumentSections::CellLatticeVectors(LatticeBlockType::LATTICE_CART))
        .parse_next(input)
}

fn assign_lattice_abc<'s>(input: &mut &'s str) -> PResult<DocumentSections<'s>> {
    Caseless("lattice_abc")
        .map(|_| DocumentSections::CellLatticeVectors(LatticeBlockType::LATTICE_ABC))
        .parse_next(input)
}

pub fn assign_lattice_type<'s>(input: &mut &'s str) -> PResult<DocumentSections<'s>> {
    alt((assign_lattice_abc, assign_lattice_cart)).parse_next(input)
}

/// This should be receiving the output of `get_block_data()`
fn parse_lattice_cart(data_input: &str) -> Result<LatticeParamBlock, CellParseError> {
    let mut lines = data_input.split_whitespace().peekable();
    let length_unit = lines.peek().and_then(|s| length_unit(s).ok());
    let values: Vec<f64> = if length_unit.is_some() {
        lines
            .skip(1)
            .filter_map(|s| s.parse::<f64>().ok())
            .collect()
    } else {
        lines
            .filter_map(|s| s.parse::<f64>().ok()) // Automatically ignores the commented out blank lines
            .collect()
    };
    let array: [f64; 9] = values.try_into().map_err(|_| CellParseError::Invalid)?;
    let a: [f64; 3] = array[0..3].try_into().unwrap();
    let b: [f64; 3] = array[3..6].try_into().unwrap();
    let c: [f64; 3] = array[6..9].try_into().unwrap();
    let lattice_cart = LatticeCart::new(a, b, c);
    let unit = length_unit.unwrap_or_default();
    Ok(LatticeParamBlock::new(
        unit,
        LatticeParam::LatticeCart(lattice_cart),
    ))
}

/// This should be receiving the output of `get_block_data()`
fn parse_lattice_abc(data_input: &str) -> Result<LatticeParamBlock, CellParseError> {
    let mut lines = data_input.split_whitespace().peekable();
    let length_unit = lines.peek().and_then(|s| length_unit(s).ok());
    let values: Vec<f64> = if length_unit.is_some() {
        lines
            .skip(1)
            .filter_map(|s| s.parse::<f64>().ok())
            .collect()
    } else {
        lines
            .filter_map(|s| s.parse::<f64>().ok()) // Automatically ignores the commented out blank lines
            .collect()
    };
    let values: [f64; 6] = values.try_into().map_err(|_| CellParseError::Invalid)?;
    let [a, b, c, alpha, beta, gamma] = values;
    let lattice_abc = LatticeABC::new(a, b, c, alpha, beta, gamma);
    let unit = length_unit.unwrap_or_default();
    Ok(LatticeParamBlock::new(
        unit,
        LatticeParam::LatticeABC(lattice_abc),
    ))
}

pub fn parse_lattice_param(
    input: &mut &str,
    lat_type: LatticeBlockType,
) -> Result<LatticeParamBlock, CellParseError> {
    let data_input = get_block_data(input).map_err(|_| CellParseError::Invalid)?;
    match lat_type {
        LatticeBlockType::LATTICE_CART => parse_lattice_cart(&data_input),
        LatticeBlockType::LATTICE_ABC => parse_lattice_abc(&data_input),
    }
}

#[cfg(test)]
mod test {
    use crate::{
        keywords::{DocumentSections, LatticeBlockType},
        parsing::helpers::{block::get_block_data, current_sections},
    };

    use super::parse_lattice_cart;

    #[test]
    fn keywords_lattice() {
        let mut input_1 = "%BLOCK LATTICE_CART
   18.931530020488704480   -0.000000000000003553    0.000000000000000000 #a
   -9.465765010246645517   16.395185930251127360    0.000000000000000000 #b
    0.000000000000000000    0.000000000000000000    9.999213039981000861 #c
%ENDBLOCK LATTICE_CART
";
        let section = current_sections(&mut input_1).unwrap();
        if let DocumentSections::CellLatticeVectors(LatticeBlockType::LATTICE_CART) = section {
            let data = get_block_data(&mut input_1).unwrap();
            let lattice_cart = parse_lattice_cart(&data);
            println!("{:#?}", lattice_cart);
        }
        let mut input_2 = "%BLOCK LATTICE_CART
   bohr
   18.931530020488704480   -0.000000000000003553    0.000000000000000000 #a
   -9.465765010246645517   16.395185930251127360    0.000000000000000000 #b
    0.000000000000000000    0.000000000000000000    9.999213039981000861 #c
%ENDBLOCK LATTICE_CART
";
        let section = current_sections(&mut input_2).unwrap();
        if let DocumentSections::CellLatticeVectors(LatticeBlockType::LATTICE_CART) = section {
            let data = get_block_data(&mut input_2).unwrap();
            let lattice_cart = parse_lattice_cart(&data);
            println!("{:#?}", lattice_cart);
        }
    }
}

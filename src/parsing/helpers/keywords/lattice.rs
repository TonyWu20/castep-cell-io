use winnow::{ascii::Caseless, combinator::alt, PResult, Parser};

use crate::{
    data::{LatticeABC, LatticeCart, LatticeParam},
    keywords::{DocumentSections, LatticeBlockType},
    parsing::{helpers::block::get_block_data, CellParseError},
};

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
fn parse_lattice_cart(data_input: &str) -> Result<LatticeParam, CellParseError> {
    let values: Vec<f64> = data_input
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect();
    if values.len() != 9 {
        return Err(CellParseError::UnexpectedLength);
    }
    let array: [f64; 9] = values.try_into().unwrap();
    let a: [f64; 3] = array[0..3].try_into().unwrap();
    let b: [f64; 3] = array[3..6].try_into().unwrap();
    let c: [f64; 3] = array[6..9].try_into().unwrap();
    let lattice_cart = LatticeCart::new(a, b, c);
    Ok(LatticeParam::LatticeCart(lattice_cart))
}

/// This should be receiving the output of `get_block_data()`
fn parse_lattice_abc(data_input: &str) -> Result<LatticeParam, CellParseError> {
    let values: Vec<f64> = data_input
        .split_whitespace() // This line and the next line
        .filter_map(|s| s.parse::<f64>().ok()) // Automatically ignores the commented out blank lines
        .collect();
    if values.len() != 6 {
        return Err(CellParseError::UnexpectedLength);
    }
    let a = values[0];
    let b = values[1];
    let c = values[2];
    let alpha = values[3];
    let beta = values[4];
    let gamma = values[5];
    let lattice_abc = LatticeABC::new(a, b, c, alpha, beta, gamma);
    Ok(LatticeParam::LatticeABC(lattice_abc))
}

pub fn parse_lattice_param(
    input: &mut &str,
    lat_type: LatticeBlockType,
) -> Result<LatticeParam, CellParseError> {
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
        let mut input = "%BLOCK LATTICE_CART
   18.931530020488704480   -0.000000000000003553    0.000000000000000000 #a
   -9.465765010246645517   16.395185930251127360    0.000000000000000000 #b
    0.000000000000000000    0.000000000000000000    9.999213039981000861 #c
%ENDBLOCK LATTICE_CART
";
        let section = current_sections(&mut input).unwrap();
        if let DocumentSections::CellLatticeVectors(LatticeBlockType::LATTICE_CART) = section {
            let data = get_block_data(&mut input).unwrap();
            let lattice_cart = parse_lattice_cart(&data);
            println!("{:#?}", lattice_cart);
        }
    }
}

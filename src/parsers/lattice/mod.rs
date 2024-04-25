use winnow::combinator::alt;
use winnow::combinator::preceded;
use winnow::PResult;
use winnow::Parser;

use crate::keywords::CellLatticeVectors;

use super::block_prefix;

fn parse_lattice_cart_prefix<'s>(input: &mut &'s str) -> PResult<&'s str> {
    "LATTICE_CART".parse_next(input)
}

fn parse_lattice_abc_prefix<'s>(input: &mut &'s str) -> PResult<&'s str> {
    "LATTICE_ABC".parse_next(input)
}

pub fn parse_lattice_block<'s>(input: &mut &'s str) -> PResult<CellLatticeVectors> {
    preceded(
        block_prefix,
        alt((
            parse_lattice_cart_prefix.map(|_| CellLatticeVectors::LATTICE_CART),
            parse_lattice_abc_prefix.map(|_| CellLatticeVectors::LATTICE_ABC),
        )),
    )
    .parse_next(input)
}

#[cfg(test)]
mod test {
    use super::parse_lattice_block;

    #[test]
    fn lattice_block_keyword() {
        let mut input = ["%BLOCK LATTICE_CART", "%BLOCK LATTICE_ABC"];
        input.iter_mut().for_each(|input| {
            let lattice_type = parse_lattice_block(input).unwrap();
            println!("{:?}", lattice_type);
        });
    }
}

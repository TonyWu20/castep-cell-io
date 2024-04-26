use winnow::{
    ascii::{space1, till_line_ending, Caseless},
    combinator::{alt, preceded, terminated},
    token::take_until,
    PResult, Parser,
};

use crate::keywords::DocumentSections;

use self::keywords::{ionic_positions::assign_positions_type, lattice::assign_lattice_type};

mod keywords;

/// Go to block name
fn strip_to_block_name<'s>(input: &mut &'s str) -> PResult<&'s str> {
    preceded(terminated(Caseless("%block"), space1), till_line_ending).parse_next(input)
}

/// When it is "keyword : value"
fn field_name<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_until(0.., ":")
        .map(|s: &str| s.trim())
        .parse_next(input)
}

fn get_keyword<'s>(input: &mut &'s str) -> PResult<&'s str> {
    alt((strip_to_block_name, field_name)).parse_next(input)
}

fn dispatch_sections<'s>(input: &mut &'s str) -> PResult<DocumentSections> {
    let keyword: &str = get_keyword(input)?;
    let assign = alt((assign_lattice_type, assign_positions_type)).parse(keyword);
    match assign {
        Ok(sec) => Ok(sec),
        Err(_) => Ok(DocumentSections::Misc),
    }
}

#[cfg(test)]
mod test {
    use super::dispatch_sections;

    #[test]
    fn test_helpers() {
        let mut input = [
            "%BLOCK KPOINTS_LIST",
            "FIX_ALL_CELL  : true",
            "%BLOCK LAttICE_CART",
            "%BLOCK LATTIcE_ABC",
            "%BLOCK POSITIONS_FRAC\n 0.0000",
        ];
        input.iter_mut().for_each(|s| {
            println!("{:?}", dispatch_sections(s));
            println!("Remain:{}", s);
        });
    }
}

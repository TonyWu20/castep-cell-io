use winnow::{
    ascii::{line_ending, till_line_ending},
    combinator::{alt, repeat, terminated},
    error::{ContextError, ErrMode},
    PResult, Parser,
};

use crate::keywords::{DocumentSections, KeywordType};

use self::{
    block::strip_to_block_name,
    fields::field_name,
    keywords::{
        any_block, any_field,
        ionic_positions::assign_positions_type,
        kpoint::{assign_kpoint_block_type, assign_kpoint_field_settings},
        lattice::assign_lattice_type,
        species::assign_species_type,
    },
};

mod block;
mod fields;
mod keywords;

pub use block::get_block_data;
pub use fields::get_field_data;
pub use keywords::{
    ionic_positions::parse_ionic_positions,
    kpoint::{
        parse_bs_kpoint_list, parse_bs_kpoint_path, parse_kpoint_list, parse_kpoint_mp_grid_field,
        parse_kpoint_mp_spacing_field,
    },
    lattice::parse_lattice_param,
    species::{parse_species_lcao_block, parse_species_mass_block, parse_species_pot_block},
};

fn get_keyword<'s>(input: &mut &'s str) -> PResult<KeywordType<'s>> {
    alt((strip_to_block_name, field_name)).parse_next(input)
}

pub fn skip_comments_blank_lines<'s>(input: &mut &'s str) -> PResult<Option<&'s str>> {
    alt((comment_line, line_ending))
        .map(|_| Some("skip"))
        .parse_next(input)
}

fn comment_line<'s>(input: &mut &'s str) -> PResult<&'s str> {
    (alt(('#', '!')), till_line_ending, line_ending)
        .map(|(_, comment, _)| comment)
        .parse_next(input)
}

/// Remove comment contents from the line (all the right to the first '#' or '!')
/// and move to next line
pub fn effective_line<'s>(input: &mut &'s str) -> PResult<&'s str> {
    terminated(till_line_ending, line_ending)
        .map(|s: &str| {
            let pat = |c| c == '#' || c == '!';
            s.split(pat).next().unwrap().trim()
        })
        .parse_next(input)
}

// TODO! Handle `Misc` case to skip the unwanted data
pub fn current_sections<'s>(input: &mut &'s str) -> PResult<DocumentSections<'s>> {
    // Skip the possible comments and blank lines between the blocks and field data.
    repeat(0.., skip_comments_blank_lines).parse_next(input)?;
    let keyword: KeywordType<'_> = get_keyword(input)?;
    let block_keyword_identifiers = (
        assign_lattice_type,
        assign_positions_type,
        assign_kpoint_block_type,
        assign_species_type,
        any_block,
    );
    let field_keyword_identifiers = (assign_kpoint_field_settings, any_field);
    let assign = match keyword {
        KeywordType::Block(block) => alt(block_keyword_identifiers).parse(block),
        KeywordType::Field(name) => alt(field_keyword_identifiers).parse(name),
    };
    if let Ok(sec) = assign {
        Ok(sec)
    } else {
        Err(ErrMode::Cut(ContextError::new()))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        keywords::DocumentSections,
        parsing::helpers::{block::get_block_data, keywords::lattice::parse_lattice_param},
    };

    use super::{current_sections, keywords::ionic_positions::parse_ionic_positions};

    const TEST_DOC: &str = r#"%BLOCK LATTICE_CART
   18.931530020488704480   -0.000000000000003553    0.000000000000000000
   -9.465765010246645517   16.395185930251127360    0.000000000000000000
    0.000000000000000000    0.000000000000000000    9.999213039981000861
%ENDBLOCK LATTICE_CART

%BLOCK POSITIONS_FRAC
  C  0.0756034347004260  0.0756034355668187  0.5000000004346841
  C  0.1496332166229109  0.1496332194727908  0.5000000000710555
  C  0.2145289813410105  0.2145289823390266  0.4999999994942101
  C  0.4243965644332829 -0.0000000008663758  0.5000000004346841
  C  0.3503667805273500 -0.0000000028498315  0.5000000000710555
  C  0.2854710176611356 -0.0000000009979598  0.4999999994942101
  C  0.0000000008664097  0.4243965652996531  0.5000000004346841
  C  0.0000000028498759  0.3503667833771617  0.5000000000710555
  C  0.0000000009980058  0.2854710186590617  0.4999999994942101
  C  0.4243965644332831  0.4243965652997033  0.5000000004346841
  C  0.3503667805273500  0.3503667833772184  0.5000000000710555
  C  0.2854710176611356  0.2854710186591254  0.4999999994942101
  C  0.0000000008664096  0.0756034355668092  0.5000000004346841
  C  0.0000000028498759  0.1496332194727669  0.5000000000710555
  C  0.0000000009980058  0.2145289823389440  0.4999999994942101
  C  0.0756034347004260 -0.0000000008663848  0.5000000004346841
  C  0.1496332166229109 -0.0000000028498642  0.5000000000710555
  C  0.2145289813410104 -0.0000000009979937  0.4999999994942101
  C  0.5756034347004870  0.0756034355668650  0.5000000004346841
  C  0.6496332166229718  0.1496332194728213  0.5000000000710555
  C  0.7145289813411324  0.2145289823390875  0.4999999994942101
  C  0.9243965644334049 -0.0000000008663148  0.5000000004346841
  C  0.8503667805277160 -0.0000000028496485  0.5000000000710555
  C  0.7854710176613185 -0.0000000009978684  0.4999999994942101
  C  0.5000000008665348  0.4243965652997156  0.5000000004346841
  C  0.5000000028499829  0.3503667833772151  0.5000000000710555
  C  0.5000000009980978  0.2854710186591340  0.4999999994942101
  C  0.9243965644334049  0.4243965652997643  0.5000000004346841
  C  0.8503667805277160  0.3503667833774013  0.5000000000710555
  C  0.7854710176613185  0.2854710186592167  0.4999999994942101
  C  0.5000000008665348  0.0756034355668717  0.5000000004346841
  C  0.5000000028499828  0.1496332194728203  0.5000000000710555
  C  0.5000000009980978  0.2145289823390164  0.4999999994942101
  C  0.5756034347004870 -0.0000000008663702  0.5000000004346841
  C  0.6496332166229719 -0.0000000028498338  0.5000000000710555
  C  0.7145289813411323 -0.0000000009978800  0.4999999994942101
  C  0.0756034347004261  0.5756034355668873  0.5000000004346841
  C  0.1496332166229109  0.6496332194729492  0.5000000000710555
  C  0.2145289813410105  0.7145289823390265  0.4999999994942101
  C  0.4243965644332830  0.4999999991336769  0.5000000004346841
  C  0.3503667805273500  0.4999999971502211  0.5000000000710555
  C  0.2854710176611356  0.4999999990020930  0.4999999994942101
  C  0.0000000008664097  0.9243965652998115  0.5000000004346841
  C  0.0000000028498760  0.8503667833773728  0.5000000000710555
  C  0.0000000009980059  0.7854710186589560  0.4999999994942101
  C  0.4243965644332831  0.9243965652998088  0.5000000004346841
  C  0.3503667800000002  0.8503667799999545  0.5000000000000498
  C  0.2854710176611356  0.7854710186589140  0.4999999994942101
  C  0.0000000008664096  0.5756034355668619  0.5000000004346841
  C  0.0000000028498759  0.6496332194727666  0.5000000000710555
  C  0.0000000009980059  0.7145289823387854  0.4999999994942101
  C  0.0756034347004260  0.4999999991336520  0.5000000004346841
  C  0.1496332166229109  0.4999999971501884  0.5000000000710555
  C  0.2145289813410105  0.4999999990020590  0.4999999994942101
  C  0.5756034347004870  0.5756034355669176  0.5000000004346841
  C  0.6496332166229719  0.6496332194728739  0.5000000000710555
  C  0.7145289813411324  0.7145289823390873  0.4999999994942101
  C  0.9243965644334049  0.4999999991337378  0.5000000004346841
  C  0.8503667805277161  0.4999999971504041  0.5000000000710555
  C  0.7854710176613187  0.4999999990021843  0.4999999994942101
  C  0.5000000008665350  0.9243965652995042  0.5000000004346841
  C  0.5000000028499829  0.8503667833770565  0.5000000000710555
  C  0.5000000009980979  0.7854710186591602  0.4999999994942101
  C  0.9243965644334049  0.9243965652998170  0.5000000004346841
  C  0.8503667805277160  0.8503667833774541  0.5000000000710555
  C  0.7854710176613187  0.7854710186592695  0.4999999994942101
  C  0.5000000008665348  0.5756034355669244  0.5000000004346841
  C  0.5000000028499829  0.6496332194728730  0.5000000000710555
  C  0.5000000009980979  0.7145289823390426  0.4999999994942101
  C  0.5756034347004870  0.4999999991336825  0.5000000004346841
  C  0.6496332166229719  0.4999999971502188  0.5000000000710555
  C  0.7145289813411324  0.4999999990021199  0.4999999994942101
  V  0.3934837276229430  0.6065302751523840  0.5001896946183580 SPIN=  2.0000000000
%ENDBLOCK POSITIONS_FRAC

%BLOCK KPOINTS_LIST
  0.0000000000000000  0.0000000000000000  0.0000000000000000  1.0000000000000000
%ENDBLOCK KPOINTS_LIST

FIX_ALL_CELL : true

FIX_COM : false
%BLOCK IONIC_CONSTRAINTS
%ENDBLOCK IONIC_CONSTRAINTS

%BLOCK EXTERNAL_EFIELD
    0.0000000000    0.0000000000    0.0000000000
%ENDBLOCK EXTERNAL_EFIELD

%BLOCK EXTERNAL_PRESSURE
    0.0000000000    0.0000000000    0.0000000000
                    0.0000000000    0.0000000000
                                    0.0000000000
%ENDBLOCK EXTERNAL_PRESSURE

%BLOCK SPECIES_MASS
       C    12.0109996796
       V    50.9410018921
%ENDBLOCK SPECIES_MASS

%BLOCK SPECIES_POT
       C  C_00PBE.usp
       V  V_00PBE.usp
%ENDBLOCK SPECIES_POT

%BLOCK SPECIES_LCAO_STATES
       C        2
       V        5
%ENDBLOCK SPECIES_LCAO_STATES
"#;
    #[test]
    fn test_search() {
        let input = TEST_DOC.to_string();
        let mut input = input.as_str();
        while let Ok(section) = current_sections(&mut input) {
            match section {
                DocumentSections::CellLatticeVectors(lat_type) => {
                    let param = parse_lattice_param(&mut input, lat_type).unwrap();
                    println!("{}", param);
                }
                DocumentSections::IonicPositions(pos_type) => {
                    let positions = parse_ionic_positions(&mut input, pos_type).unwrap();
                    println!("{}", positions);
                }
                _ => {
                    let data = get_block_data(&mut input);
                    println!("{:?}", data);
                }
            }
        }
    }

    #[test]
    fn test_helpers() {
        // let mut input = [
        //     "%BLOCK KPOINTS_LIST\n",
        //     "FIX_ALL_CELL  : true",
        //     "%BLOCK LAttICE_CART\n",
        //     "%BLOCK LATTIcE_ABC\n",
        //     "%BLOCK POSITIONS_FRAC\n 0.0000",
        // ];
        // input.iter_mut().for_each(|s| {
        //     println!("{:?}", get_keyword(s));
        //     println!("Remain:{}", s);
        // });
        let mut block_input = [
            "

# Start
%BLOCK LATTICE_CART
   18.931530020488704480   -0.000000000000003553    0.000000000000000000
   -9.465765010246645517   16.395185930251127360    0.000000000000000000
    0.000000000000000000    0.000000000000000000    9.999213039981000861
%ENDBLOCK LATTICE_CART

",
            "%BLOCK IONIC_CONSTRAINTS
%ENDBLOCK IONIC_CONSTRAINTS

",
            "%BLOCK KPOINTS_LIST
  0.0000000000000000  0.0000000000000000  0.0000000000000000  1.0000000000000000
%ENDBLOCK KPOINTS_LIST

",
        ];
        block_input.iter_mut().for_each(|s| {
            let section = current_sections(s);
            println!("{:?}", section);
            if section.is_ok() {
                let data = get_block_data(s);
                println!("{:?}", data);
                // println!(
                //     "{:?}",
                //     data.split_whitespace()
                //         .filter_map(|s| s.parse::<f64>().ok())
                //         .collect::<Vec<f64>>()
                // );
                // println!("Remain:{}", s);
            }
        });
    }
}

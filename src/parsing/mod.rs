mod cell_parser;
mod error;
pub mod helpers;

pub use error::CellParseError;

use crate::cell_document::{CellEntries, IonicPositionBlock, LatticeParamBlock};

#[derive(Debug)]
pub struct CellParser<'a> {
    input: &'a str,
    lattice_param: Option<LatticeParamBlock>,
    ionic_positions: Option<IonicPositionBlock>,
    other_entries: Vec<CellEntries>,
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, fs, path::Path};

    use pest::Parser;
    use pest_derive::Parser;

    use super::CellParser;

    #[derive(Debug, Parser)]
    #[grammar = "src/parsing/cell.pest"]
    struct CELLParser;

    #[derive(Debug)]
    struct Block<'a> {
        name: &'a str,
        values: Vec<&'a str>,
    }

    #[derive(Debug)]
    struct KeywordValue<'a> {
        name: &'a str,
        value: &'a str,
    }

    #[derive(Debug)]
    enum CELLObject<'a> {
        Block(Block<'a>),
        KeywordValue(KeywordValue<'a>),
    }

    #[test]
    fn test_cell_parser() {
        let root = env!("CARGO_MANIFEST_DIR");
        let path = Path::new(root).join("FePcCOOH_N1_copy.cell");
        let input = fs::read_to_string(path).unwrap();
        let mut cell_parser = CellParser::from(&input);
        let cell_doc = cell_parser.parse();
        println!("Parse status: {:?}", cell_doc.is_ok());
        dbg!(cell_doc.unwrap());
        // let path = Path::new(root).join("SAC_GDY_V_test.cell");
        // let input = fs::read_to_string(path).unwrap();
        // let mut cell_parser = CellParser::from(&input);
        // let cell_doc = cell_parser.parse();
        // println!("Parse status: {:?}", cell_doc.is_ok());
        // println!("{}", cell_doc.unwrap());
    }

    #[test]
    fn test_pest() {
        let block = r#"
%BLOCK LATTICE_CART
      10.182880152352300       0.000000000000000       0.000000000000000
       0.000000000000000       5.969867637928440       0.000000000000000
       0.000000000000000       0.000000000000000       4.750940602435010
%ENDBLOCK LATTICE_CART

%BLOCK POSITIONS_FRAC
  O   0.1643519800967740    0.0300822058464748    0.2768833704631610
  O   0.3348562545299030    0.9636518196508849    0.7820634882457750
  O   0.8326958291482300    0.5301084132249890    0.7199957529591300
  O   0.6610893570101430    0.4734103801569969    0.2259017447033960
  O   0.8337106457826989    0.9698717969773958    0.7183058106151698
  O   0.6612108775813740    0.0454520844934712    0.2127022276070580
  O   0.1635829455251090    0.4686437035660579    0.2756069794332100
  O   0.3391253814473320    0.5275993634085179    0.7857324478518600
  O   0.0516197811641552    0.7485454174406769    0.7192018908206190
  O   0.4445448716519500    0.2504482123651820    0.2335411247263110
  O   0.9476515127699229    0.2492599285336520    0.2740397322044580
  O   0.5552299107568430    0.7602439679064849    0.7830059196944860
  O   0.0923481337996638    0.2499531202263609    0.7668392058183450
  O   0.4106703472528400    0.7435442479936909    0.2751789219973310
  O   0.9064374592598109    0.7505300036295769    0.2296745553664400
 Mg   0.0006199142552149   -0.0004320487921238   -0.0030768499228585
 Mg   0.4994228156871129   -0.0019398234625990    0.4908716202795139
 Mg  -0.0006329882669688    0.4994780040520729   -0.0020497604004188
 Mg   0.2252827903320600    0.7471379623944169    0.4935445584935200
 Mg   0.2782374660305380    0.2475124682348810    0.9955869732606208
 Mg   0.7775179799257270    0.2491749730323970    0.4846031569120610
 Mg   0.7221112967554049    0.7584509571751828    0.0158847583565514
 Si   0.0952635537672570    0.2494188083522390    0.4258637552583270
 Si   0.4071916529256810    0.7490334420514630    0.9342478419267760
 Si   0.9031549548510469    0.7493022976004839    0.5712966407917540
 Si   0.5867373163320350    0.2637625595658440    0.0400795522157690
 Cr   0.5144716596281440    0.4777557343753299    0.5410263903216420
%ENDBLOCK POSITIONS_FRAC

%BLOCK KPOINTS_LIST
   0.0000000000000000    0.2500000000000000    0.3333333333333333       0.333333333333333
   0.0000000000000000    0.2500000000000000    0.0000000000000000       0.333333333333333
   0.0000000000000000    0.2500000000000000   -0.3333333333333333       0.333333333333333
%ENDBLOCK KPOINTS_LIST

%BLOCK CELL_CONSTRAINTS
       1       2       3
       4       5       6
%ENDBLOCK CELL_CONSTRAINTS

FIX_COM : false
%BLOCK IONIC_CONSTRAINTS
%ENDBLOCK IONIC_CONSTRAINTS

%BLOCK EXTERNAL_EFIELD
    0.0000000000     0.0000000000     0.0000000000 
%ENDBLOCK EXTERNAL_EFIELD

%BLOCK SPECIES_MASS
       O     15.9989995956
      Mg     24.3050003052
      Si     28.0849990845
      Cr     51.9959983826
%ENDBLOCK SPECIES_MASS

%BLOCK SPECIES_POT
       O  O_00PBE_OP.recpot
      Mg  Mg_00PBE_OP.recpot
      Si  Si_00PBE_OP.recpot
      Cr  Cr_00.recpot
%ENDBLOCK SPECIES_POT

%BLOCK SPECIES_LCAO_STATES
       O         2
      Mg         4
      Si         2
      Cr         3
%ENDBLOCK SPECIES_LCAO_STATES

SYMMETRY_GENERATE

%BLOCK HUBBARD_U
      Fe       1       d: 0.500000000000000
%ENDBLOCK HUBBARD_U

QUANTIZATION_AXIS :    0.0000    0.0000    1.0000


"#;
        let cell = CELLParser::parse(Rule::cell_doc, block).expect("unsuccessful parse");
        let cell_doc: HashMap<&str, CELLObject> =
            HashMap::from_iter(cell.into_iter().map(|pair| match pair.as_rule() {
                Rule::block => {
                    let inner_rules = pair.into_inner();
                    let block_name = inner_rules
                        .find_first_tagged("block_name")
                        .unwrap()
                        .as_str();
                    let block_lines = inner_rules
                        .find_tagged("block_values")
                        .flat_map(|lines| lines.into_inner().map(|pair| pair.as_str()))
                        .collect::<Vec<&str>>();
                    (
                        block_name,
                        CELLObject::Block(Block {
                            name: block_name,
                            values: block_lines,
                        }),
                    )
                }
                Rule::kv_pair => {
                    let mut inner_rules = pair.into_inner();
                    let name = inner_rules.next().unwrap().as_str();
                    let value = inner_rules.next().unwrap().as_str();
                    (name, CELLObject::KeywordValue(KeywordValue { name, value }))
                }
                Rule::single_keywords => {
                    let name = pair.as_str();
                    let value = "true";
                    (name, CELLObject::KeywordValue(KeywordValue { name, value }))
                }
                _ => unreachable!(),
            }));
        cell_doc.keys().for_each(|key| println!("{key}"));
    }
    #[test]
    fn test_kv_pair() {
        let kv_pair = "FIX_COM : false";
        let single_keyword = "SYMMETRY_GENERATE";
        let kv = CELLParser::parse(Rule::kv_pair, kv_pair).expect("unsuccessful parse");
        dbg!(kv);
        let single = CELLParser::parse(Rule::kv_pair, single_keyword).expect("unsuccessful parse");
        dbg!(single);
    }
    #[test]
    fn test_comment() {
        let comment = "# COMMENT";
        let comm = CELLParser::parse(Rule::COMMENT, comment).expect("unsuccessful parse");
        dbg!(comm);
    }
}

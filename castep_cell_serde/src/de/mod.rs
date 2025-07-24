mod cell;
mod primitive;
pub use cell::{from_str, from_tokens};
#[cfg(test)]
mod de_test {

    use std::fs::read_to_string;

    use serde::Deserialize;

    use crate::{
        Cell, CellValue,
        de::{
            cell::{from_str, from_tokens},
            primitive::CellValueDeserializer,
        },
    };

    #[derive(Debug, Deserialize, PartialEq)]
    pub struct Kpoint {
        kx: f64,
        ky: f64,
        kz: f64,
        weight: f64,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    #[serde(from = "IonicPositionRepr")]
    pub struct IonicPosition {
        element: String,
        coord: [f64; 3],
        spin: Option<f64>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename = "POSITIONS_FRAC")]
    #[serde(from = "PositionsFracRepr")]
    // #[serde(transparent)]
    pub struct PositionsFrac {
        pub unit: Option<String>,
        pub positions: Vec<IonicPosition>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum PositionsFracRepr<'a> {
        Essential(Vec<IonicPosition>),
        WithUnit(&'a str, Vec<IonicPosition>),
    }

    impl<'a> From<PositionsFracRepr<'a>> for PositionsFrac {
        fn from(value: PositionsFracRepr<'a>) -> Self {
            match value {
                PositionsFracRepr::Essential(ionic_positions) => Self {
                    unit: None,
                    positions: ionic_positions,
                },
                PositionsFracRepr::WithUnit(unit, ionic_positions) => Self {
                    unit: Some(unit.to_string()),
                    positions: ionic_positions,
                },
            }
        }
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum IonicPositionRepr<'a> {
        Essential(&'a str, f64, f64, f64),
        WithSpin(&'a str, f64, f64, f64, &'a str, f64),
    }

    impl<'a> From<IonicPositionRepr<'a>> for IonicPosition {
        fn from(value: IonicPositionRepr) -> Self {
            match value {
                IonicPositionRepr::Essential(elm, x, y, z) => Self {
                    element: elm.to_string(),
                    coord: [x, y, z],
                    spin: None,
                },
                IonicPositionRepr::WithSpin(elm, x, y, z, _spin_str, spin) => Self {
                    element: elm.to_string(),
                    coord: [x, y, z],
                    spin: Some(spin),
                },
            }
        }
    }

    #[derive(Debug, Deserialize)]
    struct FixCOM(bool);

    #[test]
    fn line_de() {
        let kpoint_line = CellValue::Array(vec![
            CellValue::Float(0.0),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
            CellValue::Float(1.0),
        ]);
        let kpoint = Kpoint::deserialize(&mut CellValueDeserializer::new(&kpoint_line)).unwrap();
        dbg!(&kpoint);
        debug_assert_eq!(
            kpoint,
            Kpoint {
                kx: 0.0,
                ky: 0.0,
                kz: 0.0,
                weight: 1.0
            }
        );
        let ion_line = CellValue::Array(vec![
            CellValue::Str("O"),
            CellValue::Float(0.5),
            CellValue::Float(0.5),
            CellValue::Float(0.5),
            CellValue::Str("SPIN="),
            CellValue::Float(1.0),
        ]);
        let positions = vec![
            ion_line.clone(),
            CellValue::Array(vec![
                CellValue::Str("O"),
                CellValue::Float(0.25),
                CellValue::Float(0.25),
                CellValue::Float(0.25),
                CellValue::Str("SPIN="),
                CellValue::Float(1.0),
            ]),
        ];
        let ion = IonicPosition::deserialize(&mut CellValueDeserializer::new(&ion_line)).unwrap();
        dbg!(ion);

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        struct PositionsFracBlock {
            positions_frac: PositionsFrac,
        }
        let positions_block_from_file =
            from_tokens::<PositionsFracBlock>(&[Cell::Block("POSITIONS_FRAC", positions)]);
        dbg!(positions_block_from_file.unwrap());
        let fix_com = FixCOM::deserialize(&mut CellValueDeserializer::new(&CellValue::Bool(false)));
        dbg!(fix_com.unwrap());
    }
    const ONLY_BLOCKS: &str = r#"
%BLOCK CELL_CONSTRAINTS
       1       2       3
       4       5       6
%ENDBLOCK CELL_CONSTRAINTS


%BLOCK POSITIONS_FRAC
     O   0.1635419733526620    0.0317792047151180    0.2751746346719976
     O   0.3354045184454477    0.9672373612661035    0.7746824750061752
     O   0.8364477955763916    0.5313805688511324    0.7241701610821136
     O   0.6653182432350798    0.4679952694597609    0.2187534484550325
     O   0.0539200916667086    0.7500362471704362    0.7220278753304680
     O   0.4453965167140691    0.2440160309838545    0.2128379221037726
     O   0.0918663259131503    0.2498358217910210    0.7673845460556011
     O   0.4059051594219431    0.7518455464779391    0.2698918585108838
     O  -0.1635419733526620   -0.0317792047151180   -0.2751746346719977
     O  -0.3354045184454477   -0.9672373612661035   -0.7746824750061753
     O  -0.8364477955763918   -0.5313805688511324   -0.7241701610821137
     O  -0.6653182432350799   -0.4679952694597609   -0.2187534484550325
     O  -0.0539200916667086   -0.7500362471704362   -0.7220278753304681
     O  -0.4453965167140692   -0.2440160309838544   -0.2128379221037725
     O  -0.0918663259131503   -0.2498358217910209   -0.7673845460556012
     O  -0.4059051594219432   -0.7518455464779391   -0.2698918585108838
    Mg   0.2227159254607965    0.7504470916631593    0.4926066816971608
    Mg   0.2757366027392655    0.2484214504543120    0.9892634813029080
    Mg  -0.2227159254607965   -0.7504470916631593   -0.4926066816971609
    Mg  -0.2757366027392656   -0.2484214504543120   -0.9892634813029079
    Mg   0.0000000000000000    0.0000000000000000    0.0000000000000000
    Mg   0.5000000000000000    0.0000000000000000    0.5000000000000000
    Mg   0.0000000000000000    0.5000000000000000    0.0000000000000000
    Si   0.0942200147741907    0.2501820112892181    0.4262683890462294
    Si   0.4056313552411618    0.7503866208268292    0.9270250152159749
    Si  -0.0942200147741907   -0.2501820112892181   -0.4262683890462295
    Si  -0.4056313552411619   -0.7503866208268292   -0.9270250152159748
    Cr   0.5000000000000000    0.5000000000000000    0.5000000000000000 SPIN= 1.0000
%ENDBLOCK POSITIONS_FRAC

FIX_COM : true

%BLOCK KPOINTS_LIST
   0.0000000000000000    0.2500000000000000    0.3333333333333333       0.333333333333333
   0.0000000000000000    0.2500000000000000    0.0000000000000000       0.333333333333333
   0.0000000000000000    0.2500000000000000   -0.3333333333333333       0.333333333333333
%ENDBLOCK KPOINTS_LIST
"#;

    #[derive(Debug, Deserialize)]
    struct CellConstraints {
        lat_params: [u32; 3],
        angles: [u32; 3],
    }

    #[derive(Debug, Deserialize)]
    #[serde(transparent)]
    struct KpointsList {
        kpts: Vec<Kpoint>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    struct CellFile {
        lattice_cart: LatticeCart,
        positions_frac: PositionsFrac,
        cell_constraints: CellConstraints,
        fix_com: FixCOM,
        kpoints_list: KpointsList,
    }

    #[derive(Debug, Deserialize)]
    struct LatticeCart {
        a: [f64; 3],
        b: [f64; 3],
        c: [f64; 3],
    }

    #[test]
    fn top_de() {
        let example = read_to_string("Mg2SiO4_Cr_1.cell").unwrap();
        // let parsed = parse_cell_file(&example)
        //     .map_err(|errors| {
        //         errors
        //             .iter()
        //             .for_each(|e| rich_error(e, "Mg2SiO4_Cr_1.cell", &example));
        //     })
        //     .unwrap();
        // let cell_file = CellFile::deserialize(&mut CellFileDeserializer::new(&parsed));
        let cell_file = from_str::<CellFile>(&example);
        dbg!(cell_file.unwrap());
    }
}

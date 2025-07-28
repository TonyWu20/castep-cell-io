#![allow(dead_code)]
mod de;
mod error;
mod parser;
mod ser;

pub use de::{from_str, from_tokens};
pub use error::{CResult, Error};
pub use parser::parse_cell_file;
pub use parser::rich_error;
pub use ser::to_string;

// Intermediate representation for parsed data
#[derive(Debug, Clone)]
pub enum Cell<'a> {
    KeyValue(&'a str, CellValue<'a>),
    Block(&'a str, Vec<CellValue<'a>>),
    Flag(&'a str),
}

impl<'a> Cell<'a> {
    pub fn key(&self) -> &str {
        match self {
            Cell::KeyValue(key, _cell_value) => key,
            Cell::Block(name, _cell_value) => name,
            Cell::Flag(flag) => flag,
        }
    }
}

pub trait ToCellValue {
    fn to_cell_value(&self) -> CellValue;
}

pub trait ToCell {
    fn to_cell(&self) -> Cell;
}

pub trait ToCellFile {
    fn to_cell_file(&self) -> Vec<Cell>;
}

#[derive(Debug, Clone)]
pub enum CellValue<'a> {
    Null,
    Bool(bool),
    Str(&'a str),
    String(String),
    UInt(u32),
    Int(i32),
    Float(f64),
    Array(Vec<CellValue<'a>>),
}

impl<'a> CellValue<'a> {
    pub fn as_array(&self) -> Option<&Vec<CellValue<'a>>> {
        if let Self::Array(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
#[cfg(test)]
mod test {

    use crate::{ToCell, ToCellValue, de::CellValueDeserializer, to_string};
    use std::fs::read_to_string;

    use serde::{Deserialize, Serialize};

    use crate::{Cell, CellValue, from_str, from_tokens};
    #[derive(Debug, Deserialize, Serialize)]
    /// Block
    struct CellConstraints {
        lat_params: [u32; 3],
        angles: [u32; 3],
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(transparent)]
    /// Block
    struct KpointsList {
        kpts: Vec<Kpoint>,
    }

    impl ToCell for KpointsList {
        fn to_cell(&self) -> Cell {
            Cell::Block(
                "KPOINTS_LIST",
                self.kpts
                    .iter()
                    .map(|kpt| kpt.to_cell_value())
                    .collect::<Vec<CellValue>>(),
            )
        }
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    /// A line of block `KpointsList`
    #[serde(from = "[f64;4]")]
    pub struct Kpoint {
        coord: [f64; 3],
        weight: f64,
    }

    // struct KpointRepr([f64; 4]);

    impl From<[f64; 4]> for Kpoint {
        fn from(value: [f64; 4]) -> Self {
            Kpoint {
                coord: value[0..3].try_into().unwrap(),
                weight: value[3],
            }
        }
    }

    impl ToCellValue for Kpoint {
        fn to_cell_value(&self) -> CellValue {
            CellValue::Array(
                [
                    self.coord.into_iter().map(CellValue::Float).collect(),
                    vec![CellValue::Float(self.weight)],
                ]
                .concat(),
            )
        }
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
    /// Block
    #[serde(from = "LatticeCartRepr")]
    struct LatticeCart {
        #[serde(flatten)]
        unit: Option<LengthUnit>,
        a: [f64; 3],
        b: [f64; 3],
        c: [f64; 3],
    }
    #[derive(Debug, Clone, Copy, Hash, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
    // #[serde(untagged)]
    #[serde(rename_all = "lowercase")]
    pub enum LengthUnit {
        Bohr,
        #[serde(alias = "a0")]
        BohrA0,
        #[serde(rename = "m")]
        Meter,
        #[serde(rename = "cm")]
        Centimeter,
        #[serde(rename = "nm")]
        Nanometer,
        #[default]
        #[serde(rename = "ang")]
        Ang,
    }

    impl ToCellValue for LengthUnit {
        fn to_cell_value(&self) -> CellValue {
            match self {
                LengthUnit::Bohr => CellValue::Str("bohr"),
                LengthUnit::BohrA0 => CellValue::Str("a0"),
                LengthUnit::Meter => CellValue::Str("m"),
                LengthUnit::Centimeter => CellValue::Str("cm"),
                LengthUnit::Nanometer => CellValue::Str("nm"),
                LengthUnit::Ang => CellValue::Str("ang"),
            }
        }
    }

    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum LatticeCartRepr {
        Essential([[f64; 3]; 3]),
        WithUnit([LengthUnit; 1], [f64; 3], [f64; 3], [f64; 3]),
    }

    impl From<LatticeCartRepr> for LatticeCart {
        fn from(value: LatticeCartRepr) -> Self {
            match value {
                LatticeCartRepr::Essential(items) => LatticeCart {
                    unit: None,
                    a: items[0],
                    b: items[1],
                    c: items[2],
                },
                LatticeCartRepr::WithUnit(unit, a, b, c) => LatticeCart {
                    unit: Some(unit[0]),
                    a,
                    b,
                    c,
                },
            }
        }
    }

    impl ToCell for LatticeCart {
        fn to_cell(&self) -> Cell {
            Cell::Block(
                "LATTICE_CART",
                vec![
                    match &self.unit {
                        Some(u) => CellValue::Array(vec![u.to_cell_value()]),
                        None => CellValue::Null,
                    },
                    CellValue::Array(self.a.into_iter().map(CellValue::Float).collect()),
                    CellValue::Array(self.b.into_iter().map(CellValue::Float).collect()),
                    CellValue::Array(self.c.into_iter().map(CellValue::Float).collect()),
                ],
            )
        }
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
    #[serde(transparent)]
    pub struct PositionsFrac {
        pub positions: Vec<IonicPosition>,
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

    #[derive(Debug, Deserialize, Serialize)]
    struct FixCOM(bool);

    #[derive(Debug, Deserialize)]
    struct SymmetryTol(f64, LengthUnit);

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
                coord: [0.0, 0.0, 0.0],
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
        let positions_block_from_str = from_str::<PositionsFracBlock>(
            r#"%BLOCK POSITIONS_FRAC
     O   0.1635419733526620    0.0317792047151180    0.2751746346719976
     O   0.3354045184454477    0.9672373612661035    0.7746824750061752
     O   0.8364477955763916    0.5313805688511324    0.7241701610821136
%ENDBLOCK POSITIONS_FRAC"#,
        )
        .unwrap();
        dbg!(positions_block_from_str);
        let fix_com = FixCOM::deserialize(&mut CellValueDeserializer::new(&CellValue::Bool(false)));
        dbg!(fix_com.unwrap());
        let unit = Vec::<LengthUnit>::deserialize(&mut CellValueDeserializer::new(
            &CellValue::Array(vec![CellValue::Str("bohr")]),
        ));
        dbg!(unit.unwrap());
        let sym_tol =
            SymmetryTol::deserialize(&mut CellValueDeserializer::new(&CellValue::Array(vec![
                CellValue::Float(0.01),
                CellValue::Str("ang"),
            ])));
        dbg!(sym_tol.unwrap());
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
        let cell_file = from_str::<CellFile>(&example).unwrap();
        dbg!(&cell_file);
        let kpt_list_string = to_string(&cell_file.kpoints_list.to_cell()).unwrap();
        println!("{kpt_list_string}");
        println!("{}", to_string(&cell_file.lattice_cart.to_cell()).unwrap());
    }
}

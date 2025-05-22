use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Orbital {
    S,
    P,
    D,
    F,
}

impl Display for Orbital {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Orbital::S => "s",
                Orbital::P => "p",
                Orbital::D => "d",
                Orbital::F => "f",
            }
        )
    }
}

pub mod hubbard_u {
    use std::fmt::Display;

    use castep_periodic_table::element::ElementSymbol;
    use derive_builder::Builder;

    use crate::parsing::{BlockBuilder, BlockIO};

    use super::Orbital;

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Builder)]
    #[builder()]
    pub struct HubbardUItem {
        element: ElementSymbol,
        atom_id: Option<usize>,
        orbital: Orbital,
        u_value: f64,
    }

    impl Display for HubbardUItem {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{:>4} {} {}: {:>20.15}",
                format!("{:?}", self.element),
                self.atom_id.map_or(String::new(), |v| format!("{v}")),
                self.orbital,
                self.u_value
            )
        }
    }

    impl HubbardUItem {
        pub fn element(&self) -> ElementSymbol {
            self.element
        }

        pub fn element_mut(&mut self) -> &mut ElementSymbol {
            &mut self.element
        }

        pub fn set_element(&mut self, element: ElementSymbol) {
            self.element = element;
        }

        pub fn atom_id(&self) -> Option<usize> {
            self.atom_id
        }

        pub fn atom_id_mut(&mut self) -> &mut Option<usize> {
            &mut self.atom_id
        }

        pub fn set_atom_id(&mut self, atom_id: Option<usize>) {
            self.atom_id = atom_id;
        }

        pub fn orbital(&self) -> Orbital {
            self.orbital
        }

        pub fn orbital_mut(&mut self) -> &mut Orbital {
            &mut self.orbital
        }

        pub fn set_orbital(&mut self, orbital: Orbital) {
            self.orbital = orbital;
        }

        pub fn u_value(&self) -> f64 {
            self.u_value
        }

        pub fn u_value_mut(&mut self) -> &mut f64 {
            &mut self.u_value
        }

        pub fn set_u_value(&mut self, u_value: f64) {
            self.u_value = u_value;
        }
    }

    #[derive(Debug, Clone, PartialEq, PartialOrd)]
    pub struct HubbardUBlock {
        // unit: EnergyUnit
        settings: Vec<HubbardUItem>,
    }

    impl HubbardUBlock {
        pub fn new(settings: Vec<HubbardUItem>) -> Self {
            Self { settings }
        }

        pub fn settings(&self) -> &[HubbardUItem] {
            &self.settings
        }

        pub fn settings_mut(&mut self) -> &mut Vec<HubbardUItem> {
            &mut self.settings
        }

        pub fn set_settings(&mut self, settings: Vec<HubbardUItem>) {
            self.settings = settings;
        }
    }

    impl BlockIO for HubbardUBlock {
        type Item = HubbardUBlock;

        fn from_block(block: &crate::parsing::Block) -> Result<Self::Item, crate::CellParseError> {
            if block.name().to_lowercase().as_str() != "hubbard_u" {
                return Err(crate::CellParseError::UnexpectedBlockType((
                    "HUBBARD_U".to_string(),
                    block.name().to_string(),
                )));
            }
            todo!()
        }

        fn to_block(&self, order: usize) -> crate::parsing::Block {
            BlockBuilder::default()
                .order(order)
                .name("HUBBARD_U".to_string())
                .values(
                    self.settings()
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>(),
                )
                .build()
                .unwrap()
        }
    }
    #[cfg(test)]
    mod test {
        use std::str::FromStr;

        use castep_periodic_table::element::ElementSymbol;
        use pest::Parser;

        use crate::{
            cell_document::sections::hubbard::{
                hubbard_u::{HubbardUItem, HubbardUItemBuilder},
                Orbital,
            },
            parsing::{BlockIO, CELLParser, Rule},
        };

        use super::HubbardUBlock;

        #[test]
        fn test_hubbard_u() {
            let hubbard_u_line = "    U 2  d: 1.2 f: 2.1";
            let parsed = CELLParser::parse(Rule::hubbard_u_line, hubbard_u_line).unwrap();
            let atom_id = parsed
                .find_first_tagged("atom_id")
                .and_then(|p| p.as_str().parse::<usize>().ok());
            let element_symbol = parsed
                .find_first_tagged("symbol")
                .and_then(|p| ElementSymbol::from_str(p.as_str()).ok())
                .unwrap();
            let values = parsed
                .filter_map(|pair| {
                    if matches!(pair.as_rule(), Rule::hubbard_u_value) {
                        let mut inner = pair.into_inner();
                        let orbital = inner
                            .next()
                            .and_then(|orbital| match orbital.as_str().to_lowercase().as_str() {
                                "s" => Some(Orbital::S),
                                "p" => Some(Orbital::P),
                                "d" => Some(Orbital::D),
                                "f" => Some(Orbital::F),
                                _ => None,
                            })
                            .unwrap();
                        let value = inner
                            .next()
                            .and_then(|value| value.as_str().parse::<f64>().ok())
                            .unwrap();
                        Some(
                            HubbardUItemBuilder::default()
                                .element(element_symbol)
                                .orbital(orbital)
                                .atom_id(atom_id)
                                .u_value(value)
                                .build()
                                .unwrap(),
                        )
                    } else {
                        None
                    }
                })
                .collect::<Vec<HubbardUItem>>();
            values.iter().for_each(|item| println!("{item}"));
            let hubbard_u_block = HubbardUBlock::new(values);
            println!("{}", hubbard_u_block.to_block(1).to_string());
        }
    }
}

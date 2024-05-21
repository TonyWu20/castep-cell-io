use std::fmt::Display;

use castep_periodic_table::{
    data::ELEMENT_TABLE,
    element::{ElementSymbol, LookupElement},
};

use crate::{formatting::BlockDisplay, keywords::PositionsKeywords};

use super::LengthUnit;

#[derive(Debug, Clone)]
pub struct IonicPositionBlock {
    unit: LengthUnit,
    positions: Vec<IonicPosition>,
    keyword: PositionsKeywords,
    spin_polarised: bool,
}

impl Display for IonicPositionBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl IonicPositionBlock {
    pub fn new(
        unit: LengthUnit,
        positions: Vec<IonicPosition>,
        keyword: PositionsKeywords,
        spin_polarised: bool,
    ) -> Self {
        Self {
            unit,
            positions,
            keyword,
            spin_polarised,
        }
    }

    pub fn positions(&self) -> &[IonicPosition] {
        self.positions.as_ref()
    }

    pub fn keyword(&self) -> PositionsKeywords {
        self.keyword
    }

    pub fn spin_polarised(&self) -> bool {
        self.spin_polarised
    }

    pub fn unit(&self) -> LengthUnit {
        self.unit
    }

    pub fn positions_mut(&mut self) -> &mut Vec<IonicPosition> {
        &mut self.positions
    }
}

impl BlockDisplay for IonicPositionBlock {
    fn block_tag(&self) -> String {
        format!("{:?}", self.keyword)
    }

    fn entries(&self) -> String {
        self.positions
            .iter()
            .map(|p| p.export(self.spin_polarised))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IonicPosition {
    symbol: ElementSymbol,
    coordinate: [f64; 3],
    mixture: Option<Mixture>,
}

#[derive(Debug, Clone, Copy)]
pub struct Mixture(usize, f64);

impl Display for Mixture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MIXTURE:{:>5}{:10.6})", self.0, self.1)
    }
}

impl Mixture {
    pub fn new(id: usize, val: f64) -> Self {
        Mixture(id, val)
    }
}

impl IonicPosition {
    pub fn new(symbol: ElementSymbol, coordinate: [f64; 3], mixture: Option<Mixture>) -> Self {
        Self {
            symbol,
            coordinate,
            mixture,
        }
    }

    pub fn coordinate(&self) -> [f64; 3] {
        self.coordinate
    }

    pub fn symbol(&self) -> ElementSymbol {
        self.symbol
    }

    pub fn mixture(&self) -> Option<Mixture> {
        self.mixture
    }
    pub fn export(&self, spin_polarised: bool) -> String {
        let coordinates = self
            .coordinate
            .iter()
            .map(|v| format!("{v:20.16}"))
            .collect::<Vec<String>>()
            .concat();
        let element = ELEMENT_TABLE.get_by_symbol(self.symbol()).unwrap();
        let spin = if spin_polarised && element.spin > 0 {
            format!(
                " SPIN={:14.10}",
                ELEMENT_TABLE.get_by_symbol(self.symbol()).unwrap().spin() as f64
            )
        } else {
            "".into()
        };
        let mixture = if let Some(mix) = self.mixture {
            format!("{}", mix)
        } else {
            "".into()
        };

        let line = format!("{:>3}{}{}{}", element.symbol(), coordinates, spin, mixture);
        line
    }
}

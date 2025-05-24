use derive_builder::Builder;
use pest::Parser;

use crate::{
    cell_document::units::ParsableUnit,
    parsing::{Block, BlockBuilder, BlockIO, CELLParser, Rule},
    CellParseError, LengthUnit,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Builder, Default)]
#[builder(default)]
/// Struct corresponds to the `lattice_cart` block in `.cell`
pub struct LatticeCart {
    unit: LengthUnit,
    a: [f64; 3],
    b: [f64; 3],
    c: [f64; 3],
}

impl LatticeCart {
    pub fn unit(&self) -> LengthUnit {
        self.unit
    }

    pub fn set_unit(&mut self, unit: LengthUnit) {
        self.unit = unit;
    }

    pub fn a(&self) -> [f64; 3] {
        self.a
    }

    pub fn set_a(&mut self, a: [f64; 3]) {
        self.a = a;
    }

    pub fn b(&self) -> [f64; 3] {
        self.b
    }

    pub fn set_b(&mut self, b: [f64; 3]) {
        self.b = b;
    }

    pub fn c(&self) -> [f64; 3] {
        self.c
    }

    pub fn set_c(&mut self, c: [f64; 3]) {
        self.c = c;
    }
}

impl BlockIO for LatticeCart {
    type Item = LatticeCart;

    fn from_block(block: &Block) -> Result<Self::Item, CellParseError> {
        if block.name().to_lowercase() != "lattice_cart" {
            return Err(CellParseError::UnexpectedBlockType((
                "lattice_cart".to_string(),
                block.name().to_string(),
            )));
        }
        let unit = if block.values().len() == 4 {
            LengthUnit::parse_from_str(block.values().first().unwrap().as_str())?
        } else {
            LengthUnit::default()
        };
        let vector_lines = if block.values().len() == 3 {
            block.values().iter()
        } else {
            let mut v = block.values().iter();
            v.next();
            v
        };
        let [a, b, c]: [[f64; 3]; 3] = vector_lines
            .map(|line| -> [f64; 3] {
                CELLParser::parse(Rule::lattice_cart_value, line)
                    .unwrap()
                    .map(|p| p.as_str().parse::<f64>().unwrap())
                    .collect::<Vec<f64>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[f64; 3]>>()
            .try_into()
            .unwrap();
        Ok(LatticeCartBuilder::default()
            .unit(unit)
            .a(a)
            .b(b)
            .c(c)
            .build()
            .expect("all fields initialized"))
    }

    fn to_block(&self, order: usize) -> Block {
        let unit = match self.unit {
            LengthUnit::Ang => None,
            _ => Some(self.unit.to_string()),
        };
        let [a, b, c] = [self.a, self.b, self.c]
            .map(|v| Some(v.map(|value| format!("{:>20.15}", value)).join(" ")));
        let value = [unit, a, b, c]
            .into_iter()
            .flatten()
            .collect::<Vec<String>>();
        BlockBuilder::default()
            .name("LATTICE_CART".to_string())
            .order(order)
            .values(value)
            .build()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use pest::Parser;

    use crate::parsing::{Block, BlockIO, CELLParser, Rule};

    use super::LatticeCart;

    #[test]
    fn test_lattice_cart_blk() {
        let block = r#"%BLOCK LATTICE_CART
            Bohr
      10.182880152352300       0.000000000000000       0.000000000000000
       0.000000000000000       5.969867637928440       0.000000000000000
       0.000000000000000       0.000000000000000       4.750940602435010
%ENDBLOCK LATTICE_CART
"#;
        let block = CELLParser::parse(Rule::block, block)
            .unwrap()
            .next()
            .map(|p| Block::from_pair(p, 0))
            .unwrap();
        let lattice_cart = LatticeCart::from_block(&block).unwrap();
        dbg!(lattice_cart.to_block(0));
    }
}

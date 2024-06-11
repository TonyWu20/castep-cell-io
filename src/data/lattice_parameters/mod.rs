use std::fmt::Display;

use crate::{data::units::Degrees, formatting::BlockDisplay, LengthUnit};

use super::CellData;

#[derive(Debug, Clone, Copy)]
pub struct LatticeParamBlock {
    unit: LengthUnit,
    parameter: LatticeParam,
}

impl Display for LatticeParamBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl BlockDisplay for LatticeParamBlock {
    fn block_tag(&self) -> String {
        match self.parameter {
            LatticeParam::LatticeABC(_) => "LATTICE_ABC".to_string(),
            LatticeParam::LatticeCart(_) => "LATTICE_CART".to_string(),
        }
    }

    fn entries(&self) -> String {
        let param = match self.parameter {
            LatticeParam::LatticeCart(v) => format!("{v}"),
            LatticeParam::LatticeABC(v) => format!("{v}"),
        };
        if let LengthUnit::Ang = self.unit {
            param
        } else {
            format!("{}\n{}", self.unit, param)
        }
    }
}

impl LatticeParamBlock {
    pub fn new(unit: LengthUnit, parameter: LatticeParam) -> Self {
        Self { unit, parameter }
    }

    pub fn unit(&self) -> LengthUnit {
        self.unit
    }

    pub fn parameter(&self) -> LatticeParam {
        self.parameter
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LatticeParam {
    LatticeCart(LatticeCart),
    LatticeABC(LatticeABC),
}

#[derive(Debug, Clone, Copy)]
pub struct LatticeCart {
    a: [f64; 3],
    b: [f64; 3],
    c: [f64; 3],
}

impl Display for LatticeCart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line_a = self
            .a
            .iter()
            .map(|v| format!("{v:24.18}"))
            .collect::<Vec<String>>()
            .join(" ");
        let line_b = self
            .b
            .iter()
            .map(|v| format!("{v:24.18}"))
            .collect::<Vec<String>>()
            .join(" ");
        let line_c = self
            .c
            .iter()
            .map(|v| format!("{v:24.18}"))
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{}", [line_a, line_b, line_c].join("\n"))
    }
}

impl LatticeCart {
    pub fn new(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> Self {
        Self { a, b, c }
    }

    pub fn a(&self) -> [f64; 3] {
        self.a
    }

    pub fn b(&self) -> [f64; 3] {
        self.b
    }

    pub fn c(&self) -> [f64; 3] {
        self.c
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LatticeABC {
    a: f64,
    b: f64,
    c: f64,
    alpha: Degrees,
    beta: Degrees,
    gamma: Degrees,
}

impl Display for LatticeABC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let abc = [self.a, self.b, self.c]
            .iter()
            .map(|v| format!("{v:24.18}"))
            .collect::<Vec<String>>()
            .join(" ");
        let angles = [self.alpha, self.beta, self.gamma]
            .iter()
            .map(|v| format!("{:24.18}", v.value()))
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{}", [abc, angles].join("\n"))
    }
}

impl LatticeABC {
    pub fn new(a: f64, b: f64, c: f64, alpha: f64, beta: f64, gamma: f64) -> Self {
        let alpha = Degrees::new(alpha);
        let beta = Degrees::new(beta);
        let gamma = Degrees::new(gamma);
        Self {
            a,
            b,
            c,
            alpha,
            beta,
            gamma,
        }
    }

    pub fn a(&self) -> f64 {
        self.a
    }

    pub fn b(&self) -> f64 {
        self.b
    }

    pub fn c(&self) -> f64 {
        self.c
    }

    pub fn alpha(&self) -> Degrees {
        self.alpha
    }

    pub fn beta(&self) -> Degrees {
        self.beta
    }

    pub fn gamma(&self) -> Degrees {
        self.gamma
    }
}

/// Marker trait implmentation
impl CellData for LatticeParam {}

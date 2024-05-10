use super::CellData;

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
    alpha: f64,
    beta: f64,
    gamma: f64,
}

impl LatticeABC {
    pub fn new(a: f64, b: f64, c: f64, alpha: f64, beta: f64, gamma: f64) -> Self {
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

    pub fn alpha(&self) -> f64 {
        self.alpha
    }

    pub fn beta(&self) -> f64 {
        self.beta
    }

    pub fn gamma(&self) -> f64 {
        self.gamma
    }
}

/// Marker trait implmentation
impl CellData for LatticeParam {}

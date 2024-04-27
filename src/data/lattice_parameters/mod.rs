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
}

/// Marker trait implmentation
impl CellData for LatticeParam {}

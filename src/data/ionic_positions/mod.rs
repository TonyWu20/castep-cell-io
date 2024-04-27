use castep_periodic_table::element::ElementSymbol;

#[derive(Debug, Clone, Copy)]
pub struct IonicPosition {
    symbol: ElementSymbol,
    coordinate: [f64; 3],
    mixture: Option<Mixture>,
}

#[derive(Debug, Clone, Copy)]
pub struct Mixture(usize, f64);

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
}

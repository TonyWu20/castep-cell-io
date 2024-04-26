use castep_periodic_table::element::ElementSymbol;

#[derive(Debug)]
pub struct IonicPosition {
    symbol: ElementSymbol,
    coordinate: [f64; 3],
    spin: Option<u8>,
    mixture: Option<Mixture>,
}

#[derive(Debug)]
pub struct Mixture(usize, f64);

impl IonicPosition {
    pub fn new(
        symbol: ElementSymbol,
        coordinate: [f64; 3],
        spin: Option<u8>,
        mixture: Option<Mixture>,
    ) -> Self {
        Self {
            symbol,
            coordinate,
            spin,
            mixture,
        }
    }
}

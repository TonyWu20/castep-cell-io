#![allow(dead_code)]
use data::{IonicPosition, LatticeParam};

mod data;
mod keywords;
mod parsing;

#[derive(Debug)]
pub struct CellDocument {
    lattice: LatticeParam,
    ionic_positions: IonicPosition,
}

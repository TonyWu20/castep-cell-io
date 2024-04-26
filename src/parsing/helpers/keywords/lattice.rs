use winnow::{ascii::Caseless, combinator::alt, PResult, Parser};

use crate::keywords::{CellLatticeVectorsKeywords, DocumentSections};

fn assign_lattice_cart<'s>(input: &mut &'s str) -> PResult<DocumentSections> {
    Caseless("lattice_cart")
        .map(|_| DocumentSections::CellLatticeVectors(CellLatticeVectorsKeywords::LATTICE_CART))
        .parse_next(input)
}

fn assign_lattice_abc<'s>(input: &mut &'s str) -> PResult<DocumentSections> {
    Caseless("lattice_abc")
        .map(|_| DocumentSections::CellLatticeVectors(CellLatticeVectorsKeywords::LATTICE_ABC))
        .parse_next(input)
}

pub fn assign_lattice_type<'s>(input: &mut &'s str) -> PResult<DocumentSections> {
    alt((assign_lattice_abc, assign_lattice_cart)).parse_next(input)
}

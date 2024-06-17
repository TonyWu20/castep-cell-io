use std::fmt::Display;

use castep_periodic_table::element::ElementSymbol;

use crate::formatting::{BlockDisplay, FieldDisplay};

#[derive(Debug, Clone, Copy)]
pub struct CellConstraints {
    a: u32,
    b: u32,
    c: u32,
    alpha: u32,
    beta: u32,
    gamma: u32,
}

impl CellConstraints {
    pub fn new(a: u32, b: u32, c: u32, alpha: u32, beta: u32, gamma: u32) -> Self {
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

impl Default for CellConstraints {
    fn default() -> Self {
        let [a, b, c, alpha, beta, gamma] = [1, 2, 3, 0, 0, 0];
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

#[derive(Default, Debug, Clone, Copy)]
pub struct FixAllCell {
    is_fix: bool,
}

impl FixAllCell {
    pub fn new(is_fix: bool) -> Self {
        Self { is_fix }
    }

    pub fn is_fix(&self) -> bool {
        self.is_fix
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct FixAllIons {
    is_fix: bool,
}

impl FixAllIons {
    pub fn new(is_fix: bool) -> Self {
        Self { is_fix }
    }

    pub fn is_fix(&self) -> bool {
        self.is_fix
    }
}

#[derive(Default, Debug, Clone, Copy)]
/// # Note: If `FixAllIons` == `false`, default to `true`
pub struct FixCom {
    is_fix: bool,
}

impl FixCom {
    pub fn new(is_fix: bool) -> Self {
        Self { is_fix }
    }

    pub fn is_fix(&self) -> bool {
        self.is_fix
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct FixVol {
    is_fix: bool,
}

impl FixVol {
    pub fn new(is_fix: bool) -> Self {
        Self { is_fix }
    }

    pub fn is_fix(&self) -> bool {
        self.is_fix
    }
}

#[derive(Debug, Clone, Default)]
pub struct IonicConstraintsBlock {
    constraints: Vec<Constraint>,
}

impl IonicConstraintsBlock {
    pub fn new(constraints: Vec<Constraint>) -> Self {
        Self { constraints }
    }

    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Constraint {
    id: usize,
    element: ElementSymbol,
    id_in_species: usize,
    i: f64,
    j: f64,
    k: f64,
}

impl Constraint {
    pub fn new(
        id: usize,
        element: ElementSymbol,
        id_in_species: usize,
        i: f64,
        j: f64,
        k: f64,
    ) -> Self {
        Self {
            id,
            element,
            id_in_species,
            i,
            j,
            k,
        }
    }
}

impl Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:>6}{:>3} {} {:20.16}{:20.16}{:20.16}",
            self.id, self.element, self.id_in_species, self.i, self.j, self.k
        )
    }
}

impl FieldDisplay for FixAllCell {
    fn field_tag(&self) -> String {
        "FIX_ALL_CELL".to_string()
    }

    fn value(&self) -> String {
        format!("{}", self.is_fix)
    }
}

impl FieldDisplay for FixAllIons {
    fn field_tag(&self) -> String {
        "FIX_ALL_IONS".to_string()
    }

    fn value(&self) -> String {
        format!("{}", self.is_fix)
    }
}

impl FieldDisplay for FixCom {
    fn field_tag(&self) -> String {
        "FIX_COM".to_string()
    }

    fn value(&self) -> String {
        format!("{}", self.is_fix)
    }
}

impl Display for FixAllCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl Display for FixAllIons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl Display for FixCom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl BlockDisplay for IonicConstraintsBlock {
    fn block_tag(&self) -> String {
        "IONIC_CONSTRAINTS".to_string()
    }

    fn entries(&self) -> String {
        // } else {
        self.constraints
            .iter()
            .map(|c| format!("{c}"))
            .collect::<Vec<String>>()
            .join("\n")
        // }
    }
}

impl Display for IonicConstraintsBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
    }
}

#[cfg(test)]
mod test {

    use super::IonicConstraintsBlock;

    #[test]
    fn constraints() {
        let ic = IonicConstraintsBlock::default();
        println!("{}", ic)
    }
}

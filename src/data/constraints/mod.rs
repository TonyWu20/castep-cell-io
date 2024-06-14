use std::fmt::Display;

use castep_periodic_table::element::ElementSymbol;

use crate::formatting::FieldDisplay;

#[derive(Debug, Clone, Copy)]
pub struct CellConstraints {
    a: u32,
    b: u32,
    c: u32,
    alpha: u32,
    beta: u32,
    gamma: u32,
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

pub struct IonicConstraints {
    constraints: Vec<Constraint>,
}

impl IonicConstraints {
    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }
}

pub struct Constraint {
    id: usize,
    element: ElementSymbol,
    id_in_species: usize,
    i: f64,
    j: f64,
    k: f64,
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

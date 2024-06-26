use std::fmt::Display;

use nalgebra::{Matrix3, Vector3};

use crate::formatting::BlockDisplay;

#[derive(Debug, Clone)]
pub struct SymmetryOpsBlock {
    operations: Vec<SymmetryOps>,
}

impl SymmetryOpsBlock {
    pub fn new(operations: Vec<SymmetryOps>) -> Self {
        Self { operations }
    }

    pub fn operations(&self) -> &[SymmetryOps] {
        &self.operations
    }
}

#[derive(Debug, Clone)]
/// Note: Not sure if the rotation matrices used by `castep` follow column-major order or
/// row-major order. Assume column-major order first.
pub struct SymmetryOps {
    rotation: Matrix3<f64>,
    translation: Vector3<f64>,
}

impl SymmetryOps {
    pub fn new(rotation: Matrix3<f64>, translation: Vector3<f64>) -> Self {
        Self {
            rotation,
            translation,
        }
    }

    pub fn rotation(&self) -> Matrix3<f64> {
        self.rotation
    }

    pub fn translation(&self) -> Vector3<f64> {
        self.translation
    }
}

impl Display for SymmetryOps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = [
            format!("{}", self.rotation.map(|v| format!("{v:24.18}"))),
            format!("{}", self.translation.map(|v| format!("{v:24.18}"))),
        ]
        .join("\n");
        write!(f, "{}", content)
    }
}

impl BlockDisplay for SymmetryOpsBlock {
    fn block_tag(&self) -> String {
        "SYMMETRY_OPS".to_string()
    }

    fn entries(&self) -> String {
        self.operations
            .iter()
            .map(|ops| format!("{ops}"))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Display for SymmetryOpsBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content())
    }
}

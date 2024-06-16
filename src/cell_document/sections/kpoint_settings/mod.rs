use std::fmt::Display;

use crate::InvLengthUnit;

pub mod mp_settings;
pub mod nc_settings;

#[derive(Debug, Clone, Copy, Default)]
pub enum KpointTask {
    #[default]
    #[allow(clippy::upper_case_acronyms)]
    SCF,
    Spectral,
    Phonon,
}

#[derive(Debug, Clone)]
pub enum KpointSettings {
    List(KpointListBlock),
    MPGrid(KpointMPGrid),
    MPSpacing(KpointMPSpacing),
}

impl Display for KpointSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            KpointSettings::List(list_block) => format!("{}", list_block),
            KpointSettings::MPGrid(grid) => format!("{}", grid),
            KpointSettings::MPSpacing(spacing) => format!("{}", spacing),
        };
        write!(f, "{content}")
    }
}

#[derive(Debug, Clone)]
pub enum NCKpointSettings {
    List(KpointListBlock),
    Path(BSKpointPath),
    PathSpacing(BSKpointPathSpacing),
}

impl Display for NCKpointSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            NCKpointSettings::List(l) => format!("{}", l),
            NCKpointSettings::Path(pth) => format!("{}", pth),
            NCKpointSettings::PathSpacing(pth_spacing) => format!("{}", pth_spacing),
        };
        write!(f, "{content}")
    }
}

/// The first three entries on a line are the fractional positions of the k-point
/// relative to the reciprocal space lattice vectors. The final entry on a line
/// is the weight of the k-point relative to the others specified. The sum of the
/// weights must be equal to 1. The first three entries on a line are the fractional
/// positions of the k-point relative to the reciprocal space lattice vectors.
/// The final entry on a line is the weight of the k-point relative to the others
/// specified. The sum of the weights must be equal to 1.
#[derive(Debug, Clone)]
pub struct KpointListBlock {
    task: KpointTask,
    kpoint_list: Vec<[f64; 4]>,
}

impl KpointListBlock {
    pub fn new(task: KpointTask, kpoint_list: Vec<[f64; 4]>) -> Self {
        Self { task, kpoint_list }
    }
}

/// # Example
/// `KPOINT_MP_GRID I_i I_j I_k`
/// This specifies the dimensions of the Monkhorst-Pack grid requested in the
/// directions of the reciprocal space lattice vectors. The generated grid will
/// be `I_i x I_j x I_k`; any symmetries generated (or supplied) will be used to reduce this number, when computing the irreducible wedge.
#[derive(Debug, Clone, Copy)]
pub struct KpointMPGrid {
    task: KpointTask,
    grid: [u32; 3],
}

impl KpointMPGrid {
    pub fn new(task: KpointTask, grid: [u32; 3]) -> Self {
        Self { task, grid }
    }
}

/// # Example
/// `KPOINT_MP_SPACING R [units]`
/// The single entry is the maximum distance between k-points on the Monkhorst-Pack
/// grid. The dimensions of the grid will be chosen such that the maximum separation
/// of k-points is less than this.
#[derive(Debug, Clone, Copy)]
pub struct KpointMPSpacing {
    task: KpointTask,
    spacing: f64,
    unit: InvLengthUnit,
}

impl KpointMPSpacing {
    pub fn new(task: KpointTask, spacing: f64, unit: InvLengthUnit) -> Self {
        Self {
            task,
            spacing,
            unit,
        }
    }
}

/// # Example
/// `KPOINT_MP_OFFSET R_i R_j R_k`
/// This specifies the offset of the Monkhorst-Pack grid with respect to the
/// origin of the Brillouin zone. The three entries are the offset in fractional
/// coordinates relative to the reciprocal lattice vectors.
#[derive(Debug, Clone, Copy)]
pub struct KpointMPOffset {
    offset: [f64; 3],
}

impl KpointMPOffset {
    pub fn new(offset: [f64; 3]) -> Self {
        Self { offset }
    }
}

#[derive(Debug, Clone)]
pub struct BSKpointPath {
    paths: Vec<[f64; 3]>,
}

impl BSKpointPath {
    pub fn new(paths: Vec<[f64; 3]>) -> Self {
        Self { paths }
    }
}

/// # Example
/// BS_KPOINT_PATH_SPACING : 0.125
/// This keyword determines the maximum spacing between k-points used for non-self consistent band structure calculations, along the path specified by `BS_KPOINT_PATH`.
#[derive(Debug, Clone)]
pub struct BSKpointPathSpacing {
    unit: InvLengthUnit,
    spacing: f64,
}

impl BSKpointPathSpacing {
    pub fn new(unit: InvLengthUnit, spacing: f64) -> Self {
        Self { unit, spacing }
    }
}

use crate::InvLengthUnit;

pub mod mp_settings;

#[derive(Debug, Clone, Copy, Default)]
pub enum KpointTask {
    #[default]
    SCF,
    Spectral,
    Phonon,
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

/// `KPOINT_MP_GRID I_i I_j I_k`
/// This specifies the dimensions of the Monkhorst-Pack grid requested in the
/// directions of the reciprocal space lattice vectors. The generated grid will
/// be `I_i x I_j x I_k`; any symmetries generated (or supplied) will be used to reduce this number, when computing the irreducible wedge.
#[derive(Debug, Clone, Copy)]
pub struct KpointMPGrid {
    task: KpointTask,
    grid: [u32; 3],
}

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

/// `KPOINT_MP_OFFSET R_i R_j R_k`
/// This specifies the offset of the Monkhorst-Pack grid with respect to the
/// origin of the Brillouin zone. The three entries are the offset in fractional
/// coordinates relative to the reciprocal lattice vectors.
pub struct KpointMPOffset([f64; 3]);

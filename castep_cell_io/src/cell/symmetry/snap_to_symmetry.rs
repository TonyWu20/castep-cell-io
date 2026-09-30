use castep_cell_fmt::{Cell, ToCell};

/// Forces the supplied lattice parameters and ionic positions to obey the
/// symmetries supplied or generated in the cell (to machine precision).
///
/// Keyword type: Flag
///
/// 6.11: `SNAP_TO_SYMMETRY` in the `.cell` file turns snapping on. When the
/// keyword is not present, CASTEP does not snap the cell. The keyword carries
/// no value; CASTEP reads it with `io_freeform_defined`.
///
/// Example:
/// SNAP_TO_SYMMETRY
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SnapToSymmetry;

impl ToCell for SnapToSymmetry {
    fn to_cell(&self) -> Cell<'_> {
        Cell::Flag("SNAP_TO_SYMMETRY")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snap_to_symmetry_to_cell() {
        let s = SnapToSymmetry;
        let cell = s.to_cell();
        match cell {
            Cell::Flag(name) => assert_eq!(name, "SNAP_TO_SYMMETRY"),
            _ => panic!("Expected Cell::Flag, got {:?}", cell),
        }
    }
}

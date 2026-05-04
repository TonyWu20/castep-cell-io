use castep_cell_fmt::{Cell, ToCell};

/// If present, indicates that symmetry should be generated rather than read from the input.
///
/// Keyword type: Flag
///
/// When SYMMETRY_GENERATE is present in the .cell file, CASTEP will generate symmetry
/// operations automatically from the unit cell geometry instead of reading them from
/// a SYMMETRY_OPS block.
///
/// Example:
/// SYMMETRY_GENERATE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SymmetryGenerate;

impl ToCell for SymmetryGenerate {
    fn to_cell(&self) -> Cell<'_> {
        Cell::Flag("SYMMETRY_GENERATE")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetry_generate_to_cell() {
        let sg = SymmetryGenerate;
        let cell = sg.to_cell();
        match cell {
            Cell::Flag(name) => assert_eq!(name, "SYMMETRY_GENERATE"),
            _ => panic!("Expected Cell::Flag, got {:?}", cell),
        }
    }
}

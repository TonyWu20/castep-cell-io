use castep_cell_fmt::{Cell, ToCell};

/// Marker keyword with no effect.
///
/// 6.11 registers `ATOMIC_INIT` as a no-op ("Dummy! No action"): it is
/// accepted on input but does nothing. Modelled as a presence marker so
/// round-tripping a document that contains the keyword does not drop it.
///
/// Keyword type: Flag (presence only, no value)
///
/// Example:
/// ATOMIC_INIT
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtomicInit;

impl ToCell for AtomicInit {
    fn to_cell(&self) -> Cell<'_> {
        Cell::Flag("ATOMIC_INIT")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_init_to_cell() {
        let a = AtomicInit;
        let cell = a.to_cell();
        match cell {
            Cell::Flag(name) => assert_eq!(name, "ATOMIC_INIT"),
            _ => panic!("Expected Cell::Flag, got {:?}", cell),
        }
    }
}

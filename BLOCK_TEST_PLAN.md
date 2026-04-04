# Block Type Test Plan (Units 5-8)

## Status
- Units 1-4: Complete (296 tests passing)
- Units 5-8: Pending (block types with FromBlock trait)

## Overview

27 block type files in `castep_cell_data/src/cell/` need unit tests. Breaking into 4 parallel work units based on complexity and pattern similarity.

---

## Work Unit 5A: Lattice Block Types

**Pattern:** E (blocks with optional unit specification)
**Files:** 1
**Estimated tests:** 12

### Files to Test

1. `castep_cell_data/src/cell/lattice_param.rs`
   - `LatticeCart` (3 data rows)
   - `LatticeABC` (2 data rows)

### Test Requirements

For each type:
- ✓ Parse with unit specification (first row = unit)
- ✓ Parse without unit (all rows = data)
- ✓ Verify correct row count
- ✓ Error on insufficient rows
- ✓ Error on empty block
- ✓ Verify BLOCK_NAME constant

### Example Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_io::{CellValue, parse::FromBlock};

    #[test]
    fn test_lattice_cart_with_unit() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Str("ang")]),
            CellValue::Array(vec![CellValue::Float(1.0), CellValue::Float(0.0), CellValue::Float(0.0)]),
            CellValue::Array(vec![CellValue::Float(0.0), CellValue::Float(1.0), CellValue::Float(0.0)]),
            CellValue::Array(vec![CellValue::Float(0.0), CellValue::Float(0.0), CellValue::Float(1.0)]),
        ];
        let result = LatticeCart::from_block_rows(&rows).unwrap();
        assert!(result.unit.is_some());
        assert_eq!(result.a, [1.0, 0.0, 0.0]);
    }

    #[test]
    fn test_lattice_cart_without_unit() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Float(1.0), CellValue::Float(0.0), CellValue::Float(0.0)]),
            CellValue::Array(vec![CellValue::Float(0.0), CellValue::Float(1.0), CellValue::Float(0.0)]),
            CellValue::Array(vec![CellValue::Float(0.0), CellValue::Float(0.0), CellValue::Float(1.0)]),
        ];
        let result = LatticeCart::from_block_rows(&rows).unwrap();
        assert!(result.unit.is_none());
    }

    #[test]
    fn test_lattice_cart_insufficient_rows() {
        let rows = vec![
            CellValue::Array(vec![CellValue::Float(1.0), CellValue::Float(0.0), CellValue::Float(0.0)]),
        ];
        assert!(LatticeCart::from_block_rows(&rows).is_err());
    }

    #[test]
    fn test_lattice_cart_empty() {
        assert!(LatticeCart::from_block_rows(&[]).is_err());
    }

    #[test]
    fn test_block_name() {
        assert_eq!(LatticeCart::BLOCK_NAME, "LATTICE_CART");
    }
}
```

---

## Work Unit 5B: Position/Kpoint Block Types (Part 1)

**Pattern:** F (entry list blocks with optional fields)
**Files:** 3
**Estimated tests:** 18

### Files to Test

1. `castep_cell_data/src/cell/positions/positions_frac.rs`
   - `PositionFracEntry` (FromCellValue)
   - `PositionsFrac` (FromBlock)

2. `castep_cell_data/src/cell/bz_sampling_kpoints/kpoints_list.rs`
   - `KpointEntry` (FromCellValue)
   - `KpointsList` (FromBlock)

3. `castep_cell_data/src/cell/bz_sampling_kpoints/bs_kpoints_list.rs`
   - `BsKpointEntry` (FromCellValue)
   - `BsKpointsList` (FromBlock)

### Test Requirements

For each entry type:
- ✓ Parse entry without optional fields
- ✓ Parse entry with optional fields (SPIN, weight, label)
- ✓ Error on insufficient array elements

For each block type:
- ✓ Parse block with multiple entries
- ✓ Parse empty block
- ✓ Verify BLOCK_NAME constant

### Example Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_io::{CellValue, FromCellValue, parse::FromBlock};

    #[test]
    fn test_entry_without_optional() {
        let val = CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
        ]);
        let entry = PositionFracEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.species.to_string(), "Fe");
        assert!(entry.spin.is_none());
    }

    #[test]
    fn test_entry_with_spin() {
        let val = CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
            CellValue::Str("SPIN"),
            CellValue::Float(2.0),
        ]);
        let entry = PositionFracEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.spin, Some(2.0));
    }

    #[test]
    fn test_block_multiple_entries() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
            ]),
            CellValue::Array(vec![
                CellValue::Str("O"),
                CellValue::Float(0.5),
                CellValue::Float(0.5),
                CellValue::Float(0.5),
            ]),
        ];
        let result = PositionsFrac::from_block_rows(&rows).unwrap();
        assert_eq!(result.positions.len(), 2);
    }

    #[test]
    fn test_block_empty() {
        let result = PositionsFrac::from_block_rows(&[]).unwrap();
        assert_eq!(result.positions.len(), 0);
    }
}
```

---

## Work Unit 5C: Constraint Block Types

**Pattern:** G (constraint blocks with species/indices)
**Files:** 3
**Estimated tests:** 15

### Files to Test

1. `castep_cell_data/src/cell/constraints/ionic_constraints.rs`
   - `IonicConstraintEntry` (FromCellValue)
   - `IonicConstraints` (FromBlock)

2. `castep_cell_data/src/cell/constraints/cell_constraints.rs`
   - `CellConstraintEntry` (FromCellValue)
   - `CellConstraints` (FromBlock)

3. `castep_cell_data/src/cell/constraints/nonlinear_constraints.rs`
   - `NonlinearConstraintEntry` (FromCellValue)
   - `NonlinearConstraints` (FromBlock)

### Test Requirements

For each entry type:
- ✓ Parse entry with all required fields
- ✓ Verify species/index parsing
- ✓ Verify coefficient parsing
- ✓ Error on insufficient elements

For each block type:
- ✓ Parse block with multiple constraints
- ✓ Parse empty block
- ✓ Verify BLOCK_NAME constant

### Example Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_io::{CellValue, FromCellValue, parse::FromBlock};

    #[test]
    fn test_constraint_entry() {
        let val = CellValue::Array(vec![
            CellValue::UInt(1),
            CellValue::Str("Fe"),
            CellValue::UInt(1),
            CellValue::Float(1.0),
            CellValue::Float(0.0),
            CellValue::Float(0.0),
        ]);
        let entry = IonicConstraintEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.constraint_number, 1);
        assert_eq!(entry.ion_number, 1);
        assert_eq!(entry.coefficients, [1.0, 0.0, 0.0]);
    }

    #[test]
    fn test_block_multiple_constraints() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::UInt(1),
                CellValue::Str("Fe"),
                CellValue::UInt(1),
                CellValue::Float(1.0),
                CellValue::Float(0.0),
                CellValue::Float(0.0),
            ]),
        ];
        let result = IonicConstraints::from_block_rows(&rows).unwrap();
        assert_eq!(result.constraints.len(), 1);
    }
}
```

---

## Work Unit 5D: Complex Block Types

**Pattern:** H (nested structures, multiple sub-fields)
**Files:** 8
**Estimated tests:** 30

### Files to Test

1. `castep_cell_data/src/cell/species/hubbard_u/hubbard_u_aux.rs`
   - `HubbardUEntry` (FromCellValue)
   - `HubbardU` (FromBlock)

2. `castep_cell_data/src/cell/species/hubbard_u/atom_hubbard_u_aux.rs`
   - `AtomHubbardUEntry` (FromCellValue)
   - `AtomHubbardU` (FromBlock)

3. `castep_cell_data/src/cell/species/quantization_axis.rs`
   - `QuantizationAxisEntry` (FromCellValue)
   - `QuantizationAxis` (FromBlock)

4. `castep_cell_data/src/cell/species/species_mass.rs`
   - `SpeciesMassEntry` (FromCellValue)
   - `SpeciesMass` (FromBlock)

5. `castep_cell_data/src/cell/species/species_pot.rs`
   - `SpeciesPotEntry` (FromCellValue)
   - `SpeciesPot` (FromBlock)

6. `castep_cell_data/src/cell/species/species_lcao_states.rs`
   - `SpeciesLcaoStatesEntry` (FromCellValue)
   - `SpeciesLcaoStates` (FromBlock)

7. `castep_cell_data/src/cell/symmetry/symmetry_ops.rs`
   - `SymmetryOpEntry` (FromCellValue)
   - `SymmetryOps` (FromBlock)

8. `castep_cell_data/src/cell/external_fields/external_efield.rs`
   - `ExternalEfield` (FromBlock)

### Test Requirements

For each entry type:
- ✓ Parse entry with required fields
- ✓ Parse entry with optional fields (if applicable)
- ✓ Verify nested field parsing
- ✓ Error on malformed input

For each block type:
- ✓ Parse block with entries
- ✓ Handle empty blocks
- ✓ Verify BLOCK_NAME constant

### Example Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_io::{CellValue, FromCellValue, parse::FromBlock};

    #[test]
    fn test_species_mass_entry() {
        let val = CellValue::Array(vec![
            CellValue::Str("Fe"),
            CellValue::Float(55.845),
        ]);
        let entry = SpeciesMassEntry::from_cell_value(&val).unwrap();
        assert_eq!(entry.species.to_string(), "Fe");
        assert_eq!(entry.mass, 55.845);
    }

    #[test]
    fn test_block() {
        let rows = vec![
            CellValue::Array(vec![
                CellValue::Str("Fe"),
                CellValue::Float(55.845),
            ]),
        ];
        let result = SpeciesMass::from_block_rows(&rows).unwrap();
        assert_eq!(result.entries.len(), 1);
    }

    #[test]
    fn test_block_name() {
        assert_eq!(SpeciesMass::BLOCK_NAME, "SPECIES_MASS");
    }
}
```

---

## Remaining Files (Not Block Types)

These files contain non-block types or already have tests:

- `castep_cell_data/src/cell/bz_sampling_kpoints/kpoint.rs` (unit enum, covered in Unit 4)
- `castep_cell_data/src/cell/constraints/fix_*.rs` (boolean flags, covered in Unit 3)
- `castep_cell_data/src/cell/symmetry/symmetry_tol.rs` (float KeyValue, covered in Unit 2)
- `castep_cell_data/src/cell/external_fields/external_pressure.rs` (float KeyValue, covered in Unit 2)
- All `mod.rs` files (module declarations only)

---

## Execution Strategy

1. **Parallel execution** of Units 5A-5D (independent work units)
2. Each unit delegated to one codebase-rewriter agent
3. After all units complete:
   - Run `cargo test -p castep_cell_data`
   - Run `cargo clippy -p castep_cell_data`
   - Verify total test count increases by ~75 tests
4. If any unit fails quality gates, fix before final verification

## Total Estimated New Tests: ~75

- Unit 5A: 12 tests
- Unit 5B: 18 tests
- Unit 5C: 15 tests
- Unit 5D: 30 tests

## Common Test Imports

All block type tests use:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_io::{CellValue, FromCellValue, parse::FromBlock};
    
    // tests here
}
```

---

## Next Steps

Ready to delegate Units 5A-5D to codebase-rewriter agents in parallel.

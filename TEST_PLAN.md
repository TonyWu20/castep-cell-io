# Unit Test Plan for castep_cell_data Migration

## Overview

157 files migrated from serde to custom traits (FromCellValue/FromKeyValue/FromBlock). This plan breaks test additions into 8 work units grouped by pattern and module.

**Current status:** 1 test passing in castep_cell_data
**Target:** Comprehensive unit test coverage for all migrated types

---

## Test Patterns Identified

### Pattern A: Simple Integer KeyValue Types
- Wraps `u32` or `i32`
- Implements `FromCellValue` via `value_as_u32`/`value_as_i32`
- Implements `FromKeyValue` with `KEY_NAME`
- Example: `BsMaxIter`, `BsNbands`

**Test structure:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_io::CellValue;

    #[test]
    fn test_from_cell_value() {
        let val = CellValue::UInt(50);
        let result = BsMaxIter::from_cell_value(&val).unwrap();
        assert_eq!(result.0, 50);
    }

    #[test]
    fn test_case_insensitive_key() {
        // Verify KEY_NAME is uppercase
        assert_eq!(BsMaxIter::KEY_NAME, "BS_MAX_ITER");
    }
}
```

### Pattern B: Simple Float KeyValue Types
- Wraps `f64`
- Implements `FromCellValue` via `value_as_f64`
- Example: `Spin`, `ElecEnergyTol`

**Test structure:**
```rust
#[test]
fn test_from_cell_value() {
    let val = CellValue::Float(3.0);
    let result = Spin::from_cell_value(&val).unwrap();
    assert_eq!(result.0, 3.0);
}
```

### Pattern C: Enum KeyValue Types (String-based)
- Enum with multiple variants
- Case-insensitive string matching in `from_cell_value`
- Example: `Task`, `XcFunctional`

**Test structure:**
```rust
#[test]
fn test_case_insensitive_parsing() {
    let val = CellValue::Str("singlepoint");
    assert_eq!(Task::from_cell_value(&val).unwrap(), Task::SinglePoint);
    
    let val = CellValue::Str("SINGLEPOINT");
    assert_eq!(Task::from_cell_value(&val).unwrap(), Task::SinglePoint);
    
    let val = CellValue::Str("SinglePoint");
    assert_eq!(Task::from_cell_value(&val).unwrap(), Task::SinglePoint);
}

#[test]
fn test_invalid_variant() {
    let val = CellValue::Str("invalid");
    assert!(Task::from_cell_value(&val).is_err());
}
```

### Pattern D: Boolean KeyValue Types
- Wraps `bool` or is a unit struct (flag)
- Implements `FromCellValue` via `value_as_bool`
- Example: `SpinPolarized`, `FixAllCell`

**Test structure:**
```rust
#[test]
fn test_from_cell_value() {
    let val = CellValue::Bool(true);
    assert!(SpinPolarized::from_cell_value(&val).unwrap().0);
    
    let val = CellValue::Bool(false);
    assert!(!SpinPolarized::from_cell_value(&val).unwrap().0);
}
```

### Pattern E: Block Types with Optional Units
- First row may contain unit specification
- Subsequent rows contain numeric data
- Example: `LatticeCart`, `LatticeABC`

**Test structure:**
```rust
#[test]
fn test_with_unit() {
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
fn test_without_unit() {
    let rows = vec![
        CellValue::Array(vec![CellValue::Float(1.0), CellValue::Float(0.0), CellValue::Float(0.0)]),
        CellValue::Array(vec![CellValue::Float(0.0), CellValue::Float(1.0), CellValue::Float(0.0)]),
        CellValue::Array(vec![CellValue::Float(0.0), CellValue::Float(0.0), CellValue::Float(1.0)]),
    ];
    let result = LatticeCart::from_block_rows(&rows).unwrap();
    assert!(result.unit.is_none());
}

#[test]
fn test_insufficient_rows() {
    let rows = vec![
        CellValue::Array(vec![CellValue::Float(1.0), CellValue::Float(0.0), CellValue::Float(0.0)]),
    ];
    assert!(LatticeCart::from_block_rows(&rows).is_err());
}
```

### Pattern F: Block Types with Entry Lists
- Each row is a structured entry
- Entries may have optional fields
- Example: `PositionsFrac`, `KpointsList`

**Test structure:**
```rust
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
fn test_entry_with_optional() {
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
```

### Pattern G: Constraint Block Types
- Single-row or multi-row constraints
- May contain species/atom indices
- Example: `IonicConstraints`, `CellConstraints`

### Pattern H: Complex Nested Types
- Types with multiple sub-fields
- May combine multiple patterns
- Example: `HubbardU`, `SymmetryOps`

---

## Work Units

### Unit 1: Simple Integer Types (param/*)
**Files:** ~40 files across band_structure, basis_set, electronic_minimisation, geometry_optimization, phonon
**Pattern:** A
**Estimated tests per file:** 2-3
**Total tests:** ~100

**Scope:**
- All `*_max_iter`, `*_nbands`, `*_max_cg_steps` types
- All `*_convergence_win` types
- Test: valid integer parsing, KEY_NAME verification

### Unit 2: Simple Float Types (param/*)
**Files:** ~35 files across electronic, efield, geometry_optimization, molecular_dynamics
**Pattern:** B
**Estimated tests per file:** 2-3
**Total tests:** ~85

**Scope:**
- All `*_energy_tol`, `*_force_tol` types
- All `*_damping`, `*_scale` types
- Test: valid float parsing, KEY_NAME verification

### Unit 3: Boolean/Flag Types (param/* and cell/constraints/*)
**Files:** ~25 files
**Pattern:** D
**Estimated tests per file:** 2-3
**Total tests:** ~60

**Scope:**
- `SpinPolarized`, `CalculateStress`, `WriteFormatted*` types
- `FixAllCell`, `FixAllIons`, `FixCom`, `FixVol` types
- Test: true/false parsing, KEY_NAME verification

### Unit 4: Enum Types (param/general/*, param/electronic/*)
**Files:** ~15 files (Task, XcFunctional, ElecMethod, etc.)
**Pattern:** C
**Estimated tests per file:** 4-6
**Total tests:** ~75

**Scope:**
- All enum types with string-based variants
- Test: case-insensitive parsing for all variants, invalid input error handling

### Unit 5: Lattice Block Types (cell/lattice_param.rs)
**Files:** 1 file, 2 types (LatticeCart, LatticeABC)
**Pattern:** E
**Estimated tests per type:** 5-6
**Total tests:** ~12

**Scope:**
- `LatticeCart`, `LatticeABC`
- Test: with/without units, correct row count, insufficient rows error

### Unit 6: Position/Kpoint Block Types (cell/positions/*, cell/bz_sampling_kpoints/*)
**Files:** ~6 files
**Pattern:** F
**Estimated tests per file:** 4-6
**Total tests:** ~30

**Scope:**
- `PositionsFrac`, `PositionsAbs`
- `KpointsList`, `BsKpointsList`
- Test: entries with/without optional fields, empty blocks, malformed entries

### Unit 7: Constraint Block Types (cell/constraints/*)
**Files:** ~5 files
**Pattern:** G
**Estimated tests per file:** 3-5
**Total tests:** ~20

**Scope:**
- `IonicConstraints`, `CellConstraints`, `NonlinearConstraints`
- Test: single/multi-row parsing, species/index handling

### Unit 8: Complex Types (cell/species/*, cell/symmetry/*, cell/external_fields/*)
**Files:** ~15 files
**Pattern:** H
**Estimated tests per file:** 3-5
**Total tests:** ~55

**Scope:**
- `HubbardU`, `QuantizationAxis`, `SpeciesMass`, `SpeciesPot`
- `SymmetryOps`, `SymmetryTol`
- `ExternalEfield`, `ExternalPressure`
- Test: nested field parsing, optional sub-fields, complex validation

---

## Total Estimated Tests: ~437

## Execution Strategy

1. **Sequential execution** of units 1-8
2. Each unit delegated to one codebase-rewriter agent
3. After each unit completes:
   - Run `cargo test -p castep_cell_data`
   - Run `cargo clippy -p castep_cell_data`
   - Verify no regressions
4. If unit fails quality gates, fix before proceeding

## Test File Organization

Tests should be added as inline `#[cfg(test)]` modules at the end of each file:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_io::CellValue;

    // tests here
}
```

## Common Test Utilities

All tests use these imports:
```rust
use super::*;
use castep_cell_io::CellValue;
```

For block types, also import:
```rust
use castep_cell_io::parse::FromBlock;
```

---

## Next Steps

Ready to begin Unit 1 delegation to codebase-rewriter agent.

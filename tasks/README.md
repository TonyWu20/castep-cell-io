# Task Breakdown Summary

## Overview
Phase 2A and 2B broken into 11 parallel tasks + 1 sequential Phase 3 task.

## Phase 2A: Param Types (Tasks 01-06)
**136 files total** - all can run in parallel

| Task | Module(s) | Files | Pattern |
|------|-----------|-------|---------|
| 01 | general | 24 | B/C |
| 02 | electronic_minimisation | 16 | B/C |
| 03 | molecular_dynamics | 21 | B/C |
| 04 | geometry_optimization, phonon | 23 | B/C |
| 05 | basis_set, density_mixing, band_structure | 26 | B/C |
| 06 | electronic, efield, nmr, population_analysis, pseudopotential, exchange_correlation | 20 | B/C |

## Phase 2B: Cell Block Types (Tasks 07-11)
**~30 files total** - all can run in parallel

| Task | Module(s) | Files | Pattern |
|------|-----------|-------|---------|
| 07 | lattice_param, external_fields | 4 | D (optional unit) |
| 08 | species/hubbard_u, species/species_mass | 4 | D (optional unit) |
| 09 | positions, bz_sampling_kpoints | 5 | E (Vec rows) |
| 10 | constraints | 7 | E (Vec rows) |
| 11 | species (remaining), symmetry | 7 | E (Vec rows) |

## Phase 3: Cleanup (Task 12)
**Sequential - runs after all Phase 2 tasks complete**
- Remove serde infrastructure
- Rename crate castep_cell_serde → castep_cell_io
- Update all dependencies and imports

## Parallelization Strategy
- **Wave 1 (parallel)**: Tasks 01-11 can all run simultaneously
- **Wave 2 (sequential)**: Task 12 runs after Wave 1 completes

## Patterns Reference

**Pattern B (newtype):**
```rust
impl FromKeyValue for T {
    const KEY_NAME: &'static str = "KEY";
    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(Inner::from_cell_value(value)?))
    }
}
```

**Pattern C (keyword enum):**
```rust
impl FromCellValue for T {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "variant" => Ok(Self::Variant),
            other => Err(Error::Message(format!("unknown T: {other}"))),
        }
    }
}
impl FromKeyValue for T {
    const KEY_NAME: &'static str = "KEY";
    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}
```

**Pattern D (optional unit prefix):**
```rust
impl FromBlock for T {
    const BLOCK_NAME: &'static str = "BLOCK";
    fn from_block(rows: &[CellValue<'_>]) -> CResult<Self> {
        // Check first row for unit, parse remaining rows
    }
}
```

**Pattern E (Vec rows):**
```rust
impl FromCellValue for Row { /* parse array */ }
impl FromBlock for T {
    const BLOCK_NAME: &'static str = "BLOCK";
    fn from_block(rows: &[CellValue<'_>]) -> CResult<Self> {
        let data = rows.iter().map(Row::from_cell_value).collect()?;
        Ok(Self(data))
    }
}
```

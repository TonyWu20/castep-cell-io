# Task 07: Migrate cell/lattice_param and cell/external_fields (Pattern D)

## Objective
Migrate 4 block types with optional unit prefix (Pattern D) from serde to `FromBlock` trait.

## Files (4)
- castep_cell_data/src/cell/lattice_param.rs (LatticeCart, LatticeABC)
- castep_cell_data/src/cell/external_fields/external_efield.rs
- castep_cell_data/src/cell/external_fields/external_pressure.rs

## Pattern D (optional unit prefix)
Block types where first row may be a unit keyword, followed by data rows.

**Steps per file:**
1. Delete `*Repr` enum and `From<*Repr>` impl
2. Remove `#[derive(Serialize, Deserialize)]` and `#[serde(...)]`
3. Add `FromBlock` impl that:
   - Checks first row for unit keyword (case-insensitive)
   - Parses remaining rows as data
4. Update tests to use `parse()` instead of serde

**Example structure:**
```rust
impl FromBlock for TypeName {
    const BLOCK_NAME: &'static str = "BLOCK_NAME";
    
    fn from_block(rows: &[CellValue<'_>]) -> CResult<Self> {
        // Check first row for unit
        let (unit, data_start) = if let Some(CellValue::Str(s)) = rows.first() {
            match s.to_ascii_lowercase().as_str() {
                "unit1" => (Some(Unit::Unit1), 1),
                "unit2" => (Some(Unit::Unit2), 1),
                _ => (None, 0),
            }
        } else {
            (None, 0)
        };
        
        // Parse data rows
        let data = rows[data_start..].iter()
            .map(|row| /* parse row */)
            .collect::<CResult<Vec<_>>>()?;
            
        Ok(Self { unit, data })
    }
}
```

## Acceptance Criteria
- [ ] All 4 files migrated (lattice_param.rs has 2 types)
- [ ] No `*Repr` enums remain
- [ ] `cargo test -p castep_cell_data` passes
- [ ] `cargo clippy` passes

## Dependencies
None (can run in parallel with other cell tasks)

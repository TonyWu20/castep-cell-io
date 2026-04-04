# Task 09: Migrate cell/positions and cell/bz_sampling_kpoints (Pattern E)

## Objective
Migrate 5 block types with Vec rows (Pattern E) from serde to `FromBlock` trait.

## Files (5)
- castep_cell_data/src/cell/positions/positions_frac.rs
- castep_cell_data/src/cell/bz_sampling_kpoints/kpoint.rs
- castep_cell_data/src/cell/bz_sampling_kpoints/kpoints_list.rs
- castep_cell_data/src/cell/bz_sampling_kpoints/bs_kpoints_list.rs

## Pattern E (Vec of rows)
Block types where each row is a data record, often with a row type implementing `FromCellValue`.

**Steps per file:**
1. Delete `*Repr` enum and `From<*Repr>` impl
2. Remove `#[derive(Serialize, Deserialize)]` and `#[serde(...)]`
3. If row type exists, add `FromCellValue` impl for it
4. Add `FromBlock` impl that maps rows to Vec
5. Update tests

**Example structure:**
```rust
// Row type
impl FromCellValue for RowType {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        let arr = value_as_array(value)?;
        // Parse array elements
        Ok(Self { /* fields */ })
    }
}

// Block type
impl FromBlock for BlockType {
    const BLOCK_NAME: &'static str = "BLOCK_NAME";
    
    fn from_block(rows: &[CellValue<'_>]) -> CResult<Self> {
        let data = rows.iter()
            .map(RowType::from_cell_value)
            .collect::<CResult<Vec<_>>>()?;
        Ok(Self(data))
    }
}
```

## Acceptance Criteria
- [ ] All 5 files migrated
- [ ] Row types implement `FromCellValue`
- [ ] Block types implement `FromBlock`
- [ ] No `*Repr` enums remain
- [ ] `cargo test -p castep_cell_data` passes
- [ ] `cargo clippy` passes

## Dependencies
None (can run in parallel)

# Task 01: Migrate param/general types

## Objective
Migrate 24 param types in `general/` module from serde to `FromKeyValue` trait.

## Files (24)
- castep_cell_data/src/param/general/backup_interval.rs
- castep_cell_data/src/param/general/calculate_densdiff.rs
- castep_cell_data/src/param/general/calculate_elf.rs
- castep_cell_data/src/param/general/calculate_hirshfeld.rs
- castep_cell_data/src/param/general/calculate_stress.rs
- castep_cell_data/src/param/general/checkpoint.rs
- castep_cell_data/src/param/general/comment.rs
- castep_cell_data/src/param/general/continuation.rs
- castep_cell_data/src/param/general/data_distribution.rs
- castep_cell_data/src/param/general/iprint.rs
- castep_cell_data/src/param/general/num_backup_iter.rs
- castep_cell_data/src/param/general/opt_strategy.rs
- castep_cell_data/src/param/general/page_wvfns.rs
- castep_cell_data/src/param/general/print_clock.rs
- castep_cell_data/src/param/general/print_memory_usage.rs
- castep_cell_data/src/param/general/rand_seed.rs
- castep_cell_data/src/param/general/reuse.rs
- castep_cell_data/src/param/general/run_time.rs
- castep_cell_data/src/param/general/stop.rs
- castep_cell_data/src/param/general/write_checkpoint.rs
- castep_cell_data/src/param/general/write_formatted_density.rs
- castep_cell_data/src/param/general/write_formatted_elf.rs
- castep_cell_data/src/param/general/write_formatted_potential.rs
- castep_cell_data/src/param/general/write_orbitals.rs

## Pattern
Follow `task.rs` as reference. For each file:

1. Remove `#[derive(Serialize, Deserialize)]`
2. Remove all `#[serde(...)]` attributes
3. Add `FromKeyValue` impl (and `FromCellValue` if keyword enum)
4. Update tests to use `parse()` instead of serde

**Pattern B (newtype):**
```rust
impl FromKeyValue for TypeName {
    const KEY_NAME: &'static str = "KEYWORD";
    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Ok(Self(InnerType::from_cell_value(value)?))
    }
}
```

**Pattern C (keyword enum):**
```rust
impl FromCellValue for TypeName {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value_as_str(value)?.to_ascii_lowercase().as_str() {
            "variant1" => Ok(Self::Variant1),
            other => Err(Error::Message(format!("unknown TypeName: {other}"))),
        }
    }
}

impl FromKeyValue for TypeName {
    const KEY_NAME: &'static str = "KEYWORD";
    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}
```

## Acceptance Criteria
- [ ] All 24 files have no serde derives or attributes
- [ ] All types implement `FromKeyValue`
- [ ] Keyword enums also implement `FromCellValue`
- [ ] `cargo test -p castep_cell_data` passes
- [ ] `cargo clippy` passes

## Dependencies
None (can run in parallel with other param tasks)

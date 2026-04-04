# Task 12: Phase 3 - Remove serde and rename crate

## Objective
Remove all serde infrastructure and rename `castep_cell_serde` to `castep_cell_io`.

## Files to modify/delete (13)
### Delete (4)
- castep_cell_serde/src/de/mod.rs
- castep_cell_serde/src/de/deserializer.rs
- castep_cell_serde/src/de/value_deserializer.rs
- castep_cell_serde/src/ser.rs

### Modify (9)
- castep_cell_serde/src/error.rs (remove serde trait impls)
- castep_cell_serde/src/lib.rs (remove `mod de; mod ser;`)
- castep_cell_serde/src/format.rs (rename `to_string_direct` → `to_string`)
- castep_cell_serde/Cargo.toml (drop serde/anyhow/derive_builder deps, rename to castep_cell_io)
- Cargo.toml (workspace: update member name)
- castep_cell_data/Cargo.toml (update dep name/path, drop serde)
- castep-param-io/Cargo.toml (update dep name/path only)

### Rename (1)
- castep_cell_serde/ → castep_cell_io/

### Global find-replace
- `use castep_cell_serde::` → `use castep_cell_io::`

## Steps (sequential)
1. Delete `castep_cell_serde/src/de/` directory
2. Delete `castep_cell_serde/src/ser.rs`
3. Update `castep_cell_serde/src/error.rs`: remove `impl serde::de::Error` and `impl serde::ser::Error`
4. Update `castep_cell_serde/src/lib.rs`: remove `mod de;` and `mod ser;`
5. Update `castep_cell_serde/src/format.rs`: rename `to_string_direct` → `to_string`
6. Update `castep_cell_serde/Cargo.toml`:
   - Remove `serde`, `anyhow`, `derive_builder` from dependencies
   - Change `name = "castep_cell_serde"` → `name = "castep_cell_io"`
7. Rename directory: `mv castep_cell_serde castep_cell_io`
8. Update workspace `Cargo.toml`: change member `"castep_cell_serde"` → `"castep_cell_io"`
9. Update `castep_cell_data/Cargo.toml`:
   - Change dep name: `castep_cell_serde` → `castep_cell_io`
   - Update path: `path = "../castep_cell_io"`
   - Remove `serde` dependency
10. Update `castep-param-io/Cargo.toml`:
    - Change dep name: `castep_cell_serde` → `castep_cell_io`
    - Update path: `path = "../castep_cell_io"`
11. Global find-replace in all `.rs` files: `use castep_cell_serde::` → `use castep_cell_io::`
12. Run `cargo test --workspace` to verify

## Acceptance Criteria
- [ ] No `de/` directory exists
- [ ] No `ser.rs` exists
- [ ] Crate renamed to `castep_cell_io`
- [ ] No serde dependencies in castep_cell_io or castep_cell_data
- [ ] All imports updated
- [ ] `cargo build --workspace` succeeds
- [ ] `cargo test --workspace` passes
- [ ] `cargo clippy --workspace` passes

## Dependencies
**BLOCKS ALL OTHER TASKS** - Must run after Tasks 01-11 complete.
This is a single sequential task that cannot be parallelized due to file renames and global changes.

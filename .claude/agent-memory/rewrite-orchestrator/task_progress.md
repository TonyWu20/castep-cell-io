---
name: Task progress
description: Per-task status checklist for PLAN.md rewrite execution
type: project
---

# Task Checklist

## Phase 1 — Add new API to castep_cell_serde (additive, old API untouched)

| ID    | Subject                                               | Status  |
|-------|-------------------------------------------------------|---------|
| T-001 | Create src/query.rs with query + scalar helpers       | pending |
| T-002 | Create src/parse.rs with trait defs + primitive impls | pending |
| T-003 | Create src/format.rs with direct formatter            | pending |
| T-004 | Write Phase 1 integration tests                       | pending |

## Phase 2 — Migrate castep_cell_data bottom-up

| ID    | Subject                                                         | Status  |
|-------|-----------------------------------------------------------------|---------|
| T-005 | 14 unit enums (units/): remove serde, add FromCellValue         | pending |
| T-006 | Species: remove serde, add FromCellValue                        | pending |
| T-007 | ~120 param keyword types: remove serde, add FromKeyValue        | pending |
| T-008 | Pattern D block types: delete Repr enums, add FromBlock         | pending |
| T-009 | Pattern E block types: delete Repr + transparent, add impls     | pending |
| T-010 | Update all tests in castep_cell_data                            | pending |

## Phase 3 — Remove old serde layer and rename crate

| ID    | Subject                                                         | Status  |
|-------|-----------------------------------------------------------------|---------|
| T-011 | Rename to_string_direct → to_string, delete src/ser.rs          | pending |
| T-012 | Delete src/de/ (3 files)                                        | pending |
| T-013 | Remove serde trait impls from src/error.rs                      | pending |
| T-014 | Update src/lib.rs: remove mod de/ser, serde-based tests         | pending |
| T-015 | Cargo.toml (castep_cell_serde): drop serde/anyhow/derive_builder, rename pkg | pending |
| T-016 | Rename directory castep_cell_serde/ → castep_cell_io/           | pending |
| T-017 | Root Cargo.toml: update workspace member name                   | pending |
| T-018 | castep_cell_data/Cargo.toml: update dep name/path               | pending |
| T-019 | castep-param-io/Cargo.toml: update dep name/path                | pending |
| T-020 | All use castep_cell_serde:: in castep_cell_data → castep_cell_io | pending |
| T-021 | castep_cell_data/Cargo.toml: drop serde dep                     | pending |
| T-022 | Final: cargo test --workspace — all green                       | pending |

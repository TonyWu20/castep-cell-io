---
name: Project overview
description: castep-cell-io rewrite plan summary — drop serde, add plain traits, rename crate
type: project
---

The rewrite drops all serde dependency from `castep_cell_serde` and `castep_cell_data`, replaces serde-based deserialization with explicit plain-Rust parsing traits, and renames `castep_cell_serde` → `castep_cell_io`.

**Why:** Serde workarounds (Repr enums, untagged, multi-level From chains) fight serde's data model; case-insensitive key lookup silently fails; double-\n bug in KeyValue serialization.

**How to apply:** Additive Phase 1 first (new API in castep_cell_serde), then bottom-up Phase 2 migration of castep_cell_data, then Phase 3 cleanup (delete old serde layer, rename crate).

Phases:
- Phase 1: Add query.rs, parse.rs, format.rs to castep_cell_serde
- Phase 2: Migrate castep_cell_data bottom-up (units → species → param → cell blocks → tests)
- Phase 3: Delete de/, ser.rs, serde impls from error.rs; rename crate; update Cargo.tomls

Key files:
- castep_cell_serde/src/ — lib.rs, parser.rs, ser.rs, error.rs, de/
- castep_cell_data/src/ — units/, cell/, param/, lib.rs
- Root Cargo.toml — workspace members
- castep_cell_data/Cargo.toml — dep on castep_cell_serde
- castep-param-io/Cargo.toml — dep on castep_cell_serde (deprecated, must still compile)

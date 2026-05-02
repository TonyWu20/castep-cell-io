## Plan Review Decisions — PHASE_PLAN — 2026-05-03

### Design Assessment

The plan is fundamentally sound. It faithfully follows established architectural patterns: flat `Option<T>` fields on `CellDocument`, `bon::Builder` for struct types, `KEY_ALIASES`/`BLOCK_ALIASES` for plural-form handling, `FromBlock`/`FromKeyValue`/`ToCell` trait implementations, and proper `mod.rs` wiring. The BS_/SPECTRAL_ separation as distinct keyword types (rather than aliasing hacks) is architecturally correct — they represent different CASTEP keyword prefixes with potentially different semantics over time.

### Deferred Item Decisions

None — this is the first phase after a branch reset; no prior deferred.md files exist.

### Plan Amendments Applied

1. **Specify parsing mechanism**: Added to Design Notes: "In `from_cell_file`, use `FromKeyValue::from_cells(tokens)?` for all key-value types... For block types, use `find_block_any(tokens, &[...])`."

2. **Lattice mutual exclusion**: Added to Goal 2: "If both `LATTICE_CART` and `LATTICE_ABC` blocks are present in the input, return an error indicating only one lattice specification is allowed."

3. **BS_ aliases on spectral types**: Updated Alias patterns with explicit examples: "SpectralKpointPath gets `BLOCK_ALIASES = &["BS_KPOINT_PATH", "BS_KPOINTS_PATH", "SPECTRAL_KPOINTS_PATH"]`". Net-new types without BS_ equivalents noted as not needing BS_ aliases.

4. **Clarify LatticeABC pre-existence**: Added to Goal 2: "LatticeABC already exists in `cell/lattice_param.rs` with `FromBlock`, `ToCell`, `bon::Builder`, and test coverage — the net-new work is the enum variant, `From` impls, `ToCell` match arm, and `from_cell_file` wiring."

5. **Resolve PhononKpointPathEntry**: Open questions section replaced with Resolved Questions. `PhononFineKpointPath` uses the existing `PhononKpointPathEntry`. `SpectralKpointPathEntry` kept separate from `BsKpointPathEntry`.

## Plan Review Decisions — magical-waddling-parnas — 2026-05-05

### Design Assessment

The plan's core approach — bon fallible builder via `finish_fn` — is architecturally sound. Four gaps were identified: (1) contradictory instruction about the lattice parse-time check, (2) missing spectral/BS_ duplication handling (D2), (3) unspecified builder protocol for `from_cell_file` transition, and (4) no concrete test plan. All are fixable and have been incorporated into the plan.

### Deferred Item Decisions

#### D2: Mutual-exclusion validation between spectral and BS_ types
**Decision:** Absorb
**Rationale:** The current `from_cell_file` unconditionally populates both `spectral_kpoint_path` and `bs_kpoint_path` when BS_ block names are encountered. Without absorbing this, parsed documents with BS_ aliases would pass ^4 validation with duplicate data.
**Action:** Added `bs_kpoint_path` and `bs_kpoints_list` to the ^4 mutual exclusion count. Added parse-time guards in `from_cell_file` to skip BS_ parsing when the spectral equivalent is already populated.

#### D6: `#[non_exhaustive]` on `Lattice` enum
**Decision:** Defer again
**Rationale:** Adding `#[non_exhaustive]` changes downstream match semantics and is a cross-cutting concern affecting all public enums (Lattice, Positions, unit enums, etc.). Should be applied in a dedicated API stabilization pass before 1.0, not piggybacked onto validation work.
**Action:** Updated precondition — apply during the API stabilization milestone immediately preceding 1.0.

### Plan Amendments

- Added `bs_kpoint_path` and `bs_kpoints_list` to the ^4 validation check (D2 absorption)
- Added parse-time guards in `from_cell_file` to prevent BS_ duplication (D2 absorption)
- Clarified that the lattice check at lines 486-491 is **kept** for error-message quality
- Specified builder protocol: `maybe_*` setters for optional fields, `Into` for required fields
- Added concrete test plan with 7 unit tests and 1 integration test
- Verified `IsComplete` trait path compatibility with bon v3.9.1

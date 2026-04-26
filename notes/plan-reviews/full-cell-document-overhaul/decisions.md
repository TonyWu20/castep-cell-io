## Plan Review Decisions — full-cell-document-overhaul — 2026-04-27

### Design Assessment

The plan is fundamentally sound. The sub-group decomposition follows the proven `ParamDocument` pattern, builder pattern via `bon` with `Default` for groups is correct, and the `validate()` pattern for each group plus inter-group validation in `CellDocument` is well-structured. The backward-compatibility principle (keep existing BS_ types, emit BS_ on serialization) is correct. The separate SPECTRAL_ structs approach (not just aliases) enables clean mutual-exclusion validation at the group level.

**Key gaps identified and addressed:**
1. Missing KPOINT/KPOINTS alias infrastructure — already done (commit 04f4d1e), integrated into plan
2. Missing `KEY_ALIASES` on `PhononKpointPathSpacing` — added as Part 1f
3. `CellConstraints` not publicly exported from `constraints/mod.rs` — addressed in Part 5
4. `LATTICE_ABC` parsing missing from `CellDocument` — added to Part 2/4
5. Pre-existing bug: `fix_com`/`fix_all_cell`/`fix_all_ions` value discarded during parsing — noted in Part 3
6. Part 5 export strategy clarified per existing param module convention

### Deferred Item Decisions

None — no deferred items from prior phases.

### Plan Amendments

Applied:
1. **Context** — Added "Already done" section documenting KPOINT/KPOINTS alias infrastructure (commit 04f4d1e)
2. **All keyword types** — Added `BLOCK_ALIASES`/`KEY_ALIASES` specifications to every new type in Parts 1a, 1b, 1d, 1e
3. **Part 1f** — New section: fix missing `KEY_ALIASES` on `PhononKpointPathSpacing`
4. **Part 2** — Added `LATTICE_ABC` parsing support in `CellDocument::from_cell_file`
5. **Part 3** — Updated `SpectralParams`: clarified each type uses its own `from_cells` with `BLOCK_ALIASES`/`KEY_ALIASES`; fixed `fix_com`/`fix_all_cell`/`fix_all_ions` parsing is a pre-existing bug the sub-group approach fixes
6. **Part 4** — Added `LATTICE_ABC` parsing note
7. **Part 5** — Clarified export strategy: `LatticeABC` exported from `lib.rs`; sub-group types accessible via module hierarchy (not top-level re-exports, consistent with param module pattern)
8. **Verification** — Added alias round-trip tests and `LATTICE_ABC` parsing to check list
9. **Part 1b (SpectralKpointPath)** — Added entry type sharing decision: separate `SpectralKpointPathEntry` for now (keeps BS_/SPECTRAL_ split clean); share later if structurally identical
10. **Part 3 (ConstraintsParams)** — Added diagnostic note: `cell_constraints` silently overrides `fix_all_cell`; document the behavior, consider warning mechanism
11. **Part 3 (SpectralParams validation)** — Fleshed out validation approach: group fields by type, count set bits per exclusion group, reject if >1

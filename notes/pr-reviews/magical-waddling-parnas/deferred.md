# Deferred Items: Add Mutual Exclusion Validation to CellDocument Builder

Items identified during review that are worth doing but out of scope for this phase.

---

### D1: Integration tests for spectral ^4, phonon ^5, symmetry ^6 rules via parse pipeline

**Source**: Strategic review concern C2
**Rationale**: The integration test (`tests/mutual_exclusion_validation.rs`) only exercises the kpoints ^3 rule through the full parse pipeline. The other three validation rules (spectral ^4+D2, phonon ^5, symmetry ^6) are only tested at the builder level via unit tests, not end-to-end through `parse::<CellDocument>()`.

**Suggested approach**: Add integration test functions for:
- `SPECTRAL_KPOINT_PATH` + `SPECTRAL_KPOINTS_MP_GRID` in the same `.cell` input
- `PHONON_KPOINT_PATH` + `PHONON_KPOINT_LIST` in the same `.cell` input
- `SYMMETRY_OPS` + `SYMMETRY_GENERATE` in the same `.cell` input

### D2: Phonon fine-variant mutual-exclusion validation

**Source**: Strategic review concern C4
**Rationale**: `phonon_fine_kpoint_path` and `phonon_fine_kpoint_list` have no mutual-exclusion validation, unlike their non-fine counterparts. The directions explicitly scope the phonon mutual-exclusion to the non-fine pair only, but it's worth validating whether CASTEP semantics require mutual exclusion for fine kpoint fields too.

**Suggested approach**: Research CASTEP documentation for fine phonon field semantics, then add validation rules and tests if required.

### D3: Error message naming consistency

**Source**: Strategic review minor observation
**Rationale**: Builder validation error messages use Rust field names (e.g., `kpoints_list`, `kpoints_mp_grid`) while the existing lattice parse-time error uses CASTEP block names (`LATTICE_CART`, `LATTICE_ABC`). Consider unifying to one convention.

**Suggested approach**: Decide on a convention (either Rust field names or CASTEP block names) and update error messages for consistency.

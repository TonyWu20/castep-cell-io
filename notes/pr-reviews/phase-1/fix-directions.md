# Fixes: Phase 1 — New Keyword Types + Flat CellDocument Wiring

**Source branch**: issue-8
**Fixes for**: 1 issue

## Summary

One issue found during code review: spectral block types parsed after BS_ block types in `from_cell_file`. Fix is required.

---

### Issue 1: Spectral block types parsed after BS_ blocks

**Classification:** Correctness
**Severity:** Minor
**File:** `castep_cell_io/src/cell_document.rs`
**Problem:** In `from_cell_file`, spectral block types (`spectral_kpoint_path`, `spectral_kpoints_list`) are parsed AFTER BS_ block types (`bs_kpoint_path`, `bs_kpoints_list`). The directions explicitly require spectral to be parsed before BS_ (P0 mitigation for BS_ alias overlap). While `find_block_any` is read-only (both find the same block), the incorrect ordering violates documented architectural guidance.
**Fix:** Reorder parsing in `from_cell_file` so spectral block types are parsed BEFORE BS_ block types. Move spectral block parsing (currently after `kpoints_mp_offset` and before `bs_kpoint_path`) to after `magres_kpoints_list` and before `bs_kpoint_path`. Key-value spectral types can remain in their current position (they have no BS_ alias overlap). Do NOT change `to_cell_file` serialization order.
**Acceptance:** `cargo check -p castep_cell_io` and `cargo test -p castep_cell_io`

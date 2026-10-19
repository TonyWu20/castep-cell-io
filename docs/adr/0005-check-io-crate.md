# 0005 — `castep-check-io` crate for binary model files

The workspace gains a third member, `castep-check-io`. It reads, edits, and writes the binary CASTEP model files `.check` (full model, with wavefunction) and `.castep_bin` (analysis model, no wavefunction, `CASTEP_BIN` magic record). The public type is `CheckDocument`, mirroring the `CellDocument`/`ParamDocument` document pattern: `from_file`, `to_file`, builder via `bon`.

## Why a crate, not a module in `castep-cell-io`

The binary model file is a different document kind from `.cell`/`.param`. The text files carry user-set options. The binary files carry the full resolved state: parameters, cell, wavefunction, eigenvalues, density, and optional analysis sections. The author, lifecycle, and churn are different. A crate boundary keeps binary layout churn off the text keyword release track. The version record and layout change per CASTEP release, independent of the `.cell`/`.param` keyword set.

## Relationship to the other crates

`castep-check-io` depends on `castep-cell-fmt` by path. It reuses `CastepVersion` and the release features from the fmt crate. It does not depend on `castep-cell-io`. The binary document types are self-contained; no text keyword type is shared.

The fmt crate remains the single source of truth for the CASTEP release set. Feature wiring mirrors `castep-cell-io`:

```toml
# castep_check_io/Cargo.toml
[features]
default = ["castep-6-11"]
castep-6-11 = ["castep-cell-fmt/castep-6-11"]
castep-23 = ["castep-cell-fmt/castep-23"]
```

A build that enables no release feature is a compile error, as in `castep-cell-fmt`.

## Version handling

The binary file stores its writer version in the second record of the parameters section: a `char(10)` written `f6.3` from the writer's version number, e.g. ` 6.110` for CASTEP 6.11.

**Read policy.** The reader parses the version record into `(major, minor)` and keeps the raw string. It accepts the file when the file version is at or below the build target and the reader knows the file's layout. The known layout floor is 3.0. Two rejection errors carry the file version and the target: `VersionTooNew` and `VersionTooOld`. The error for `VersionTooNew` names the feature that must be enabled once a later release lands.

**Write policy.** The writer always emits the target release layout. That is the highest enabled release, mirroring ADR-0004 serialization semantics. The version record gets the target version string. A byte-exact roundtrip holds when the file version equals the target version.

**Where the version gates live.** The CASTEP 6.11 source gates records on version in exactly one place: the parameters section. `parameters_dump` writes every record unconditionally. `parameters_restore` reads records under `if (local_version_number >= X.Y)` gates, 44 gates at 13 levels (3.0, 3.1, 3.2, 4.0, 4.1, 4.2, 4.3, 4.4, 5.0, 5.5, 6.0, 6.1, plus the base set). The cell section and the electronic sections carry no version gates. The reader encodes the gate table. The writer emits the full target layout.

**The MD block carries one more rule.** After the `md_thermostat` record, both writer and reader write exactly two legacy f64 records in a fixed slot. The slot values depend on the thermostat. For `NOSE-HOOVER`/`HOOVER-LANG` they are `(md_ion_t, 10*md_ion_t)`. For `LANGEVIN` they are `(0.1*md_ion_t, md_ion_t)`. The reader stores the raw pair. The writer emits the stored pair without recomputing, so a roundtrip stays byte-exact.

**The placeholder feature.** `castep-23` enables no binary layout yet. The reader rejects a file version above the target with `VersionTooNew` either way. When a real later release lands, its layout module lands under its own feature and moves out of the placeholder, exactly as ADR-0004 moves keyword types.

## Consequences

Three crates share one release feature set and one `CastepVersion`. A later-release change touches the fmt enum, then the text keyword sets, then the binary layout modules. Each crate releases on its own track.

`castep-check-io` pulls `castep-cell-fmt` into a binary-only consumer's dependency graph. The cost is small. The text parser is a light crate. The benefit is one version source of truth across the workspace.

## Layout facts fixed during implementation

The section layouts were extracted from the CASTEP 6.11 source and validated
record-for-record against real model files. These facts are binding for the
codec:

- **Record framing.** Every record is `4B BE length | payload | 4B BE length`.
  Physical sizes: i32 is 4 bytes, f64 is 8 bytes, complex128 is 16 bytes,
  a logical is 4 bytes (0 or 1), `char(n)` is n bytes, space-padded on the
  right. The `.castep_bin` kind adds one leading 10-byte `CASTEP_BIN` magic
  record.
- **Parameters.** The section is 340 records in the 6.11 layout: the
  `BEGIN_PARAMETERS_DUMP` tag, the version record, 23 block tag pairs, and
  all value records of `parameters_dump`. The table in `params.rs`
  (`PARAM_FIELDS`) names every field, its kind, its target size, and its
  version gate. The table was generated from the source dump and checked
  byte-exact against `Cu111_CO.castep_bin`.
- **Cell sections.** `cell_dump_cell` and `cell_dump_global` emit
  `[256-byte tag][value record]` pairs. Value sizes derive from counts read
  earlier in the file. Four value pairs are conditional on their count being
  nonzero: `SYMMETRY_*` (on `num_symmetry_operations`),
  `CRYSTAL_SYMMETRY_*` (on `num_crystal_symmetry_operations`),
  `IONIC_CONSTRAINTS` (on `num_ionic_constraints`), and
  `PHONON_GAMMA_DIRECTIONS` (on `num_phonon_gamma_directions`). A closed
  gate omits the tag record and the value record together. The global
  section inherits `num_species` and `max_ions_in_species` from the
  preceding unit cell section.
- **Wavefunction block.** Present in `.check` files only. Layout
  (`wave_write_all_ser`): 4-byte kind tag (`wave` or `Gpt `), three i32
  grid dimensions, four i32 wave dimensions, then per spin and k-point: a
  28-byte k-point record (three f64 coordinates plus the `waves_at_kp`
  count), three plane-wave grid coordinate records (i32 grid indices,
  not f64), and one complex128 record per band.
- **Electronic section order.** `found_gs_wvfn` (4B), `found_gs_den` (4B),
  `total_energy` (8B), the first Fermi level (8B), `nbands_max` and
  `nspins` (one 8B record), the wavefunction block in `.check` files, the
  eigenvalue records (per k-point: 24B coordinates, then per spin one
  f64 record of occupancies and one of eigenvalues), a second
  `found_gs_den` record, the 12B fine-grid dimensions, the density column
  records (per column: i32 x, i32 y, then per z one complex128 charge
  value and, for two-spin runs, one complex128 spin value), the optional
  sections, and the 10-byte `END` record. The four leading scalar records
  are four separate records, not one.
- **Optional sections.** Each starts with a 10-byte header record. The
  known headers are the ones `model_write_all` writes: `E_FERMI`,
  `OEP_POT`, `DE_DLOGE`, `FORCES`, `STRESS`, `MD`, `PIMD`, `MD_NOSE`,
  `MD_NOSE_2`, `BFGS`, `LBFGS`, `TPSD`, `DAMPED_MD`, `FIRE`, `VAR_CELL`,
  `PHONON`, `PHON_GAMMA`, `SECONDD`, `FORCE_CON`, `FINE_FREQ`,
  `BORN_CHGS`, `PART_CHGS`, `DIELECTRIC`, `RAMAN_TENS`, `SHIELDING`,
  `EFG`, `GTENSOR`, `HYPERFINE`, `JCOUP`, `JCOUP_FC`, `JCOUP_SPIN`,
  `JCOUP_ORB`, `GENETIC`, `LDAUOCCM`. The reader types the small fixed
  headers (`E_FERMI`, `FORCES`, `STRESS`, `MD`) and consumes any other
  section raw, up to the next 10-byte known header or `END`.
- **Roundtrip proof.** A byte-exact `from_bytes` to `to_bytes` roundtrip
  holds for `Cu111_CO.castep_bin`, `Cu111_CO.check` (with wavefunction),
  `Cu111_CO_F7.check`, `Fe2O3.castep_bin`, and a live 0614 spin-job
  `.check`. The tests live in `castep_check_io/tests/roundtrip.rs`.
- **Local build variants are a layout axis, not a version axis.**
  `ParamsLayout` carries the record widths a local build can change inside
  one release. The first knob is `devel_code_len`, standard 80, some local
  builds 256. The reader and the writer each take a layout. A document
  stores the layout it was read with, and `with_layout` retargets the
  writer. The writer pads `devel_code` to the target width and refuses to
  truncate a wider stored value (`LayoutOverflow`). The config is
  authoritative. The crate ships no auto-detection of file widths.
  One width names one build, and the command and the config apply it to
  the read and the write alike. Cross-width conversion is a library flow
  (read, `with_layout`, write), not a command flag. The `toml` dependency
  parses the layout config file. The `castep-check-io` command exposes the
  width as the `--devel-code-len` flag and a `--layout-config` file.
- **Local build variants break the read side, not the layout.** The Sep 30
  `~/Downloads/CASTEP-6.11-nixos` build carries a local patch that widens
  `devel_code` from the standard 80 bytes to `file_maxpath` (256 bytes).
  It aborts in `parameters_restore` on any standard-layout model file
  ("I/O past end of record on unformatted file"), because its reader
  expects the 256-byte record. The `/export/castep-rust-eigensolve`
  build keeps the standard 80-byte record, and its source differs from
  the 6.11 reference only in diagnostics and the `OCC_WEIGHTED` mixing
  keyword, so it is the write-oracle binary. This matters for any CASTEP
  binary used to validate the codec: confirm the build reads a known-good
  standard-layout file before blaming the writer.

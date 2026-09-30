# 0004 — CASTEP version gating via Cargo features

`castep-cell-fmt` and `castep-cell-io` declare the target CASTEP release through Cargo `[features]`. There is one feature per release, named `castep-<major>-<minor>`. Hyphens stand in for dots, since Cargo feature names cannot hold a dot. `castep-6-11` is the first release. `castep-23` is a placeholder for later releases that have not been pinned to a release number yet.

Both crates declare the features:

```toml
# castep_cell_fmt/Cargo.toml
[features]
default = ["castep-6-11"]
castep-6-11 = []
castep-23 = []

# castep_cell_io/Cargo.toml  (propagates to the fmt path-dependency)
[features]
default = ["castep-6-11"]
castep-6-11 = ["castep-cell-fmt/castep-6-11"]
castep-23 = ["castep-cell-fmt/castep-23"]
```

The placeholder `castep-23` is not a default feature. A default build targets 6-11 only. Enabling it adds the later-release keyword set on top.

`castep-cell-io` depends on `castep-cell-fmt` by path. The features flow down to the fmt crate, which owns the version.

## The version type

`castep_cell_fmt/src/version.rs` owns the release set. `CastepVersion` is a `#[repr(u16)]` enum. It has one variant per enabled release. `V6_11` encodes the release as the code `611`. `V23` is the placeholder for later releases, encoded as `2300`, so it always sorts above every current `6.x` release.

A `SUPPORTED` static lists the enabled releases, in ascending order, one definition per enabled combination. `CastepVersion::target()` returns the highest one, which is the build's target release. `supported()` returns the union a build accepts. The type, its impls, and `SUPPORTED` are gated on the release features. A build that enables no release feature fails with a single `compile_error!`. That keeps a no-release build a compile-time error, not a silent one.

Semantics: parsing accepts every keyword of every enabled release. Serialization emits the target release's spelling, which is the highest enabled one. With only `castep-6-11` enabled, the target is the 6-11 release. With the placeholder `castep-23` also enabled, the target becomes `V23` and the later-release keyword set is unlocked.

`castep-cell-io` re-exports `CastepVersion` as `castep_cell_io::CastepVersion`. Downstream code can read the active target without reaching into the fmt crate.

## The 6.11 keyword set, from source not docs

The keyword set comes from the CASTEP 6-11 source distribution, not the website docs. The reference files are `Fundamental/cell.f90` and `Fundamental/parameters.f90`.

Eight cell keywords are present in the release but were missing from the crate. They were added. Each follows the crate style: a newtype, a `bon::Builder`, and a custom `Default` or validation. Each data shape was checked against the Fortran reader.

| Keyword | 6.11 `typ` | File form | Rust type / group |
| --- | --- | --- | --- |
| `HUBBARD_ALPHA` | `B:B` | block, optional units line (default `ev`) | `HubbardAlpha` / `SpeciesParams` |
| `CHEMICAL_POTENTIAL` | `B:B` | block, optional units line (default `ev`) | `ChemicalPotential` / `SpeciesParams` |
| `SPECIES_GAMMA` | `B:B` | block, optional units line (default `radsectesla`) | `SpeciesGamma` / `SpeciesParams` |
| `CELL_NOISE` | `P:B` | inline `KEY : value unit` (default `0.0 ang`) | `CellNoise` / `DynamicsParams` |
| `POSITIONS_NOISE` | `P:B` | inline `KEY : value unit` (default `0.0 ang`) | `PositionsNoise` / `DynamicsParams` |
| `JCOUPLING_SITE` | `S:B` | inline `KEY : species ion` | `JcouplingSite` / `OpticsMagresParams` |
| `SNAP_TO_SYMMETRY` | `D:B` | bare presence flag | `SnapToSymmetry` / `SymmetryParams` |
| `ATOMIC_INIT` | `D:D` | bare marker, no payload | `AtomicInit` / `CellDocument` |

## Data-shape fixes against the earlier doc-based draft

Two of the new types were wrong in the doc-based draft. Both were fixed against the 6.11 reader.

`JCOUPLING_SITE` is an inline string keyword, not a block. Its reader is `io_freeform_string`. The freeform tokenizer only admits a `%BLOCK` label when the keyword's first type letter is `B`. `JCOUPLING_SITE` is `S:B`, so it is written inline as `JCOUPLING_SITE : Fe 2`. The parser reads it as a `Cell::KeyValue`. The type implements `FromKeyValue`, not `FromBlock`.

A `HUBBARD_ALPHA` row needs the ion number. The reader aborts when the second token is not an integer. So `AtomHubbardAlpha::ion_number` is a plain `u32`. This differs from `HUBBARD_U`, where the ion number is optional. The row's orbital specs are letter-value pairs, written joined or separate. A colon can join a letter to its value.

## What is gated, and what is not yet

The release features gate two things. The first is the `CastepVersion` set in `version.rs`, which controls the target release. The second is a set of keyword types in the io crate, which are not present in the 6.11 source. Those types are gated behind the placeholder `castep-23` feature with `#[cfg(feature = "castep-23")]`.

Each gated item has three gates. The keyword module declaration in its parent `mod.rs` is gated. The group-struct field, its `from_cell_file` builder call, and its `to_cell_file` line are gated. Any test that references a gated field is gated. A default 6-11 build compiles with every gate closed, with zero warnings.

The eight types for the `castep-6-11` release are added unconditionally. They are the baseline release and are always present.

The gated keyword set is the Rust `KEY_NAME`/`BLOCK_NAME` constants that are absent from the 6.11 source:

```
BOUNDARY_TYPE, DIELEC_EMB_BULK_PERMITTIVITY, DIELEC_EMB_FUNC_METHOD,
EFIELD_CALCULATE_NONLINEAR, GEOM_PRECONDITIONER,
IMPLICIT_SOLVENT_APOLAR_FACTOR, IMPLICIT_SOLVENT_APOLAR_TERM,
IMPLICIT_SOLVENT_SURFACE_TENSION, POPN_WRITE, RELATIVISTIC_TREATMENT,
SEDC_CUSTOM_PARAMS, SEDC_D_G06, SEDC_D_JCHS, SEDC_D_TS, SEDC_LAMBDA_OBS,
SEDC_N_OBS, SEDC_S6_G06, SEDC_S6_JCHS, SEDC_SR_JCHS, SEDC_SR_TS,
SUPERCELL_KPOINT_LIST_CASTEP, TDDFT_POSITION_METHOD, TSSEARCH_ENERGY_TOL,
TSSEARCH_MAX_PATH_POINTS, USE_SMEARED_IONS, WRITE_CHECKPOINT, XC_DEFINITION
```

Two notes on this list. `ATOM_HUBBARD_U` and `ATOM_HUBBARD_ALPHA` are row sub-type constants of the 6.11 `HUBBARD_U` and `HUBBARD_ALPHA` blocks, so they stay ungated. `EFIELD_IGNORE_MOL_MODES` is not on this list: it is a 6.11 keyword, not a post-6.11 one. The 6.11 source registers it as `EFIELD_IGNORE_MOLEC_MODES`, but the crate used the short `EFIELD_IGNORE_MOL_MODES` spelling. That keyword was renamed to the 6.11 spelling `EFIELD_IGNORE_MOLEC_MODES` and stays ungated.

When a real later release feature, such as `castep-6-12`, is added, its keywords move from `castep-23` to that feature. The placeholder `castep-23` then holds only the keywords of releases still without a pinned release number.

## Alias policy

An alias is a keyword spelling the target release actually accepts. The crate accepts a `KEY_ALIASES` or `BLOCK_ALIASES` entry only when the target release's source registers that spelling. An alias the release does not accept is wrong for that release and is omitted.

For the 6-11 target, a spelling is valid only if it appears in the 6.11 source as a registered keyword or block. `QUANTISATION_AXIS` and `QUANTIZATION_AXIS` both appear in 6.11, so the `QUANTIZATION_AXIS` type keeps `QUANTISATION_AXIS` as an alias. `EFIELD_IGNORE_MOL_MODES` does not appear in 6.11. The 6.11 keyword is `EFIELD_IGNORE_MOLEC_MODES`, so that type takes no alias.

6.11 registers both singular and plural forms only for the `KPOINT_LIST` and `KPOINT_PATH` families. It registers only the singular form for the `MP_GRID`, `MP_OFFSET`, and `MP_SPACING` families, all `PHONON_FINE` forms, and `MAGRES`. Ten k-point types carry a plural alias that 6.11 does not register. Those aliases are gated behind the placeholder `castep-23` feature, so a 6.11 build accepts no such alias and the `castep-23` build does. When a real later release is confirmed to accept a plural form, that alias moves behind that release's feature.

## Consequences

A build targets exactly the highest enabled release. The default build is 6-11 only. Enabling the placeholder `castep-23` makes `target()` the `V23` placeholder and unlocks the later-release keyword set.

When a real later release feature, such as `castep-6-12`, is added, it changes `SUPPORTED` and `target()`. Its keywords then move out of the `castep-23` placeholder onto that specific feature. The 6.11 set is left untouched.

The no-release build is a hard compile error. It gives one clean diagnostic.

The Fortran source is the authoritative keyword reference for the release. The website docs are no longer the source of truth on this branch.

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.0] - 2026-09-30

### Added
- `castep-6-11` (default) and `castep-23` release features in both crates select the target CASTEP release
- `CastepVersion` in `castep-cell-fmt`, re-exported by `castep-cell-io`, lists supported releases and the target
- 8 cell keywords verified against the CASTEP 6.11 source: `HUBBARD_ALPHA`, `CELL_NOISE`, `POSITIONS_NOISE`, `CHEMICAL_POTENTIAL`, `JCOUPLING_SITE`, `ATOMIC_INIT`, `SNAP_TO_SYMMETRY`, `SPECIES_GAMMA`
- New `GammaUnit` type for `SPECIES_GAMMA`
- 27 post-6.11 keyword types available under the `castep-23` placeholder feature

### Changed
- **BREAKING**: renamed `EfieldIgnoreMolModes` to `EfieldIgnoreMolecModes`. The keyword is `EFIELD_IGNORE_MOLEC_MODES`, the spelling 6.11 registers
- **BREAKING**: the 27 post-6.11 keyword types and 10 plural k-point aliases are off by default. Enable `castep-23` to accept them
- Alias policy: the crate accepts an alias only when the target release registers that spelling
- `castep-cell-io` now depends on `castep-cell-fmt` 0.3.0

## [0.6.0] - 2026-05-14

### Changed
- **BREAKING**: Migrated `CellDocument` from flat single-struct to group substructs (`CellDocumentGroups`) for logical organization
- **BREAKING**: `CellDocument` now stores `LatticeCart`, `KpointsParams`, `SymmetryOps`, `CellConstraints`, `SpeciesPot`, `IonicPositions`, `Specie`, `PhononKpoints`, `SpectralKpoints`, `BlockComment`, and `CellComment` instead of flat fields
- Builder API updated to reflect new nested group structure

### Fixed
- Length-unit values now emit `CellValue::Str` instead of `CellValue::String` for round-trip compatibility
- Missing `validate()` calls added in `CellDocument::build()`
- `mp_grid` + `mp_spacing` mutual exclusion validation enforced in `KpointsParams`

### Added
- Fixture-anchored tests for `CellDocumentGroups` migration (Fe2O3, ZnO_LR, Co3O4)
- Round-trip test for `LatticeCart` confirming row-major convention

## [0.5.0] - 2026-05-05

### Changed
- **BREAKING**: Converted `CellDocument` builder to fallible with 4 mutual-exclusion validation rules:
  `kpoints_list`/`kpoints_mp_grid`/`kpoints_mp_spacing`, spectral/BS k-points,
  `phonon_kpoint_path`/`phonon_kpoint_list`, `symmetry_generate`/`symmetry_ops` — now
  mutually exclusive at build time
- **BREAKING**: `CellDocument::builder().build()` now returns `CResult<CellDocument>`
  instead of `CellDocument`
- Converted `from_cell_file` to use builder pattern with BS_ duplication guarding

### Added
- Integration tests for mutual exclusion validation of k-point specifications

### Fixed
- BS_ k-point block duplication: `BS_KPOINT_PATH`/`BS_KPOINTS_LIST` parsing now suppressed
  when `SPECTRAL_KPOINT_PATH`/`SPECTRAL_KPOINTS_LIST` is present

## [0.4.0] - 2026-04-06

### Changed
- **BREAKING**: Refactored `ParamDocument` from flat 180-field struct into 18 nested sub-structs for better organization and maintainability
- **BREAKING**: Field access changed from `doc.field` to `doc.group.field` (e.g., `doc.task` → `doc.general.task`)
- **BREAKING**: Builder API now requires nested builders (e.g., `ParamDocument::builder().general(GeneralParams::builder()...build()).build()`)
- Removed dependency on experimental `bon` feature `experimental-overwritable`
- Improved type-state checking with smaller, focused builders

### Added
- 18 new parameter group modules: `GeneralParams`, `ElectronicParams`, `BasisSetParams`, `ExchangeCorrelationParams`, `ElectronicMinimisationParams`, `GeometryOptimizationParams`, `PhononParams`, `BandStructureParams`, `MolecularDynamicsParams`, `ElectricFieldParams`, `PseudopotentialParams`, `DensityMixingParams`, `PopulationAnalysisParams`, `OpticsParams`, `NmrParams`, `SolvationParams`, `ElectronicExcitationsParams`, `TransitionStateParams`
- Convenience methods for frequently accessed fields (e.g., `doc.task()`, `doc.xc_functional()`)
- Intra-group validation methods for each parameter group
- Inter-group validation for mutual exclusivity constraints

### Fixed
- Builder type-state safety restored by removing experimental feature

## [0.3.0] - 2025-01-15

### Added
- Initial release with flat `ParamDocument` structure
- Support for all CASTEP .param file keywords
- Builder pattern using `bon` crate with experimental features

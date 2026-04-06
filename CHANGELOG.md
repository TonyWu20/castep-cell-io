# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

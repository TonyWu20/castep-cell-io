# castep-cell-io — Context

A Rust I/O library for CASTEP `.cell` and `.param` input files. Enables type-safe, programmatic construction, parsing, and formatting of CASTEP input files for computational chemistry and materials science workflows.

## Language

### File Formats

**Cell file** (`.cell`):
CASTEP input file defining the system geometry: lattice vectors, ionic positions, species, and optional block-level constraints (symmetry, k-points, phonon, etc.).
_Avoid_: cell input, structure file, geometry file

**Param file** (`.param`):
CASTEP input file defining calculation parameters: task type, exchange-correlation functional, basis set cutoffs, convergence criteria, and other algorithmic settings.
_Avoid_: parameter file, input settings

### Syntactic Constructs (IR — `castep-cell-fmt`)

**Block**:
A multi-line structured section in `.cell` files, delimited by `%block BLOCK_NAME` / `%endblock BLOCK_NAME`. Contains rows of typed cell values. Represented as `Cell::Block(name, Vec<CellValue>)` in the IR.
_Avoid_: section

**KeyValue**:
A single-line keyword = value pair, used in both `.cell` and `.param` files. Represented as `Cell::KeyValue(key, CellValue)` in the IR.
_Avoid_: setting pair, assignment

**Flag**:
A keyword present or absent without a value (`.cell` files only). Represented as `Cell::Flag(name)` in the IR.
_Avoid_: toggle, boolean flag

**CellValue**:
The typed value in a Cell IR node. Variants: Null, Bool, Str, String, UInt, Int, Float, Array.
_Avoid_: token, literal

### Domain Types (`castep-cell-io`)

**Keyword**:
A named parameter in `.param` files (e.g., `Task`, `CutOffEnergy`, `XCFunctional`). Each maps to a single Rust type implementing `FromKeyValue` / `ToCell`.
_Avoid_: parameter, setting, key

**Param Group**:
A logical collection of related keywords aggregated into a single struct (e.g., `GeneralParams`, `ExchangeCorrelationParams`). Uses `bon::Builder`. Aggregated by `ParamDocument`.
_Avoid_: section, namespace, param module

**Cell Block Type**:
A typed Rust representation of a `.cell` block (e.g., `PositionsFrac`, `SpeciesPot`, `Kpoint`, `SymmetryOp`). Each implements `FromBlock` / `ToCell`. Held by `CellDocument` (directly or via a group sub-struct).
_Avoid_: block struct, block group

**Cell Document Group**:
A logical collection of related cell block types aggregated into a single struct (e.g., `KPointsGroup`, `ConstraintsGroup`, `SymmetryGroup`). Mirrors the Param Group pattern for `CellDocument`'s ~48 optional block fields. Uses `bon::Builder`.
_Avoid_: block category, cell sub-document

### Foundational Domain Concepts

**Species**:
A chemical element type, identified either by atomic symbol (`"Cu"`, `"Mg"`) or by atomic number (`29`, `12`). Represented as the `Species` enum (Symbol or AtomicNumber).
_Avoid_: element, atom type, element type

**Positions**:
Atomic coordinates within the unit cell. Two representations: Fractional (direct lattice coordinates) or Absolute (Cartesian coordinates in Å).
_Avoid_: coordinates, sites, atomic positions

**Lattice**:
The unit cell geometry. Two representations: Cartesian (three lattice vectors) or ABC (three lengths + three angles). Represented as the `Lattice` enum (`LatticeCart` | `LatticeABC`).
_Avoid_: cell parameters, box, cell vectors

**ParamDocument**:
The top-level document type representing a complete `.param` file. Composed of 18 required Param Group fields (each with `bon::Builder`, all `Option<T>` internally). Validates inter-group mutual exclusion (e.g., band-count parameter conflicts).
_Avoid_: param file representation, param doc

**CellDocument**:
The top-level document type representing a complete `.cell` file. Composed of `Lattice` + `Positions` (required) plus ~48 optional Cell Block Type fields (being migrated to Cell Document Groups). Validates mutual exclusion (e.g., conflicting k-point specifications).
_Avoid_: cell file representation, cell doc

### Physical Units

**LengthUnit, EnergyUnit, ForceUnit, PressureUnit, etc.**:
Typed unit enums in `castep_cell_io::units`. Each enum defines valid CASTEP unit variants (e.g., for Length: Bohr, Angstrom, cm, m). Used to annotate keyword values where units are optional in the file format. The library does **not** perform automatic unit conversion — it records which unit is specified.
_Avoid_: unit type, measurement unit

### Trait Hierarchy — Parsing

**FromCellValue**:
Leaf-level parse trait: converts a single `CellValue` into a Rust type. Implemented for primitives (`f64`, `u32`, `i32`, `bool`, `String`, `[T; N]`) and all keyword/block entry types.
_Avoid_: deserialize, value parser

**FromKeyValue**:
Key-value parse trait: looks up a keyword by `KEY_NAME` / `KEY_ALIASES` in the token slice, extracts its `CellValue`, and delegates to `FromCellValue`. Implemented for all keyword types.
_Avoid_: keyword parser, key lookup

**FromBlock**:
Block parse trait: finds a block by `BLOCK_NAME` / `BLOCK_ALIASES` in the token slice, parses its rows, and constructs the domain type. Implemented for all cell block types.
_Avoid_: block parser, section deserializer

**FromCellFile**:
Top-level parse trait: assembles a complete domain type (e.g., `CellDocument`, `ParamDocument`) from the full `Vec<Cell>` token slice. Orchestrates calls to `FromKeyValue` and `FromBlock` for constituent fields.
_Avoid_: file parser, document deserializer

### Trait Hierarchy — Serialization

**ToCellValue**:
Leaf-level serialize trait: converts a Rust type to a single `CellValue`.
_Avoid_: serialize, value formatter

**ToCell**:
Mid-level serialize trait: converts a domain type (keyword type, cell block type) to a single `Cell` IR node (KeyValue or Block).
_Avoid_: format, emit

**ToCellFile**:
Top-level serialize trait: converts a complete document type to `Vec<Cell>`. Composition of `ToCell` calls on constituent fields, filtering out `None` fields.
_Avoid_: serialize, document formatter

## Relationships

- A **Cell file** contains zero or more **Blocks**, zero or more **KeyValues**, and zero or more **Flags**
- A **Param file** contains zero or more **KeyValues** (no Blocks or Flags)
- A **Block** contains one or more rows of **CellValues**
- A **KeyValue** contains exactly one **CellValue**
- A **Cell Block Type** is the typed Rust representation of a **Block**
- A **Keyword** is the typed Rust representation of a **KeyValue** (in `.param` context)
- A **Param Group** collects related **Keywords** into a single struct
- A **Cell Document Group** collects related **Cell Block Types** into a single struct
- A **ParamDocument** is composed of 18 **Param Groups**
- A **CellDocument** is composed of **Lattice**, **Positions**, and ~48 optional **Cell Block Types** (being organized into **Cell Document Groups**)
- **FromCellFile** is implemented for **ParamDocument** and **CellDocument**
- **ToCellFile** is implemented for **ParamDocument** and **CellDocument**
- Each **Keyword** implements **FromKeyValue** + **ToCell**
- Each **Cell Block Type** implements **FromBlock** + **ToCell**

## Example Dialogue

> **Dev:** "When I add a new param keyword type under `castep_cell_io::param`, what do I need to implement?"
>
> **Domain expert:** "Create a new file with the keyword type struct, implement `FromKeyValue` + `ToCell`, add `KEY_NAME`, and wire it into the corresponding **Param Group** builder. The group struct's `ToCellFile` implementation will pick it up automatically."
>
> **Dev:** "Same pattern for a new block type in `cell/`?"
>
> **Domain expert:** "Yes — but use `FromBlock` + `ToCell` instead, set `BLOCK_NAME`, and wire it into the appropriate **Cell Document Group** (or directly into `CellDocument` until the group migration is complete)."
>
> **Dev:** "And I'd write tests against the real fixture file, right? Not with synthetic data?"
>
> **Domain expert:** "Against the fixture — parse the real `.cell` or `.param`, assert concrete values with a `(Source: ...)` citation. That's the ODD rule. No vacuous assertions, no circular round-trips."

## Flagged Ambiguities

- **"Block"** is used for both the syntactic construct (`%block...%endblock`, a `Cell::Block` in the IR) and the domain type (`Cell Block Type` like `PositionsFrac`). Resolution: CONTEXT.md documents both usages explicitly. In code, the two are distinguished by context — IR code operates on `Cell::Block`, domain code operates on named struct types. No rename needed.

- **"Keyword"** was used interchangeably with "parameter" and "setting" in early development. Resolution: **Keyword** is the canonical term for individual `.param` entries. **Param Group** for collections.

## Architecture

### Crate Boundaries

The workspace has two crates with a strict unidirectional dependency:

```
castep-cell-io
  depends on: castep-cell-fmt
  owns: domain types, block types, param groups, unit enums,
        document-level validation, ToCellFile/FromCellFile impls

castep-cell-fmt
  depends on: (none internal)
  owns: IR (Cell, CellValue), chumsky parser, formatter,
        trait hierarchy (traits + primitive impls), query helpers
```

**Rule**: `castep-cell-fmt` has zero knowledge of domain types. It works only with the IR. `castep-cell-io` implements the traits defined in `castep-cell-fmt` for its domain types.

### Module Organization (`castep-cell-io`)

```
src/
  lib.rs           — pub use re-exports
  cell_document.rs  — CellDocument, Lattice, Positions enums
  param_document.rs — ParamDocument, validation
  cell/
    mod.rs
    positions/      — PositionsFrac, PositionsAbs, entry types
    species/        — Species enum, SpeciesPot, HubbardU, etc.
    symmetry/       — SymmetryOps, SymmetryGenerate, SymmetryTol
    bz_sampling_kpoints/ — Kpoint, KpointsList, BsKpointPath, etc.
    constraints/    — IonicConstraints, FixAllIons, FixAllCell, etc.
    external_fields/ — ExternalEfield, ExternalPressure
    lattice_param/  — LatticeCart, LatticeABC
    phonon/         — PhononKpointList, PhononSupercellMatrix, etc.
    velocities/     — IonicVelocities
  param/
    mod.rs
    general/        — Task, Comment, Continuation, RandSeed, ...
    exchange_correlation/ — XcFunctional, SpinPolarized, ...
    basis_set/      — CutOffEnergy, GridScale, FineGmax, ...
    electronic/     — NBands, Nup, SedcSRTs, ...
    electronic_minimisation/ — ElecEnergyTol, MaxCGSteps, ...
    geometry_optimization/ — GeomForceTol, GeomConvergenceWin, ...
    phonon/         — PhononMaxCycles, PhononCutoffEnergy, ...
    band_structure/ — BsKpointPathBs, BsMaxBands, ...
    molecular_dynamics/ — MxMdIonT, MxMdIonL, ...
    efield/         — EfieldPolarisation, EfieldPolarBlock, ...
    pseudopotential/ — PseudopotentialPolicy, PseudopotentialGenerate, ...
    density_mixing/ — DensityMixingScheme, MixingCutoffEnergy, ...
    population_analysis/ — CalculateDensity, CalculatePDOS, ...
    optics/         — OpticsCorrelation, OpticsNonlinear, ...
    nmr/            — NmrCalculate, NmrGIA, ...
    solvation/       — SolvationModel, SolventDielectric, ...
    electronic_excitations/ — ElecExcitation, ElecExcitationCutoff, ...
    transition_state/ — TransitionStateSearch, TSType, ...
    *params/         — Aggregation structs: general_params, exchange_correlation_params, etc.
  units/
    mod.rs
    length.rs, energy.rs, force.rs, pressure.rs, etc.
```

### Key Architectural Decisions

| Decision | Rationale |
|----------|-----------|
| Two-crate separation | `fmt` owns the parser/IR (changes with format spec); `io` owns domain types (changes with CASTEP versions). Isolates churn. |
| chumsky for parsing | Combinator-based parsing matches line-oriented CASTEP format well. Text-first, no binary formats. Allowed state-machine via `then`/`or`. |
| bon builders mandatory | Consistent ergonomics across all struct construction. Fallible builders for documents with validation. |
| thiserror for lib errors | Typed error enum in `fmt`. `io` maps validation failures to `Error::Message(String)`. |
| No async | Library is purely synchronous — file I/O + format conversion. No async benefit. |
| serde for external interchange | Used on types that may be serialized to config formats (JSON, YAML). Not used for CASTEP file format — that goes through ToCell/FromCellValue. |

## Coding Patterns

| Area | Convention |
|------|------------|
| Builders | `#[derive(bon::Builder)]` on all non-trivial structs. Fallible `build()` for documents with validation. |
| Error handling | `thiserror` in `fmt`. `Result<_, String>` at document level → mapped to `Error::Message(...)`. |
| Tests | Inline `#[cfg(test)] mod tests` in every source file. Integration tests in `tests/` for document-level validation. |
| Style | Functional: iterators over for-loops. Minimize mutable state. Immutable transformations. |
| Visibility | `pub use` re-exports from `lib.rs`. Internal modules are private. |
| Module structure | One type per file. Group files in subdirectories by logical category. |

## Pipeline Expectations

**ODD (Outcome-Driven Development)** replaces TDD throughout this project. All tests must anchor to ground truth (fixture files), not synthetic data or vacuous assertions.

### Stage Order

1. `/init-project` (done — this document)
2. `/drive-outcomes` for each phase (defines success criteria → explores → implements → verifies against fixtures)
3. `/make-judgement` after each task group for peer review

### First Phase

**Complete the `CellDocument` migration to Cell Document Groups** — mirroring the successful `ParamDocument` v0.4.0 refactoring. Organize the ~48 flat optional cell block fields into logical sub-groups, ensuring all keyword types defined in `cell/` are accessible through `CellDocument`.

### Ground Truth Fixtures

- `castep_cell_fmt/Mg2SiO4_Cr_1.cell` — forsterite with Cr doping (`.cell`)
- `Co3O4_2.param` — cobalt oxide param (`.param`)
- **Need more**: Both `.cell` and `.param` fixture files must be expanded for comprehensive ground truth anchoring across all block types and keyword categories.

### Fixture Anchoring Rules

Every test assertion must:
1. Read from a real fixture file
2. Assert concrete expected values with a `(Source: fixture_file:line)` citation
3. Be falsifiable — the test must fail when the implementation is wrong

Prohibited patterns:
- Vacuous assertions (`is_finite()`, `is_ok()` without value check)
- Circular round-trips (synthetic data → encode → decode → assert)
- Placeholder bounds without source citation
- Synthetic data that mirrors parser expectations

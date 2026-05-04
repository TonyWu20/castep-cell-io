# Phase 1 Draft Elaboration

**Generated:** 2026-05-03
**From:** plans/phase-1/PHASE_PLAN.md, notes/directions/phase-1/deferred-and-patterns.md, notes/directions/phase-1/codebase-state.md

---

## Overview

This document elaborates the architectural design for each of the 5 goals in Phase 1. For each goal: design decisions, crate boundary decisions, pattern requirements, type signatures, known pitfalls, and task constraints.

All implementation is in `castep_cell_io/src/cell/` (domain types crate). No changes to `castep_cell_fmt`. The `CellDocument` + `Lattice`/`Positions` enums live in `castep_cell_io/src/cell_document.rs`, re-exported from `lib.rs`.

---

## Shared Wiring Pattern (all goals touch this)

Every new keyword type follows the **export chain**: type file -> `mod` + `pub use` in parent `mod.rs` -> import in `cell_document.rs` -> struct field + parse block + serialize block.

### Key-value type wiring in `from_cell_file`

Use `FromKeyValue::from_cells(cells)?` which returns `CResult<Option<Self>>`. This method is provided by the trait (in `castep_cell_fmt/src/parse.rs` line 64) and handles alias resolution automatically. It returns `Ok(None)` when the key (and all aliases) are absent -- perfect for optional fields.

```rust
let kpoints_mp_grid = KpointsMpGrid::from_cells(cells)?;
```

Do NOT use the raw `find_map` pattern on `Cell::KeyValue` for key-value types. That pattern duplicates alias-resolution logic and is only for flag-like types parsed via `has_flag`.

### Block type wiring in `from_cell_file`

Use `find_block_any(cells, &[...])` for blocks with aliases, or `find_block(cells, "NAME")` for blocks without aliases. Both return `CResult<&[CellValue]>` -- use `.ok()` + `.map(|rows| T::from_block_rows(rows)).transpose()?` for optional blocks.

```rust
let cell_constraints = find_block(cells, "CELL_CONSTRAINTS")
    .ok()
    .map(|rows| CellConstraints::from_block_rows(rows))
    .transpose()?;
```

### Serialization in `to_cell_file`

Follow the existing pattern:

```rust
if let Some(kmg) = &self.kpoints_mp_grid {
    cells.push(kmg.to_cell());
}
```

For flag types (SymmetryGenerate), push `Cell::Flag("NAME")` directly.

### Insertion order in `to_cell_file`

Maintain the existing block ordering:
1. Lattice, positions (required)
2. K-point sampling (bz_sampling_kpoints)
3. Constraints and flags
4. Symmetry (after constraints, before external fields)
5. External fields
6. Species properties
7. Phonon blocks
8. Dynamics (ionic velocities)

New fields slot into their existing section. For example, new bz_sampling_kpoints fields go in the k-point sampling section, new phonon fields in the phonon section.

---

## Goal 1: Fix gaps, wire orphan types

### Work items

| # | Item | File | Action |
|---|------|------|--------|
| 1a | `pub use CellConstraints` | `cell/constraints/mod.rs` | Add `pub use cell_constraints::CellConstraints;` |
| 1b | Add `KEY_ALIASES` | `cell/phonon/phonon_kpoint_path_spacing.rs` | Add `const KEY_ALIASES: &'static [&'static str] = &["PHONON_KPOINTS_PATH_SPACING"];` |
| 1c | Wire 6 orphan types | `cell_document.rs` | Add struct fields, parsing, serialization |

### 1a: CellConstraints pub export

`cell/constraints/mod.rs` line 1 declares `mod cell_constraints;` but has no `pub use` for `CellConstraints`. The type has `FromBlock`, `ToCell`, `bon::Builder`, and 7 unit tests. It's fully functional but invisible to external consumers.

**Fix:** Add one line:
```rust
pub use cell_constraints::CellConstraints;
```

Place it between the existing `fix_vol` and `ionic_constraints` pub uses (alphabetically).

### 1b: PhononKpointPathSpacing KEY_ALIASES

The type currently has `KEY_NAME = "PHONON_KPOINT_PATH_SPACING"` with no aliases. CASTEP accepts `PHONON_KPOINTS_PATH_SPACING` (plural). Without the alias, plural-form inputs silently fail to parse.

**Fix:** Add to the `FromKeyValue` impl:
```rust
const KEY_ALIASES: &'static [&'static str] = &["PHONON_KPOINTS_PATH_SPACING"];
```

### 1c: Wire 6 orphan types into CellDocument

**Type-by-type wiring specification:**

#### KpointsMpGrid (key-value, `[u32; 3]`)

```rust
// Struct field in CellDocument
pub kpoints_mp_grid: Option<KpointsMpGrid>,

// Parsing in from_cell_file (before kpoints_list)
let kpoints_mp_grid = KpointsMpGrid::from_cells(cells)?;

// Serialization in to_cell_file (before kpoints_list, after bs_kpoints_list)
if let Some(kmg) = &self.kpoints_mp_grid {
    cells.push(kmg.to_cell());
}
```

Import: `KpointsMpGrid` (already exported from `bz_sampling_kpoints/mod.rs` line 15).

#### KpointsMpSpacing (key-value, `{ value: f64, unit: Option<InvLengthUnit> }`)

```rust
pub kpoints_mp_spacing: Option<KpointsMpSpacing>,

let kpoints_mp_spacing = KpointsMpSpacing::from_cells(cells)?;

if let Some(kms) = &self.kpoints_mp_spacing {
    cells.push(kms.to_cell());
}
```

Import: `KpointsMpSpacing` (already exported from `bz_sampling_kpoints/mod.rs` line 14).

#### BsKpointPathSpacing (key-value, `{ value: f64, unit: Option<InvLengthUnit> }`)

```rust
pub bs_kpoint_path_spacing: Option<BsKpointPathSpacing>,

let bs_kpoint_path_spacing = BsKpointPathSpacing::from_cells(cells)?;

if let Some(bps) = &self.bs_kpoint_path_spacing {
    cells.push(bps.to_cell());
}
```

Import: `BsKpointPathSpacing` (already exported from `bz_sampling_kpoints/mod.rs` line 13).

#### CellConstraints (block, `{ lengths: [u32; 3], angles: [u32; 3] }`)

```rust
pub cell_constraints: Option<CellConstraints>,

let cell_constraints = find_block(cells, "CELL_CONSTRAINTS")
    .ok()
    .map(|rows| CellConstraints::from_block_rows(rows))
    .transpose()?;

if let Some(cc) = &self.cell_constraints {
    cells.push(cc.to_cell());
}
```

Import: `CellConstraints` (requires Goal 1a fix first). No aliases needed -- `CELL_CONSTRAINTS` has no plural variant.

#### FixVOL (key-value, `bool`)

```rust
pub fix_vol: Option<FixVOL>,

let fix_vol = FixVOL::from_cells(cells)?;

if let Some(fv) = &self.fix_vol {
    cells.push(fv.to_cell());
}
```

Import: `FixVOL` (already exported from `constraints/mod.rs` line 12).

**Note on FixVOL KEY_ALIASES:** The codebase-state.md flagged this as a gap, but `FIX_VOL` has no plural variant (`FIX_VOLS` is not a valid CASTEP keyword). The current `KEY_ALIASES = &[]` (from the trait default) is correct. No fix needed.

#### SymmetryTol (key-value, `{ value: f64, unit: LengthUnit }`)

```rust
pub symmetry_tol: Option<SymmetryTol>,

let symmetry_tol = SymmetryTol::from_cells(cells)?;

if let Some(st) = &self.symmetry_tol {
    cells.push(st.to_cell());
}
```

Import: `SymmetryTol` (already exported from `symmetry/mod.rs` line 5).

**Note on SymmetryTol KEY_ALIASES:** The codebase-state.md flagged this, but `SYMMETRY_TOL` has no plural variant. The default `&[]` is correct.

### Build struct update

In the `Ok(CellDocument { ... })` return, add the 6 new fields (alphabetically within their sections):
```rust
cell_constraints,
fix_vol,
kpoints_mp_grid,
kpoints_mp_spacing,
bs_kpoint_path_spacing,
symmetry_tol,
```

### Integration test impact

The existing `#[ignore]` integration test in `cell_document.rs` (line 682-699) will need updating to assert on new fields once test fixtures exist. Out of scope for this phase.

### Design decisions

- **Use `FromKeyValue::from_cells`** for all key-value wiring. This is the first time this pattern is used in `CellDocument` and establishes the canonical approach for Goals 3-5.
- **FixVOL does not need KEY_ALIASES**: No plural form exists. The codebase-state.md flag was a false positive.
- **SymmetryTol does not need KEY_ALIASES**: Same reasoning.
- **CellConstraints wiring**: Simple `find_block` (no aliases needed). The `%BLOCK CELL_CONSTRAINTS` name has no variant form.

---

## Goal 2: Lattice::Abc variant + parsing

### Work items

| # | Item | File |
|---|------|------|
| 2a | Add `Abc(LatticeABC)` variant | `cell_document.rs` (Lattice enum) |
| 2b | Add `From<LatticeABC> for Lattice` + `From<LatticeCart> for Lattice` | `cell_document.rs` |
| 2c | Update `ToCell for Lattice` | `cell_document.rs` |
| 2d | Wire `LATTICE_ABC` parsing in `from_cell_file` | `cell_document.rs` |
| 2e | Dual-lattice error detection | `cell_document.rs` |

### Type signatures

**Current Lattice enum:**
```rust
pub enum Lattice {
    Cart(LatticeCart),
}
```

**After changes:**
```rust
pub enum Lattice {
    Cart(LatticeCart),
    Abc(LatticeABC),
}
```

### 2b: From impls

The `CellDocument` builder uses `#[builder(on(Lattice, into))]`, which means `lattice(...)` accepts anything implementing `Into<Lattice>`. We need:

```rust
impl From<LatticeCart> for Lattice {
    fn from(v: LatticeCart) -> Self { Lattice::Cart(v) }
}

impl From<LatticeABC> for Lattice {
    fn from(v: LatticeABC) -> Self { Lattice::Abc(v) }
}
```

### 2c: ToCell update

```rust
impl ToCell for Lattice {
    fn to_cell(&self) -> Cell<'_> {
        match self {
            Lattice::Cart(cart) => cart.to_cell(),
            Lattice::Abc(abc) => abc.to_cell(),
        }
    }
}
```

`LatticeABC` already has a `ToCell` impl that produces `Cell::Block("LATTICE_ABC", ...)`.

### 2d: from_cell_file parsing

Replace the current hardcoded `LATTICE_CART` parsing:

```rust
// Current (before):
let lattice_rows = find_block(cells, "LATTICE_CART")?;
let lattice = Lattice::Cart(LatticeCart::from_block_rows(lattice_rows)?);
```

**New logic:**

```rust
let has_lattice_cart = find_block(cells, "LATTICE_CART").is_ok();
let has_lattice_abc = find_block(cells, "LATTICE_ABC").is_ok();

if has_lattice_cart && has_lattice_abc {
    return Err(Error::Message(
        "Both LATTICE_CART and LATTICE_ABC are specified. \
         Only one lattice specification is allowed.".into(),
    ));
}

let lattice = if has_lattice_cart {
    let rows = find_block(cells, "LATTICE_CART")?;
    Lattice::Cart(LatticeCart::from_block_rows(rows)?)
} else {
    let rows = find_block(cells, "LATTICE_ABC")?;
    Lattice::Abc(LatticeABC::from_block_rows(rows)?)
};
```

This preserves the existing error behavior: if neither block is present, `find_block(cells, "LATTICE_ABC")` fails with `KeyNotFound("LATTICE_ABC")`, matching the current behavior of `find_block(cells, "LATTICE_CART")` failing.

### Design decisions

- **`has_lattice_cart && has_lattice_abc` check before parsing**: Prevents partial parsing errors (e.g., successfully parsing LATTICE_CART then failing on LATTICE_ABC). Fails fast with a clear message.
- **Default to LATTICE_ABC when neither specified?** No. The plan is explicit: return an error if neither is present, same as current behavior for missing LATTICE_CART.
- **No builder changes needed**: The existing `#[builder(on(Lattice, into))]` on `CellDocument` handles the new variant automatically.
- **No changes to `LatticeABC`**: The type already has `FromBlock`, `ToCell`, `bon::Builder`, and 7 unit tests in `cell/lattice_param.rs`.

---

## Goal 3: KpointsMpOffset + SymmetryGenerate

### Work items

| # | Item | File (new or existing) |
|---|------|------------------------|
| 3a | Create `KpointsMpOffset` type | `cell/bz_sampling_kpoints/kpoints_mp_offset.rs` (new) |
| 3b | Register in mod | `cell/bz_sampling_kpoints/mod.rs` |
| 3c | Create `SymmetryGenerate` type | `cell/symmetry/symmetry_generate.rs` (new) |
| 3d | Register in mod | `cell/symmetry/mod.rs` |
| 3e | Wire both into CellDocument | `cell_document.rs` |

### 3a: KpointsMpOffset type signature

```rust
// cell/bz_sampling_kpoints/kpoints_mp_offset.rs

use castep_cell_fmt::{Cell, CellValue, ToCell, ToCellValue};
use castep_cell_fmt::parse::{FromCellValue, FromKeyValue};
use castep_cell_fmt::{CResult, Error};
use castep_cell_fmt::query::value_as_f64;

/// Specifies the Monkhorst-Pack grid offset for k-point sampling.
///
/// Keyword type: Real array
///
/// Format: kpoints_mp_offset <ox> <oy> <oz>
///
/// Example:
/// KPOINTS_MP_OFFSET : 0.25 0.25 0.25
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KpointsMpOffset(pub [f64; 3]);

impl FromCellValue for KpointsMpOffset {
    fn from_cell_value(value: &CellValue<'_>) -> CResult<Self> {
        match value {
            CellValue::Array(arr) if arr.len() == 3 => {
                let offset = [
                    value_as_f64(&arr[0])?,
                    value_as_f64(&arr[1])?,
                    value_as_f64(&arr[2])?,
                ];
                Ok(KpointsMpOffset(offset))
            }
            _ => Err(Error::Message(
                "KpointsMpOffset must be an array of 3 floats".into(),
            )),
        }
    }
}

impl FromKeyValue for KpointsMpOffset {
    const KEY_NAME: &'static str = "KPOINT_MP_OFFSET";
    const KEY_ALIASES: &'static [&'static str] = &["KPOINTS_MP_OFFSET"];

    fn from_cell_value_kv(value: &CellValue<'_>) -> CResult<Self> {
        Self::from_cell_value(value)
    }
}

impl ToCell for KpointsMpOffset {
    fn to_cell(&self) -> Cell<'_> {
        Cell::KeyValue(
            "KPOINT_MP_OFFSET",
            CellValue::Array(
                self.0.iter().map(|&v| CellValue::Float(v)).collect(),
            ),
        )
    }
}

impl ToCellValue for KpointsMpOffset {
    fn to_cell_value(&self) -> CellValue<'_> {
        CellValue::Array(
            self.0.iter().map(|&v| CellValue::Float(v)).collect(),
        )
    }
}
```

**No `bon::Builder`** -- simple tuple newtype with one field. Matches the `KpointsMpGrid` pattern.

Tests (in `#[cfg(test)] mod tests`): from_cell_value (valid, insufficient elements, too many elements, non-array), to_cell_value, to_cell, round-trip, key_name, key_aliases. Follow the `KpointsMpGrid` test structure exactly.

### 3c: SymmetryGenerate type signature

`SymmetryGenerate` is a pure flag -- when present in a `.cell` file, it enables automatic symmetry generation. It carries no data.

```rust
// cell/symmetry/symmetry_generate.rs

use castep_cell_fmt::{Cell, ToCell};

/// Flag to enable automatic symmetry generation.
///
/// When present, CASTEP will automatically generate symmetry operations
/// from the input structure rather than using those explicitly specified
/// in SYMMETRY_OPS.
///
/// Flag type - no associated value.
///
/// Example:
/// SYMMETRY_GENERATE
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SymmetryGenerate;

impl ToCell for SymmetryGenerate {
    fn to_cell(&self) -> Cell<'_> {
        Cell::Flag("SYMMETRY_GENERATE")
    }
}
```

**No `FromKeyValue` / `FromCellValue` impls** -- this is not parsed via `FromKeyValue::from_cells`. Instead, it's parsed via `has_flag` in `CellDocument`.

### 3e: CellDocument wiring

```rust
// Parsing (in from_cell_file, in the symmetry section after symmetry_ops):
let symmetry_generate = if has_flag(cells, "SYMMETRY_GENERATE") {
    Some(SymmetryGenerate)
} else {
    None
};

let kpoints_mp_offset = KpointsMpOffset::from_cells(cells)?;

// Serialization (in to_cell_file, after kpoints_mp_spacing):
if let Some(kmo) = &self.kpoints_mp_offset {
    cells.push(kmo.to_cell());
}

// Serialization (in to_cell_file, after symmetry_tol):
if self.symmetry_generate.is_some() {
    cells.push(Cell::Flag("SYMMETRY_GENERATE"));
}
```

**Import additions to `cell_document.rs`:**
- From `bz_sampling_kpoints`: add `KpointsMpOffset`
- From `symmetry`: add `SymmetryGenerate`

**Builder keyword:** `symmetry_generate` and `kpoints_mp_offset` are auto-generated by `bon::Builder` since they are `Option<T>` fields on the struct.

### Design decisions

- **`KpointsMpOffset` is a tuple newtype**: `pub struct KpointsMpOffset(pub [f64; 3])`. Consistent with `KpointsMpGrid(pub [u32; 3])`. No bon builder needed.
- **`SymmetryGenerate` is a unit struct**: No data, just presence/absence. Parsed via `has_flag` (not `FromKeyValue`) because it's a `Cell::Flag` variant, not `Cell::KeyValue`.
- **No KEY_ALIASES for SymmetryGenerate**: Single canonical name, no plural form. `has_flag` already does case-insensitive matching.

---

## Goal 4: Spectral k-point types (6 new files)

### Work items

| # | Item | File (all new in `cell/bz_sampling_kpoints/`) |
|---|------|-----------------------------------------------|
| 4a | `SpectralKpointPath` + `SpectralKpointPathEntry` | `spectral_kpoint_path.rs` |
| 4b | `SpectralKpointsList` | `spectral_kpoints_list.rs` |
| 4c | `SpectralKpointPathSpacing` | `spectral_kpoint_path_spacing.rs` |
| 4d | `SpectralKpointsMpGrid` | `spectral_kpoints_mp_grid.rs` |
| 4e | `SpectralKpointsMpSpacing` | `spectral_kpoints_mp_spacing.rs` |
| 4f | `SpectralKpointsMpOffset` | `spectral_kpoints_mp_offset.rs` |
| 4g | Register all 6 in mod | `cell/bz_sampling_kpoints/mod.rs` |
| 4h | Wire all 6 into CellDocument | `cell_document.rs` |

### Type signatures

#### 4a: SpectralKpointPath (block type)

```rust
#[derive(Debug, Clone, Copy, PartialEq, bon::Builder)]
pub struct SpectralKpointPathEntry {
    pub coord: [f64; 3],
}
// Impls: FromCellValue, ToCellValue

#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct SpectralKpointPath {
    #[builder(default)]
    pub points: Vec<SpectralKpointPathEntry>,
}
// Impls: FromBlock, ToCell
```

`SpectralKpointPathEntry` is structurally identical to `BsKpointPathEntry` but kept separate per the plan's prefix-boundary decision. Consolidation deferred.

**Block aliases:**
```rust
impl FromBlock for SpectralKpointPath {
    const BLOCK_NAME: &'static str = "SPECTRAL_KPOINT_PATH";
    const BLOCK_ALIASES: &'static [&'static str] = &[
        "SPECTRAL_KPOINTS_PATH",
        "BS_KPOINT_PATH",
        "BS_KPOINTS_PATH",
    ];
    // ...
}
```

The two BS_ aliases provide backward compatibility for legacy CASTEP input files.

#### 4b: SpectralKpointsList (block type)

```rust
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct SpectralKpointsList {
    #[builder(default)]
    pub kpts: Vec<Kpoint>,  // reuses existing Kpoint from bz_sampling_kpoints::kpoint
}
// Impls: FromBlock, ToCell
```

**Block aliases:**
```rust
const BLOCK_NAME: &'static str = "SPECTRAL_KPOINT_LIST";
const BLOCK_ALIASES: &'static [&'static str] = &[
    "SPECTRAL_KPOINTS_LIST",
    "BS_KPOINT_LIST",
    "BS_KPOINTS_LIST",
];
```

**`Kpoint` reuse:** The existing `Kpoint { coord: [f64; 3], weight: f64 }` from `bz_sampling_kpoints/kpoint.rs` is imported via `use super::Kpoint;`.

#### 4c: SpectralKpointPathSpacing (key-value type)

```rust
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SpectralKpointPathSpacing {
    pub value: f64,
    pub unit: Option<InvLengthUnit>,
}
// Impls: FromCellValue, FromKeyValue, ToCell, ToCellValue
```

**Key aliases:**
```rust
const KEY_NAME: &'static str = "SPECTRAL_KPOINT_PATH_SPACING";
const KEY_ALIASES: &'static [&'static str] = &[
    "SPECTRAL_KPOINTS_PATH_SPACING",
    "BS_KPOINT_PATH_SPACING",
    "BS_KPOINTS_PATH_SPACING",
];
```

Structurally identical to `BsKpointPathSpacing`. Follow the same `FromCellValue` logic (accepts scalar float or array `[value, unit]`).

**No `bon::Builder`** -- matches the `BsKpointPathSpacing` pattern (2-field struct).

#### 4d: SpectralKpointsMpGrid (key-value, newtype)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpectralKpointsMpGrid(pub [u32; 3]);
// Impls: FromCellValue, FromKeyValue, ToCell, ToCellValue
```

**Key aliases:**
```rust
const KEY_NAME: &'static str = "SPECTRAL_KPOINT_MP_GRID";
const KEY_ALIASES: &'static [&'static str] = &["SPECTRAL_KPOINTS_MP_GRID"];
```

No BS_ alias -- this is a net-new type with no BS_ equivalent.

**No `bon::Builder`** -- tuple newtype, matches `KpointsMpGrid`.

#### 4e: SpectralKpointsMpSpacing (key-value)

```rust
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SpectralKpointsMpSpacing {
    pub value: f64,
    pub unit: Option<InvLengthUnit>,
}
// Impls: FromCellValue, FromKeyValue, ToCell, ToCellValue
```

**Key aliases:**
```rust
const KEY_NAME: &'static str = "SPECTRAL_KPOINT_MP_SPACING";
const KEY_ALIASES: &'static [&'static str] = &["SPECTRAL_KPOINTS_MP_SPACING"];
```

No BS_ alias -- net-new type. **No `bon::Builder`** -- matches `KpointsMpSpacing`.

#### 4f: SpectralKpointsMpOffset (key-value, newtype)

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpectralKpointsMpOffset(pub [f64; 3]);
// Impls: FromCellValue, FromKeyValue, ToCell, ToCellValue
```

**Key aliases:**
```rust
const KEY_NAME: &'static str = "SPECTRAL_KPOINT_MP_OFFSET";
const KEY_ALIASES: &'static [&'static str] = &["SPECTRAL_KPOINTS_MP_OFFSET"];
```

No BS_ alias -- net-new type. **No `bon::Builder`**.

### 4g: Module registration

Add to `cell/bz_sampling_kpoints/mod.rs`:
```rust
mod spectral_kpoint_path;
mod spectral_kpoints_list;
mod spectral_kpoint_path_spacing;
mod spectral_kpoints_mp_grid;
mod spectral_kpoints_mp_spacing;
mod spectral_kpoints_mp_offset;

pub use spectral_kpoint_path::{SpectralKpointPath, SpectralKpointPathEntry};
pub use spectral_kpoints_list::SpectralKpointsList;
pub use spectral_kpoint_path_spacing::SpectralKpointPathSpacing;
pub use spectral_kpoints_mp_grid::SpectralKpointsMpGrid;
pub use spectral_kpoints_mp_spacing::SpectralKpointsMpSpacing;
pub use spectral_kpoints_mp_offset::SpectralKpointsMpOffset;
```

### 4h: CellDocument wiring

```rust
// New struct fields (6 new fields):
pub spectral_kpoint_path: Option<SpectralKpointPath>,
pub spectral_kpoints_list: Option<SpectralKpointsList>,
pub spectral_kpoint_path_spacing: Option<SpectralKpointPathSpacing>,
pub spectral_kpoints_mp_grid: Option<SpectralKpointsMpGrid>,
pub spectral_kpoints_mp_spacing: Option<SpectralKpointsMpSpacing>,
pub spectral_kpoints_mp_offset: Option<SpectralKpointsMpOffset>,
```

**Parsing additions** (after `magres_kpoints_list`, before `symmetry_ops`):

```rust
let spectral_kpoint_path = find_block_any(cells, &["SPECTRAL_KPOINT_PATH", "SPECTRAL_KPOINTS_PATH", "BS_KPOINT_PATH", "BS_KPOINTS_PATH"])
    .ok()
    .map(|rows| SpectralKpointPath::from_block_rows(rows))
    .transpose()?;

let spectral_kpoints_list = find_block_any(cells, &["SPECTRAL_KPOINT_LIST", "SPECTRAL_KPOINTS_LIST", "BS_KPOINT_LIST", "BS_KPOINTS_LIST"])
    .ok()
    .map(|rows| SpectralKpointsList::from_block_rows(rows))
    .transpose()?;

let spectral_kpoint_path_spacing = SpectralKpointPathSpacing::from_cells(cells)?;
let spectral_kpoints_mp_grid = SpectralKpointsMpGrid::from_cells(cells)?;
let spectral_kpoints_mp_spacing = SpectralKpointsMpSpacing::from_cells(cells)?;
let spectral_kpoints_mp_offset = SpectralKpointsMpOffset::from_cells(cells)?;
```

**PITFALL (CRITICAL):** The `SpectralKpointPath` block aliases include `"BS_KPOINT_PATH"` and `"BS_KPOINTS_PATH"`, and `SpectralKpointsList` block aliases include `"BS_KPOINT_LIST"` and `"BS_KPOINTS_LIST"`. This means they share block names with the existing `BsKpointPath` and `BSKpointList` types. If both a spectral field and a BS_ field are populated from the same input block, we'd get duplicate data.

**Resolution:** Parse spectral types FIRST, then BS_ types. If a block matches a spectral alias, consume it for the spectral field; if not, let the BS_ parser try next. But `find_block_any` is a read-only query -- it doesn't "consume" tokens. So both parsers will find the same block.

**Correct resolution:** The order matters. Parse spectral types first with their full alias set (including BS_ aliases). Then parse BS_ types with their own alias set. If an input file uses BS_KPOINT_PATH, it will populate both `spectral_kpoint_path` AND `bs_kpoint_path`. This is acceptable because:
1. The BS_ aliases on spectral types are for backward compatibility with legacy files
2. Real-world files will use EITHER BS_ prefix OR SPECTRAL_ prefix, not both
3. Mutual exclusion validation is deferred

**However**, this means the current parsing order in `CellDocument` must be changed. Spectral types should be parsed before BS_ types, or alternatively, spectral types should NOT include BS_ aliases (contrary to the plan). Let me flag this as a design conflict.

**RECOMMENDATION:** Keep BS_ aliases on spectral types (per plan) but parse spectral types BEFORE BS_ types. Document that this means a file using BS_ prefix blocks will populate both fields. This is acceptable for now since validation is deferred. If this is undesirable, we can discuss removing BS_ aliases from spectral types.

**Serialization additions** (after `magres_kpoints_list`, before `symmetry_ops`):
```rust
if let Some(sp) = &self.spectral_kpoint_path {
    cells.push(sp.to_cell());
}
if let Some(sl) = &self.spectral_kpoints_list {
    cells.push(sl.to_cell());
}
if let Some(sps) = &self.spectral_kpoint_path_spacing {
    cells.push(sps.to_cell());
}
if let Some(smg) = &self.spectral_kpoints_mp_grid {
    cells.push(smg.to_cell());
}
if let Some(sms) = &self.spectral_kpoints_mp_spacing {
    cells.push(sms.to_cell());
}
if let Some(smo) = &self.spectral_kpoints_mp_offset {
    cells.push(smo.to_cell());
}
```

**Import additions:** Add all 6 types + `SpectralKpointPathEntry` to the `bz_sampling_kpoints` import block.

### Design decisions

- **SpectralKpointPathEntry separate from BsKpointPathEntry**: Per plan, kept separate for clean prefix boundary. Consolidation deferred.
- **SpectralKpointsList reuses `Kpoint`**: Same as `BSKpointList`. No separate entry type needed.
- **BS_ backward compat aliases on path/list/path_spacing**: Per plan, spectral types include BS_ aliases. See pitfall above about dual-population.
- **No BS_ aliases on net-new types**: `MpGrid`, `MpSpacing`, `MpOffset` have no BS_ equivalent. Only KPOINTS plural alias.
- **No `bon::Builder` on simple types**: Tuple newtypes (MpGrid, MpOffset) and 2-field spacing structs follow the existing convention of manual construction.

---

## Goal 5: Phonon MP and fine types (8 new files)

### Work items

| # | Item | File (all new in `cell/phonon/`) |
|---|------|----------------------------------|
| 5a | `PhononKpointsMpGrid` | `phonon_kpoints_mp_grid.rs` |
| 5b | `PhononKpointsMpSpacing` | `phonon_kpoints_mp_spacing.rs` |
| 5c | `PhononKpointsMpOffset` | `phonon_kpoints_mp_offset.rs` |
| 5d | `PhononFineKpointPath` | `phonon_fine_kpoint_path.rs` |
| 5e | `PhononFineKpointPathSpacing` | `phonon_fine_kpoint_path_spacing.rs` |
| 5f | `PhononFineKpointsMpGrid` | `phonon_fine_kpoints_mp_grid.rs` |
| 5g | `PhononFineKpointsMpSpacing` | `phonon_fine_kpoints_mp_spacing.rs` |
| 5h | `PhononFineKpointsMpOffset` | `phonon_fine_kpoints_mp_offset.rs` |
| 5i | Register all 8 in mod | `cell/phonon/mod.rs` |
| 5j | Wire all 8 into CellDocument | `cell_document.rs` |

### Type signatures

#### 5a: PhononKpointsMpGrid (key-value, newtype)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhononKpointsMpGrid(pub [u32; 3]);
// Impls: FromCellValue, FromKeyValue, ToCell, ToCellValue
```

Key aliases:
```rust
const KEY_NAME: &'static str = "PHONON_KPOINT_MP_GRID";
const KEY_ALIASES: &'static [&'static str] = &["PHONON_KPOINTS_MP_GRID"];
```

Structurally identical to `KpointsMpGrid`. **No `bon::Builder`**.

#### 5b: PhononKpointsMpSpacing (key-value)

```rust
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PhononKpointsMpSpacing {
    pub value: f64,
    pub unit: Option<InvLengthUnit>,
}
// Impls: FromCellValue, FromKeyValue, ToCell, ToCellValue
```

Key aliases:
```rust
const KEY_NAME: &'static str = "PHONON_KPOINT_MP_SPACING";
const KEY_ALIASES: &'static [&'static str] = &["PHONON_KPOINTS_MP_SPACING"];
```

Structurally identical to `KpointsMpSpacing`. **No `bon::Builder`**.

#### 5c: PhononKpointsMpOffset (key-value, newtype)

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhononKpointsMpOffset(pub [f64; 3]);
// Impls: FromCellValue, FromKeyValue, ToCell, ToCellValue
```

Key aliases:
```rust
const KEY_NAME: &'static str = "PHONON_KPOINT_MP_OFFSET";
const KEY_ALIASES: &'static [&'static str] = &["PHONON_KPOINTS_MP_OFFSET"];
```

Structurally identical to `KpointsMpOffset`. **No `bon::Builder`**.

#### 5d: PhononFineKpointPath (block type)

```rust
#[derive(Debug, Clone, PartialEq, bon::Builder)]
pub struct PhononFineKpointPath {
    #[builder(default)]
    pub points: Vec<PhononKpointPathEntry>,  // REUSE from phonon_kpoint_path.rs
}
// Impls: FromBlock, ToCell
```

**Entry type reuse:** `PhononKpointPathEntry { coord: [f64; 3] }` is imported from `phonon_kpoint_path.rs` via `use super::phonon_kpoint_path::PhononKpointPathEntry;`. Per plan resolution: a separate fine-entry type adds boilerplate without semantic benefit.

Block aliases:
```rust
const BLOCK_NAME: &'static str = "PHONON_FINE_KPOINT_PATH";
const BLOCK_ALIASES: &'static [&'static str] = &["PHONON_FINE_KPOINTS_PATH"];
```

#### 5e: PhononFineKpointPathSpacing (key-value)

```rust
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PhononFineKpointPathSpacing {
    pub value: f64,
    pub unit: Option<InvLengthUnit>,
}
```

Key aliases:
```rust
const KEY_NAME: &'static str = "PHONON_FINE_KPOINT_PATH_SPACING";
const KEY_ALIASES: &'static [&'static str] = &["PHONON_FINE_KPOINTS_PATH_SPACING"];
```

Structurally identical to `PhononKpointPathSpacing`. **No `bon::Builder`**.

#### 5f: PhononFineKpointsMpGrid (key-value, newtype)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhononFineKpointsMpGrid(pub [u32; 3]);
```

Key aliases:
```rust
const KEY_NAME: &'static str = "PHONON_FINE_KPOINT_MP_GRID";
const KEY_ALIASES: &'static [&'static str] = &["PHONON_FINE_KPOINTS_MP_GRID"];
```

**No `bon::Builder`**.

#### 5g: PhononFineKpointsMpSpacing (key-value)

```rust
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PhononFineKpointsMpSpacing {
    pub value: f64,
    pub unit: Option<InvLengthUnit>,
}
```

Key aliases:
```rust
const KEY_NAME: &'static str = "PHONON_FINE_KPOINT_MP_SPACING";
const KEY_ALIASES: &'static [&'static str] = &["PHONON_FINE_KPOINTS_MP_SPACING"];
```

**No `bon::Builder`**.

#### 5h: PhononFineKpointsMpOffset (key-value, newtype)

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhononFineKpointsMpOffset(pub [f64; 3]);
```

Key aliases:
```rust
const KEY_NAME: &'static str = "PHONON_FINE_KPOINT_MP_OFFSET";
const KEY_ALIASES: &'static [&'static str] = &["PHONON_FINE_KPOINTS_MP_OFFSET"];
```

**No `bon::Builder`**.

### 5i: Module registration

Add to `cell/phonon/mod.rs`:
```rust
mod phonon_kpoints_mp_grid;
mod phonon_kpoints_mp_spacing;
mod phonon_kpoints_mp_offset;
mod phonon_fine_kpoint_path;
mod phonon_fine_kpoint_path_spacing;
mod phonon_fine_kpoints_mp_grid;
mod phonon_fine_kpoints_mp_spacing;
mod phonon_fine_kpoints_mp_offset;

pub use phonon_kpoints_mp_grid::PhononKpointsMpGrid;
pub use phonon_kpoints_mp_spacing::PhononKpointsMpSpacing;
pub use phonon_kpoints_mp_offset::PhononKpointsMpOffset;
pub use phonon_fine_kpoint_path::PhononFineKpointPath;
pub use phonon_fine_kpoint_path_spacing::PhononFineKpointPathSpacing;
pub use phonon_fine_kpoints_mp_grid::PhononFineKpointsMpGrid;
pub use phonon_fine_kpoints_mp_spacing::PhononFineKpointsMpSpacing;
pub use phonon_fine_kpoints_mp_offset::PhononFineKpointsMpOffset;
```

### 5j: CellDocument wiring

```rust
// New struct fields (8 new fields):
pub phonon_kpoints_mp_grid: Option<PhononKpointsMpGrid>,
pub phonon_kpoints_mp_spacing: Option<PhononKpointsMpSpacing>,
pub phonon_kpoints_mp_offset: Option<PhononKpointsMpOffset>,
pub phonon_fine_kpoint_path: Option<PhononFineKpointPath>,
pub phonon_fine_kpoint_path_spacing: Option<PhononFineKpointPathSpacing>,
pub phonon_fine_kpoints_mp_grid: Option<PhononFineKpointsMpGrid>,
pub phonon_fine_kpoints_mp_spacing: Option<PhononFineKpointsMpSpacing>,
pub phonon_fine_kpoints_mp_offset: Option<PhononFineKpointsMpOffset>,
```

**Parsing additions** (in the phonon section, after `phonon_kpoint_path`, before `phonon_gamma_directions`):

```rust
let phonon_kpoints_mp_grid = PhononKpointsMpGrid::from_cells(cells)?;
let phonon_kpoints_mp_spacing = PhononKpointsMpSpacing::from_cells(cells)?;
let phonon_kpoints_mp_offset = PhononKpointsMpOffset::from_cells(cells)?;

let phonon_fine_kpoint_path = find_block_any(cells, &["PHONON_FINE_KPOINT_PATH", "PHONON_FINE_KPOINTS_PATH"])
    .ok()
    .map(|rows| PhononFineKpointPath::from_block_rows(rows))
    .transpose()?;

let phonon_fine_kpoint_path_spacing = PhononFineKpointPathSpacing::from_cells(cells)?;
let phonon_fine_kpoints_mp_grid = PhononFineKpointsMpGrid::from_cells(cells)?;
let phonon_fine_kpoints_mp_spacing = PhononFineKpointsMpSpacing::from_cells(cells)?;
let phonon_fine_kpoints_mp_offset = PhononFineKpointsMpOffset::from_cells(cells)?;
```

**Serialization additions** (in the phonon section, after `phonon_kpoint_path`, before `phonon_gamma_directions`):
```rust
if let Some(pmg) = &self.phonon_kpoints_mp_grid {
    cells.push(pmg.to_cell());
}
if let Some(pms) = &self.phonon_kpoints_mp_spacing {
    cells.push(pms.to_cell());
}
if let Some(pmo) = &self.phonon_kpoints_mp_offset {
    cells.push(pmo.to_cell());
}
if let Some(pfp) = &self.phonon_fine_kpoint_path {
    cells.push(pfp.to_cell());
}
if let Some(pfps) = &self.phonon_fine_kpoint_path_spacing {
    cells.push(pfps.to_cell());
}
if let Some(pfmg) = &self.phonon_fine_kpoints_mp_grid {
    cells.push(pfmg.to_cell());
}
if let Some(pfms) = &self.phonon_fine_kpoints_mp_spacing {
    cells.push(pfms.to_cell());
}
if let Some(pfmo) = &self.phonon_fine_kpoints_mp_offset {
    cells.push(pfmo.to_cell());
}
```

**Import additions:** Add all 8 types to the `phonon` import block in `cell_document.rs`.

### Design decisions

- **PhononFineKpointPath reuses `PhononKpointPathEntry`**: Per plan resolution. The entry type is imported from the sibling module. Both fine and non-fine path entries are structurally identical `([f64; 3])`.
- **bon::Builder only on block types**: `PhononFineKpointPath` gets `#[derive(bon::Builder)]` because it has a `Vec` collection field. All other new types are either tuple newtypes or 2-field spacing structs and follow the existing no-builder convention.
- **No BS_ aliases**: Phonon types are independent of BS_ prefix. Only KPOINTS plural aliases are needed.
- **All key-value MP types**: Use `FromKeyValue::from_cells(cells)?` for consistent alias-aware parsing.

---

## Crate Boundary Decisions

| Crate | Changes |
|-------|---------|
| `castep_cell_fmt` | **None.** No new traits, no new query functions needed. |
| `castep_cell_io` | All changes: 14 new keyword type files, 3 existing file fixes, 2 `mod.rs` updates, extensive `cell_document.rs` changes. |

All new types live in `castep_cell_io/src/cell/` following the existing submodule structure. The `FromKeyValue::from_cells` method (provided by `castep_cell_fmt`) already supports the alias resolution pattern we need -- no trait changes required.

---

## Pattern Requirements Summary

### Trait impl triads

Every keyword type (except `SymmetryGenerate`) follows this exact pattern:

| Type category | Traits to implement |
|---------------|-------------------|
| Block with entries | `FromBlock` + `ToCell` (on container) + `FromCellValue` + `ToCellValue` (on entries) + `bon::Builder` |
| Key-value simple | `FromCellValue` + `FromKeyValue` + `ToCell` + `ToCellValue` (+ `bon::Builder` if multi-field) |
| Key-value newtype | `FromCellValue` + `FromKeyValue` + `ToCell` + `ToCellValue` (no builder) |
| Flag (SymmetryGenerate) | `ToCell` only (parsed via `has_flag`, not `FromKeyValue`) |

### Alias rules (canonical)

1. All k-point keyword types: include KPOINTS plural alias (e.g., `KPOINT_MP_GRID` -> alias `KPOINTS_MP_GRID`)
2. Spectral types with BS_ equivalent (path, list, path_spacing): include BS_ backward compat aliases
3. Net-new spectral types (MpGrid, MpSpacing, MpOffset): KPOINTS plural only, no BS_ alias
4. Phonon types: KPOINTS plural only (PHONON_ prefix, no BS_ involvement)

### Builder rules

| Pattern | bon::Builder? | Example |
|---------|--------------|---------|
| Block type with `Vec` field | **Yes** | `BsKpointPath`, `SpectralKpointPath`, `PhononFineKpointPath` |
| Block type with entry fields | **Yes** | `CellConstraints` |
| Entry type (`[f64; 3]`) | **Yes** (if used in a block) | `BsKpointPathEntry`, `SpectralKpointPathEntry` |
| Tuple newtype (`(pub [T; 3])`) | **No** | `KpointsMpGrid`, `PhononKpointsMpGrid`, all *Offset types |
| 2-field spacing struct | **No** | `KpointsMpSpacing`, all *Spacing types |
| Unit struct / flag | **No** | `SymmetryGenerate` |

### Existing type pattern references

When implementing a new type, use the exact existing type as a template:

| New type | Copy pattern from |
|----------|-------------------|
| `SpectralKpointPath` + `SpectralKpointPathEntry` | `BsKpointPath` + `BsKpointPathEntry` |
| `SpectralKpointsList` | `BSKpointList` |
| `SpectralKpointPathSpacing` | `BsKpointPathSpacing` |
| `SpectralKpointsMpGrid` | `KpointsMpGrid` |
| `SpectralKpointsMpSpacing` | `KpointsMpSpacing` |
| `SpectralKpointsMpOffset` | `KpointsMpOffset` |
| `KpointsMpOffset` | `KpointsMpGrid` (different element type: f64 vs u32) |
| `PhononKpointsMpGrid` | `KpointsMpGrid` |
| `PhononKpointsMpSpacing` | `KpointsMpSpacing` |
| `PhononKpointsMpOffset` | `KpointsMpOffset` |
| `PhononFineKpointPath` | `PhononKpointPath` (with `bon::Builder` added) |
| `PhononFineKpointPathSpacing` | `PhononKpointPathSpacing` |
| `PhononFineKpointsMpGrid` | `KpointsMpGrid` |
| `PhononFineKpointsMpSpacing` | `KpointsMpSpacing` |
| `PhononFineKpointsMpOffset` | `KpointsMpOffset` |

---

## Known Pitfalls and Constraints

### P0: Spectral + BS_ alias collision

**Issue:** `SpectralKpointPath` has `BLOCK_ALIASES` including `"BS_KPOINT_PATH"` and `"BS_KPOINTS_PATH"`. `SpectralKpointsList` aliases include `"BS_KPOINT_LIST"` and `"BS_KPOINTS_LIST"`. When parsing a cell file that uses BS_ prefix blocks, both the spectral field AND the BS_ field will be populated from the same block (since `find_block_any` is a lookup, not a consume).

**Impact:** Files using BS_ prefix blocks will produce `CellDocument` instances with both `spectral_kpoint_path` and `bs_kpoint_path` populated. Round-trip serialization would emit both blocks (SPECTRAL_ and BS_ prefixes), producing a different file.

**Mitigation options:**
1. **Remove BS_ aliases from spectral types** -- breaks backward compatibility with legacy files that use BS_ prefix for spectral calculations. Rejected by plan.
2. **Parse spectral types first, skip BS_ parsing if spectral field was populated** -- requires modifying the BS_ parsing to conditionally skip. Adds coupling.
3. **Accept dual-population for now** -- document the behavior, note that validation (deferred) will handle this. This is the least invasive option and matches the plan's deferral of validation.

**Recommendation:** Option 3 for now. File a deferred item to add mutual exclusion between spectral and BS_ fields in the validation phase.

### P1: CellDocument field count

After all 5 goals, `CellDocument` will have 50 fields (28 existing + 22 new). This is large but acceptable for a flat document type. The sub-group restructure (deferred) will reduce this. Consider adding a `// TODO: Group into KpointsParams, SpectralParams, etc.` comment near the struct definition.

### P2: Missing `pub use` chain

After adding any new type file, verify the full chain:
1. `mod xxx;` in parent `mod.rs`
2. `pub use xxx::TypeName;` in parent `mod.rs`
3. Import in `cell_document.rs`
4. Field on `CellDocument` struct
5. Parsing in `from_cell_file`
6. Serialization in `to_cell_file`
7. Field in `Ok(CellDocument { ... })` constructor

Missing any step causes a compile error (unused import, missing field) or silent failure (type not exported, field never populated).

### P3: Alias omission

Forgetting `KEY_ALIASES` or `BLOCK_ALIASES` on a k-point type causes silent parse failures for plural-form inputs. This is the `PhononKpointPathSpacing` bug we're fixing in Goal 1. Every new k-point type MUST have aliases.

### P4: Test gaps

`SymmetryTol` has zero tests. While adding `SymmetryTol` wiring, consider adding tests for it as part of Goal 1 (or file a deferred item). `SymmetryGenerate` will need tests -- do not skip them.

### P5: Test structure for new types

Each new type file must include a `#[cfg(test)] mod tests` block. Minimum test cases per type:

| Type category | Minimum tests |
|---------------|--------------|
| Block type | block_name, empty_block, multiple_entries, to_cell |
| Entry type | from_cell_value (valid, insufficient, too_many), to_cell_value |
| Key-value newtype | from_cell_value (valid, insufficient, too_many, non_array), to_cell_value, to_cell, key_name, key_aliases, round_trip |
| Key-value spacing | scalar, with_unit (ang), with_unit (bohr), empty_array, to_cell_scalar, to_cell_with_unit, round_trip |
| Flag type | presence detection + to_cell test |

### P6: Lattice enum non-exhaustive

After adding `Lattice::Abc`, any external code matching on `Lattice` will break. This is intentional (the plan says mandatory builders with breaking changes are acceptable). But note it's a breaking change to the public API.

### P7: Bon builder on CellDocument

The `CellDocument` builder auto-generates setters for all `Option<T>` fields. No manual builder code needed for new fields. However, with 50 fields, the builder may become unwieldy. Use `bon`'s `#[builder(default)]` where sensible (all optional fields default to `None` already, which is the bon default for `Option<T>`).

---

## Suggested Task Grouping

### Group G1 (do first): Goal 1 -- Fix gaps and wire orphans
- **Rationale:** Establishes the `FromKeyValue::from_cells` wiring pattern that all subsequent goals follow. Fixes the `CellConstraints` export gap (blocker for Goal 1c). Adds the first use of `has_flag`-style parsing (though SymmetryGenerate in Goal 3 needs this too).
- **Risk:** Low. All types already exist and have tests; only wiring is new.
- **Files:** `constraints/mod.rs`, `phonon/phonon_kpoint_path_spacing.rs`, `cell_document.rs`
- **Task count:** 3 atomic changes (1a, 1b, 1c) in 3 files

### Group G2 (do second): Goal 2 -- Lattice::Abc
- **Rationale:** Standalone change touching the same `cell_document.rs`. Should be done after G1 to avoid merge conflicts.
- **Risk:** Medium. Changes `Lattice` enum (breaking change), adds error path for dual-lattice detection.
- **Files:** `cell_document.rs` only
- **Task count:** 1 atomic change (2a-2e are all in one file)

### Group G3 (do third): Goal 3 -- KpointsMpOffset + SymmetryGenerate
- **Rationale:** Smallest new-type goal. Establishes the "new file creation + module registration + CellDocument wiring" workflow that G4 and G5 follow at scale.
- **Risk:** Low. Two simple types.
- **Files:** `kpoints_mp_offset.rs` (new), `symmetry_generate.rs` (new), `bz_sampling_kpoints/mod.rs`, `symmetry/mod.rs`, `cell_document.rs`
- **Task count:** 2 new files + 3 file edits

### Group G4 (do fourth): Goal 4 -- Spectral k-point types
- **Rationale:** 6 new types in one module. Follows the G3 workflow pattern. Adds significant new imports to `cell_document.rs`.
- **Risk:** Medium. Alias collision pitfall (P0). Largest single code addition.
- **Files:** 6 new files in `bz_sampling_kpoints/`, `bz_sampling_kpoints/mod.rs`, `cell_document.rs`
- **Task count:** 6 new files + 2 file edits

### Group G5 (do fifth): Goal 5 -- Phonon MP and fine types
- **Rationale:** 8 new types in one module. Follows the G3 workflow pattern. Largest number of files but structurally simpler (no alias collision issues).
- **Risk:** Low-Medium. Many files but each follows a well-established pattern.
- **Files:** 8 new files in `phonon/`, `phonon/mod.rs`, `cell_document.rs`
- **Task count:** 8 new files + 2 file edits

### Parallelization note

G4 and G5 touch different modules (`bz_sampling_kpoints/` vs `phonon/`) but both edit `cell_document.rs` for wiring. If done in parallel, the second group will need to rebase onto the first's CellDocument changes. Sequential execution is safer and the task groups are small enough that the overhead is low.

---

## Consolidated CellDocument Addition Map

For reference, here is the complete list of fields to add to `CellDocument` across all goals, in the order they should appear in the struct (grouped by section):

### K-point sampling section (after `magres_kpoints_list`, before `symmetry_ops`)
```
kpoints_mp_grid: Option<KpointsMpGrid>,           // G1
kpoints_mp_spacing: Option<KpointsMpSpacing>,      // G1
kpoints_mp_offset: Option<KpointsMpOffset>,        // G3
bs_kpoint_path_spacing: Option<BsKpointPathSpacing>, // G1
spectral_kpoint_path: Option<SpectralKpointPath>,  // G4
spectral_kpoints_list: Option<SpectralKpointsList>, // G4
spectral_kpoint_path_spacing: Option<SpectralKpointPathSpacing>, // G4
spectral_kpoints_mp_grid: Option<SpectralKpointsMpGrid>, // G4
spectral_kpoints_mp_spacing: Option<SpectralKpointsMpSpacing>, // G4
spectral_kpoints_mp_offset: Option<SpectralKpointsMpOffset>, // G4
```

### Constraints section (after `fix_all_cell`, before `external_efield`)
```
fix_vol: Option<FixVOL>,                           // G1
cell_constraints: Option<CellConstraints>,         // G1
```

### Symmetry section (after `symmetry_ops`, before `external_efield` in `from_cell_file`; before `external_efield` in `to_cell_file`)
```
symmetry_tol: Option<SymmetryTol>,                 // G1
symmetry_generate: Option<SymmetryGenerate>,       // G3
```

### Phonon section (after `phonon_kpoint_path`, before `phonon_gamma_directions`)
```
phonon_kpoints_mp_grid: Option<PhononKpointsMpGrid>,       // G5
phonon_kpoints_mp_spacing: Option<PhononKpointsMpSpacing>, // G5
phonon_kpoints_mp_offset: Option<PhononKpointsMpOffset>,   // G5
phonon_fine_kpoint_path: Option<PhononFineKpointPath>,     // G5
phonon_fine_kpoint_path_spacing: Option<PhononFineKpointPathSpacing>, // G5
phonon_fine_kpoints_mp_grid: Option<PhononFineKpointsMpGrid>,         // G5
phonon_fine_kpoints_mp_spacing: Option<PhononFineKpointsMpSpacing>,   // G5
phonon_fine_kpoints_mp_offset: Option<PhononFineKpointsMpOffset>,     // G5
```

### Total: 22 new fields across 5 goals

---

## Deferred Items Carried Forward

The following items are explicitly out of scope and should be tracked for future phases:

1. **10 sub-group param structs** (KpointsParams, SpectralParams, SymmetryParams, ConstraintsParams, ExternalFieldParams, SpeciesParams, PhononParams, PhononFineParams, OpticsMagresParams, DynamicsParams)
2. **CellDocument restructure** to use sub-group fields instead of flat optional fields
3. **Validation logic** (mutual exclusion between spectral/BS_ fields, cell_constraints superseding fix_all_cell, etc.)
4. **Unit tests for validation** (no validation code exists yet)
5. **Consolidation of structurally identical BS_/SPECTRAL_ types**
6. **Round-trip test fixtures** for the ignored integration test in `cell_document.rs`

---

## Non-Obvious Implementation Notes

### FixVOL KEY_ALIASES false positive

The codebase-state.md flagged `FixVOL` as lacking `KEY_ALIASES`. This is not a bug -- `FIX_VOL` has no plural variant (`FIX_VOLS` is not valid CASTEP). The `KEY_ALIASES` default (`&[]`) is correct. Same for `SymmetryTol`.

### PhononKpointPath lacks bon::Builder

The existing `PhononKpointPath` (in `phonon_kpoint_path.rs`) does NOT have `#[derive(bon::Builder)]`, unlike `BsKpointPath`. This is an existing inconsistency. For `PhononFineKpointPath` (Goal 5d), we ADD `bon::Builder` (consistent with the plan's mandate). The existing `PhononKpointPath` can be updated in a future cleanup pass.

### ToCell for block types uses primary BLOCK_NAME

When serializing, all `ToCell` impls use the primary `BLOCK_NAME`/`KEY_NAME`, not an alias. This is the existing convention and should be maintained. Aliases are for deserialization only.

### Newtype struct field access

Tuple newtypes like `KpointsMpOffset(pub [f64; 3])` have a public field. External users access it via `offset.0`. This is the existing convention (see `KpointsMpGrid.0`). No accessor methods needed.

# Implementation Plan: Missing CASTEP Keywords

## Context

The castep-cell-io project provides a parsing and serialization framework for CASTEP `.cell` and `.param` file formats. Currently, 24 cell keywords and 48 param keywords are unimplemented. The user has classified these keywords into 5 patterns in `CLASSIFICATION_TABLE.md`:

- **Pattern 1**: Simple Keyword Enum (limited string options)
- **Pattern 2**: Simple Scalar Type (single value)
- **Pattern 3**: Composite Key-Value Type (value + unit)
- **Pattern 4**: Block Type (multi-row data)
- **Pattern 5**: Unit Type (measurement units)

All keywords have detailed documentation in `KEYWORD_DOCS/` organized by functional group. The goal is to implement all missing keywords following established patterns in the codebase.

## Implementation Strategy

### Pattern 3: Composite Key-Value Type (Value + Unit)

**Keywords to implement:**

- Cell: `BS_KPOINT_PATH_SPACING`, `KPOINTS_MP_SPACING`, `PHONON_KPOINT_PATH_SPACING`
- Param: `IMPLICIT_SOLVENT_SURFACE_TENSION`

**Reference implementation:** `castep_cell_data/src/param/basis_set/cutoff_energy.rs`

**Approach:**

1. Create struct with `value: f64` and `unit: Option<UnitType>` fields
2. Implement `FromCellValue` to parse both scalar floats and arrays (value + unit)
3. Implement `FromKeyValue` with appropriate `KEY_NAME`
4. Implement `ToCell` and `ToCellValue` for serialization
5. Add unit tests

**Unit types needed:**

- `InvLengthUnit` (already exists in `castep_cell_data/src/units/inv_length_units.rs`) for spacing keywords
- `ForceConstantUnit` (already exists in `castep_cell_data/src/units/force_constant_units.rs`) for surface tension

**Documentation links:**

- `KEYWORD_DOCS/Brillouin_zone_sampling_k-points/BS_KPOINT_PATH_SPACING.md`
- `KEYWORD_DOCS/Brillouin_zone_sampling_k-points/KPOINTS_MP_SPACING.md`
- `KEYWORD_DOCS/q-vectors_for_phonon_calculations/PHONON_KPOINT_PATH_SPACING.md`
- `KEYWORD_DOCS/Solvation_energy_parameters/IMPLICIT_SOLVENT_SURFACE_TENSION.md`

**File locations:**

- Cell keywords: `castep_cell_data/src/cell/bz_sampling_kpoints/` (for k-point spacing)
- Cell keywords: `castep_cell_data/src/cell/phonon/` (create new module for phonon keywords)
- Param keywords: `castep_cell_data/src/param/solvation/` (create new module)

---

### Pattern 4: Block Type (Multi-Row Data)

**Keywords to implement:**

**Cell blocks:**

- `BS_KPOINT_PATH`, `BS_KPOINT_LIST` (Brillouin zone)
- `KPOINTS_MP_GRID` (special case: actually `[u32; 3]` despite being classified as Block)
- `MAGRES_KPOINTS_LIST`, `OPTICS_KPOINTS_LIST` (specialized k-point lists)
- `POSITIONS_ABS`, `POSITIONS_ABS_INTERMEDIATE`, `POSITIONS_ABS_PRODUCT`, `POSITIONS_FRAC_INTERMEDIATE`, `POSITIONS_FRAC_PRODUCT` (ionic positions)
- `IONIC_VELOCITIES` (ionic velocities)
- `SPECIES_Q`, `SEDC_CUSTOM_PARAMS` (species characteristics)
- `PHONON_GAMMA_DIRECTIONS`, `PHONON_KPOINT_PATH`, `PHONON_KPOINT_LIST`, `PHONON_FINE_KPOINT_LIST`, `PHONON_SUPERCELL_MATRIX`, `SUPERCELL_KPOINT_LIST_CASTEP` (phonon q-vectors)

**Param blocks:**

- `XC_DEFINITION` (exchange-correlation)

**Reference implementations:**

- Simple multi-row: `castep_cell_data/src/cell/positions/positions_frac.rs`
- Unit header + data: `castep_cell_data/src/cell/lattice_param.rs`
- Unit + entry collection: `castep_cell_data/src/cell/species/species_mass.rs`

**Approach by subtype:**

**4a. K-point path blocks** (`BS_KPOINT_PATH`, `PHONON_KPOINT_PATH`):

- Each row: 3 floats (fractional k-point coordinates)
- No unit header
- Similar to `POSITIONS_FRAC` but simpler (no species, no spin)
- Reference: `castep_cell_data/src/cell/positions/positions_frac.rs`

**4b. K-point list blocks** (`BS_KPOINT_LIST`, `MAGRES_KPOINTS_LIST`, `OPTICS_KPOINTS_LIST`, `PHONON_KPOINT_LIST`, `PHONON_FINE_KPOINT_LIST`, `SUPERCELL_KPOINT_LIST_CASTEP`):

- Each row: 3 floats (k-point) + 1 float (weight)
- No unit header
- Create entry struct with `coord: [f64; 3]` and `weight: f64`

**4c. KPOINTS_MP_GRID** (special case):

- Single line with 3 integers: `kpoints_mp_grid 3 4 6`
- Classified as Block but actually a simple key-value
- Implement as `FromKeyValue` with `[u32; 3]` value
- Parse using `value_as_u32()` for each element

**4d. Absolute position blocks** (`POSITIONS_ABS`, `POSITIONS_ABS_INTERMEDIATE`, `POSITIONS_ABS_PRODUCT`):

- Optional unit header (default: Å)
- Each row: species + 3 floats (absolute coordinates) + optional inline qualifiers
- **Inline qualifiers** (backward compatibility):
  - `SPIN` or `MAGMOM` (legacy): magnetic moment value
  - `MIXTURE`: atom mixing specification (index + weight)
- Similar to `POSITIONS_FRAC` but with unit header
- Reference: `castep_cell_data/src/cell/lattice_param.rs` (for unit header pattern)
- Reference: `castep_cell_data/src/cell/positions/positions_frac.rs` (for SPIN parsing)

**4e. Fractional position variants** (`POSITIONS_FRAC_INTERMEDIATE`, `POSITIONS_FRAC_PRODUCT`):

- Same structure as existing `POSITIONS_FRAC`
- Different block names for transition state search
- Can reuse `PositionFracEntry` type
- **Update existing `POSITIONS_FRAC`** to also support `MIXTURE` inline qualifier (currently only supports `SPIN`/`MAGMOM`)

**4f. IONIC_VELOCITIES**:

- Optional unit header (default: Å/ps)
- Each row: species + 3 floats (velocity components)
- Similar to position blocks but simpler (no spin)

**4g. SPECIES_Q**:

- Optional unit header (default: Barn, alternative: fm2)
- Each row: species + 1 float (quadrupole moment)
- Reference: `castep_cell_data/src/cell/species/species_mass.rs`
- Need to add `QuadrupoleMomentUnit` enum (Barn, Fm2)

**4h. SEDC_CUSTOM_PARAMS**:

- Each row: species + multiple float parameters
- Need to check documentation for exact format

**4i. PHONON_GAMMA_DIRECTIONS**:

- Each row: 3 floats (direction vector)
- Simple multi-row block

**4j. PHONON_SUPERCELL_MATRIX**:

- 3 rows × 3 columns (3×3 integer matrix)
- Similar to `LATTICE_CART` but integers instead of floats

**4k. XC_DEFINITION**:

- Complex block with custom exchange-correlation functional definitions
- Need to check documentation for exact format
- **Added by user**: contains a `Vec` of keyword-value entries (`XC_FUNCTIONAL`+`f64`(weight)), which is mandatory. Other key-value entries in the documentations are optional.

**Documentation links:**

- All blocks have corresponding `.md` files in `KEYWORD_DOCS/` subdirectories

**File locations:**

- K-point blocks: `castep_cell_data/src/cell/bz_sampling_kpoints/`
- Position blocks: `castep_cell_data/src/cell/positions/`
- Velocity blocks: `castep_cell_data/src/cell/velocities/` (create new module)
- Species blocks: `castep_cell_data/src/cell/species/`
- Phonon blocks: `castep_cell_data/src/cell/phonon/` (create new module)
- XC blocks: `castep_cell_data/src/param/exchange_correlation/`

---

### Pattern 1: Simple Keyword Enum

**Keywords to implement:**

**Param keywords:**

- `SPECTRAL_TASK` (5 variants: BandStructure, DOS, Optics, CoreLoss, All)
- `TDDFT_POSITION_METHOD` (variants: MOLECULAR, ...)
- `SEDC_SCHEME` (5 variants: TS, OBS, G06, JCHS, MBD\*)
- `NLXC_K_SCRN_AVERAGING_SCHEME` / `K_SCRN_AVERAGING_SCHEME` (variants: AVE_DEN, ...)
- `OPTICS_XC_FUNCTIONAL` (inherits from XC_FUNCTIONAL variants)
- `PHONON_METHOD` (variants: LINEARRESPONSE, ...)
- `BOUNDARY_TYPE` (3 variants: PERIODIC, OPEN, ZERO)
- `DIELEC_EMB_FUNC_METHOD` (2 variants: FG, UNIFORM)
- `TSSEARCH_LSTQST_PROTOCOL` (variants: CompleteLSTQST, ...)
- `TSSEARCH_METHOD` (variants: LSTQST, ...)
- `CHARGE_UNIT` (variants: e, ...)
- `LENGTH_UNIT` (variants: ang, bohr, m, cm, nm, ...): partially implemented,
  `ToCell` is not implemented.

**Reference implementation:** `castep_cell_data/src/param/general/task.rs`

**Approach:**

1. Create enum with variants from documentation
2. Implement `FromCellValue` with case-insensitive string matching
3. Implement `FromKeyValue` with appropriate `KEY_NAME`
4. Implement `ToCell` and `ToCellValue` for serialization
5. Add `#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]` attributes
6. Add `#[default]` attribute to default variant if applicable
7. Add unit tests

**Documentation links:**

- All enums have corresponding `.md` files in `KEYWORD_DOCS/` subdirectories

**File locations:**

- Electronic excitations: `castep_cell_data/src/param/electronic_excitations/` (create new module)
- Electronic parameters: `castep_cell_data/src/param/electronic/`
- Exchange-correlation: `castep_cell_data/src/param/exchange_correlation/`
- Optics: `castep_cell_data/src/param/optics/` (create new module)
- Phonon: `castep_cell_data/src/param/phonon/` (create new module)
- Solvation: `castep_cell_data/src/param/solvation/` (create new module)
- Transition state: `castep_cell_data/src/param/transition_state/` (create new module)
- General: `castep_cell_data/src/param/general/`
- Units: `castep_cell_data/src/units/` (for LENGTH_UNIT, CHARGE_UNIT)

---

### Pattern 2: Simple Scalar Type

**Keywords to implement:**

**Param keywords:**

- `TDDFT_NUM_STATES` (Integer)
- `TDDFT_SELECTED_STATE` (Integer)
- `PERC_EXTRA_BANDS` (Real)
- `SEDC_APPLY` (Logical)
- `SEDC_D_G06`, `SEDC_D_JCHS`, `SEDC_D_TS` (Real)
- `SEDC_LAMBDA_OBS`, `SEDC_N_OBS` (Real)
- `SEDC_S6_G06`, `SEDC_S6_JCHS` (Real)
- `SEDC_SR_JCHS`, `SEDC_SR_TS` (Real)
- `NLXC_EXCHANGE_REFLECT_KPTS` / `EXCHANGE_REFLECT_KPTS` (Logical)
- `NLXC_IMPOSE_TRS` / `IMPOSE_TRS` (Logical)
- `NLXC_PAGE_EX_POT` / `PAGE_EX_POT` (Integer)
- `NLXC_PPD_SIZE_X`, `NLXC_PPD_SIZE_Y`, `NLXC_PPD_SIZE_Z` / `PPD_SIZE_X`, `PPD_SIZE_Y`, `PPD_SIZE_Z` (Integer)
- `NLXC_PPD_INTEGRAL` / `PPD_INTEGRAL` (Logical)
- `NLXC_RE_EST_K_SCRN` / `RE_EST_K_SCRN` (Logical)
- `OPTICS_NBANDS`, `OPTICS_NEXTRA_BANDS` (Integer)
- `OPTICS_PERC_EXTRA_BANDS` (Real)
- `DIELEC_EMB_BULK_PERMITTIVITY` (Real)
- `IMPLICIT_SOLVENT_APOLAR_FACTOR` (Real)
- `IMPLICIT_SOLVENT_APOLAR_TERM` (Logical)
- `USE_SMEARED_IONS` (Logical)
- `TSSEARCH_CG_MAX_ITER`, `TSSEARCH_MAX_PATH_POINTS`, `TSSEARCH_QST_MAX_ITER` (Integer)
- `TSSEARCH_DISP_TOL`, `TSSEARCH_ENERGY_TOL`, `TSSEARCH_FORCE_TOL` (Real)

**Reference implementation:** `castep_cell_data/src/param/electronic/nbands.rs` (for integers), `castep_cell_data/src/param/band_structure/bs_perc_extra_bands.rs` (for floats)

**Approach:**

1. Create newtype struct wrapping primitive type (`f64`, `u32`, `i32`, `bool`)
2. Implement `FromCellValue` using appropriate query helper (`value_as_f64`, `value_as_u32`, `value_as_bool`)
3. Implement `FromKeyValue` with appropriate `KEY_NAME`
4. Implement `ToCell` and `ToCellValue` for serialization
5. Implement `Default` if applicable
6. Add unit tests

**Special cases:**

- **NLXC\_\* keywords**: Have both long form (NLXC\_\*) and short form aliases. Need to handle both key names.

**Documentation links:**

- All scalars have corresponding `.md` files in `KEYWORD_DOCS/` subdirectories

**File locations:**

- Electronic excitations: `castep_cell_data/src/param/electronic_excitations/` (create new module)
- Electronic parameters: `castep_cell_data/src/param/electronic/`
- Exchange-correlation: `castep_cell_data/src/param/exchange_correlation/`
- Optics: `castep_cell_data/src/param/optics/` (create new module)
- Solvation: `castep_cell_data/src/param/solvation/` (create new module)
- Transition state: `castep_cell_data/src/param/transition_state/` (create new module)

---

### Pattern 5: Unit Type

**New unit types needed:**

- `QuadrupoleMomentUnit` (Barn, Fm2) for `SPECIES_Q`
- Possibly others depending on block implementations

**Reference implementation:** `castep_cell_data/src/units/length_units.rs`

**Approach:**

1. Create enum with unit variants
2. Implement `FromCellValue` with case-insensitive string matching
3. Implement `ToCellValue` for serialization
4. Add `#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]`
5. Add unit tests

**File location:** `castep_cell_data/src/units/`

---

## Implementation Order

1. **Phase 1: Unit types** (if new ones needed)
   - `QuadrupoleMomentUnit`

2. **Phase 2: Pattern 1 (Keyword Enums)**
   - Start with simple enums with few variants
   - `SEDC_SCHEME`, `SPECTRAL_TASK`, `BOUNDARY_TYPE`, `PHONON_METHOD`

3. **Phase 3: Pattern 2 (Simple Scalars)**
   - Group by module (electronic, optics, solvation, transition state)
   - Implement all SEDC\_\* parameters together
   - Implement all NLXC\_\* parameters together
   - Implement all TDDFT\_\* parameters together
   - Implement all OPTICS\_\* parameters together
   - Implement all TSSEARCH\_\* parameters together

4. **Phase 4: Pattern 3 (Value + Unit)**
   - `BS_KPOINT_PATH_SPACING`, `KPOINTS_MP_SPACING`, `PHONON_KPOINT_PATH_SPACING`
   - `IMPLICIT_SOLVENT_SURFACE_TENSION`

5. **Phase 5: Pattern 4 (Blocks) - Simple**
   - `BS_KPOINT_PATH`, `PHONON_KPOINT_PATH` (simple 3-float rows)
   - `PHONON_GAMMA_DIRECTIONS` (simple 3-float rows)
   - `KPOINTS_MP_GRID` (special case: single line with 3 integers)

6. **Phase 6: Pattern 4 (Blocks) - K-point lists**
   - `BS_KPOINT_LIST`, `MAGRES_KPOINTS_LIST`, `OPTICS_KPOINTS_LIST`
   - `PHONON_KPOINT_LIST`, `PHONON_FINE_KPOINT_LIST`, `SUPERCELL_KPOINT_LIST_CASTEP`

7. **Phase 7: Pattern 4 (Blocks) - Position variants**
   - **Update existing `POSITIONS_FRAC`** to support `MIXTURE` inline qualifier
   - `POSITIONS_ABS`, `POSITIONS_ABS_INTERMEDIATE`, `POSITIONS_ABS_PRODUCT` (with unit header + inline qualifiers)
   - `POSITIONS_FRAC_INTERMEDIATE`, `POSITIONS_FRAC_PRODUCT` (reuse updated entry type)

8. **Phase 8: Pattern 4 (Blocks) - Species and other**
   - `IONIC_VELOCITIES`
   - `SPECIES_Q`, `SEDC_CUSTOM_PARAMS`
   - `PHONON_SUPERCELL_MATRIX`
   - `XC_DEFINITION` (most complex - save for last)

---

## Critical Files

**Existing files to reference:**

- `castep_cell_io/src/parse.rs` - Trait definitions (`FromCellValue`, `FromKeyValue`, `FromBlock`)
- `castep_cell_io/src/query.rs` - Query helpers (`value_as_f64`, `value_as_str`, `row_as_f64_n`, etc.)
- `castep_cell_data/src/param/general/task.rs` - Pattern 1 reference
- `castep_cell_data/src/param/electronic/nbands.rs` - Pattern 2 reference (integer)
- `castep_cell_data/src/param/basis_set/cutoff_energy.rs` - Pattern 3 reference
- `castep_cell_data/src/cell/positions/positions_frac.rs` - Pattern 4 reference (simple multi-row)
- `castep_cell_data/src/cell/lattice_param.rs` - Pattern 4 reference (unit header)
- `castep_cell_data/src/cell/species/species_mass.rs` - Pattern 4 reference (unit + entries)
- `castep_cell_data/src/units/length_units.rs` - Pattern 5 reference

**New modules to create:**

- `castep_cell_data/src/param/electronic_excitations/mod.rs`
- `castep_cell_data/src/param/optics/mod.rs`
- `castep_cell_data/src/param/phonon/mod.rs`
- `castep_cell_data/src/param/solvation/mod.rs`
- `castep_cell_data/src/param/transition_state/mod.rs`
- `castep_cell_data/src/cell/phonon/mod.rs`
- `castep_cell_data/src/cell/velocities/mod.rs`
- `castep_cell_data/src/units/quadrupole_moment_units.rs`

**Module registration:**

- Update `castep_cell_data/src/param/mod.rs` to include new param modules
- Update `castep_cell_data/src/cell/mod.rs` to include new cell modules
- Update `castep_cell_data/src/units/mod.rs` to include new unit types

---

## Verification

After implementation:

1. **Compilation**: `cargo build` must succeed
2. **Linting**: `cargo clippy` must pass with no warnings
3. **Unit tests**: Each keyword must have tests covering:
   - Parsing from valid input
   - Serialization round-trip (parse → serialize → parse)
   - Error handling for invalid input
   - Default values where applicable
4. **Integration tests**: Test parsing complete `.cell` and `.param` files containing the new keywords
5. **Documentation**: Each public type must have doc comments

**Test data location:** `Mg2SiO4_Cr_1.cell` at workspace root (existing test file)

**Test pattern:**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use castep_cell_io::parser::CellValue;

    #[test]
    fn test_parse_keyword() {
        let value = CellValue::Str("variant");
        let result = KeywordType::from_cell_value(&value).unwrap();
        assert_eq!(result, KeywordType::Variant);
    }

    #[test]
    fn test_round_trip() {
        let original = KeywordType::Variant;
        let cell_value = original.to_cell_value();
        let parsed = KeywordType::from_cell_value(&cell_value).unwrap();
        assert_eq!(original, parsed);
    }
}
```

---

## Notes

- All keyword names are case-insensitive in CASTEP format
- Use `to_ascii_lowercase()` for string matching in `FromCellValue`
- Float formatting uses `{v:20.16}` for serialization (see `castep_cell_io/src/format.rs`)
- Optional fields serialize as `CellValue::Null` which the formatter skips
- Block names in `FromBlock::BLOCK_NAME` should be uppercase
- Key names in `FromKeyValue::KEY_NAME` should be uppercase
- Follow functional programming style: prefer iterators over loops
- Use `derive_builder` for complex structs (though not yet used in existing code)
- Maintain ≥85% test coverage for new code

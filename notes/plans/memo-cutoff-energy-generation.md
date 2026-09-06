# Memo: CutOffEnergy Generation from Quality Level

## Manual procedure for deciding the cutoff energy

This is the procedure a user follows today. The implementation
reproduces it step by step.

Step 1 — Collect the pseudopotential files.
Read the `SPECIES_POT` block of the `.cell` file
(`CellDocument.species.species_pot`). Each entry names one element
and one pseudopotential filename. Every element in the calculation
must have a file.

Step 2 — Read the per-level cutoffs.
Each file header lists the `COARSE`, `MEDIUM`, and `FINE` cutoff
energies (see the table above). `ULTRAFINE` is not stored in any
file. Compute it from `FINE`:

```
ULTRAFINE = ceil(FINE * 1.1 / 10) * 10
```

Example: FINE = 280 → 280 × 1.1 = 308 → round up to 310.

Step 3 — Pick the maximum.
Take the maximum cutoff value across all elements at the one chosen
level. That maximum is the `CUT_OFF_ENERGY` value (in eV) to write
to the `.param` file.

This matches the documented CASTEP semantics:

- When `BASIS_PRECISION` is set, `CUT_OFF_ENERGY` equals the highest
  per-element cutoff at that level.
- When neither keyword is set, the default is the FINE level.
- `BASIS_PRECISION` and `CUT_OFF_ENERGY` must not appear together in
  one `.param` file.

See `KEYWORD_DOCS/Basis_set_parameters/CUT_OFF_ENERGY.md` and
`KEYWORD_DOCS/Basis_set_parameters/BASIS_PRECISION.md`.

## Status: Deferred

## Decision

Do not implement `CutOffEnergy` generation from a quality level yet.

## Reason

A quality level is a preset. It sets many parameters as a whole, not
only the cutoff energy. Implementing the cutoff alone gives a
misleading API. The full behavior lives in CASTEP's official GUI,
**Materials Studio**. Wait for a complete record of that behavior
before implementing.

## Scope: only CASTEP-shipped pseudopotential files

Support only the pseudopotential files that CASTEP ships. See
`~/Downloads/Potentials/`. No foreign formats: no VASP `.upf`, no
Quantum ESPRESSO `.psp` / `.psp8`, no ABINIT `.h`.

The shipped library holds four file types. Every type stores the
`COARSE`, `MEDIUM`, and `FINE` cutoff energies in the file header.
The `SPECIES_POT` block in `.cell` selects one shipped file per
element by filename.

| Extension | Files | Cutoff header lines | Units |
|---|---|---|---|
| `.otfg` | 108 | `60 COARSE` / `100 MEDIUM` / `140 FINE` | eV |
| `.recpot` | 1614 | `100 COARSE` / `150 MEDIUM` / `200 FINE` | eV |
| `.usp` | 196 | `COARSE = 3.675 ... (Ha)` | Hartree |
| `.uspcc` | 22 | `240 COARSE` / `280 MEDIUM` / `300 FINE` | eV |

A parser must handle two header shapes:

- `NNN LEVEL` lines: value in eV. Used by `.otfg`, `.recpot`,
  `.uspcc`.
- `LEVEL = N.NNN` lines: value in Hartree. Used by `.usp`.

Normalize all values to eV. Use 1 Ha = 27.211386 eV.

## User-specified ULTRAFINE rule

CASTEP ships no `ULTRAFINE` level. Define it as above.

Note: the shipped `BasisPrecision` enum already carries `PRECISE`
(1.2 × FINE) and `EXTREME` (1.6 × FINE). `ULTRAFINE` (1.1 × FINE)
is not one of the shipped levels. Decide whether to add it to
`BasisPrecision` or keep it as a derived value outside the enum.

## Existing components

| Component | File | Role |
|---|---|---|
| `SpeciesPot` | `src/cell/species/species_pot.rs` | Maps each element to its pseudopotential filename |
| `BasisPrecision` | `src/param/basis_set/basis_precision.rs` | COARSE / MEDIUM / FINE / PRECISE / EXTREME |
| `CutOffEnergy` | `src/param/basis_set/cutoff_energy.rs` | Stores the resolved `(value, unit)` |
| `BasisSetParams` | `src/param/basis_set_params.rs` | Aggregates all basis-set parameters |
| `PseudopotentialParams` | `src/param/pseudopotential_params.rs` | PSPOT_* keywords |

None of these generates a cutoff value. They only parse, store, and
emit the keywords.

## What is missing (implementation checklist)

1. **Pseudopotential header parser.** Read the four shipped formats
   (`.otfg`, `.recpot`, `.usp`, `.uspcc`) from the CASTEP potentials
   directory. Extract the `COARSE`, `MEDIUM`, and `FINE` cutoffs.
   Normalize Hartree values to eV. No such parser exists in the
   workspace today.
2. **ULTRAFINE derivation.** Compute it from `FINE` with the formula
   above.
3. **Max-across-species resolver.** Take the filenames from
   `SpeciesPot`, read each file, pick the cutoff at the chosen level,
   apply the ULTRAFINE rule when selected, and return the maximum.
4. **Preset expansion.** A quality level sets more parameters than
   `CUT_OFF_ENERGY`. Emit all of them together. This needs the
   Materials Studio record described below.
5. **Integration.** Wire the resolver into `CellDocument` /
   `ParamDocument` so a selected level populates `CUT_OFF_ENERGY`
   and the other preset parameters.

## What Must Be Documented First (Materials Studio)

Record the full Materials Studio workflow for selecting a quality
level. In particular, capture:

- Which parameters change together (cutoff, grid scale, mixing
  parameters, k-point sampling, etc.).
- The exact per-level cutoff values for each element's
  pseudopotential.
- The rounding or clamping rules applied.
- How the "max across all species" rule interacts with the other
  parameters in the preset.

## Blocking dependency

Wait for the Materials Studio quality-level behavior notes. Then
implement preset expansion (step 4). Steps 1–3 can be prototyped
independently. Keep the public API shape open until the final
Materials Studio spec lands, to avoid rework.

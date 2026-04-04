# CASTEP Keyword Classification Table

Instructions: Mark the correct pattern with `x` in the brackets.

Patterns:

- 1: Simple Keyword Enum (limited string options)
- 2: Simple Scalar Type (single value)
- 3: Composite Key-Value Type (value + unit)
- 4: Block Type (multi-row data)
- 5: Unit Type (measurement units)

## Brillouin zone sampling k-points

| Keyword                | Type  | Section | [ ] 1 | [ ] 2 | [ ] 3 | [ ] 4 | [ ] 5 |
| ---------------------- | ----- | ------- | ----- | ----- | ----- | ----- | ----- |
| BS_KPOINT_PATH_SPACING | Real  | Cell    | [ ]   | [ ]   | [x]   | [ ]   | [ ]   |
| KPOINTS_MP_SPACING     | Real  | Cell    | [ ]   | [ ]   | [x]   | [ ]   | [ ]   |
| BS_KPOINT_PATH         | Block | Cell    | [ ]   | [ ]   | [ ]   | [x]   | [ ]   |
| BS_KPOINT_LIST         | Block | Cell    | [ ]   | [ ]   | [ ]   | [x]   | [ ]   |
| KPOINTS_MP_GRID        | Block | Cell    | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| MAGRES_KPOINTS_LIST    | Block | Cell    | [ ]   | [ ]   | [ ]   | [x]   | [ ]   |
| OPTICS_KPOINTS_LIST    | Block | Cell    | [ ]   | [ ]   | [ ]   | [x]   | [ ]   |

`KPOINTS_MP_SPACING` is `f64`+`inverse_length_unit`
`KPOINTS_MP_GRID` is actually `[u32;3]`, although it is classified as `Block`.

## Electronic excitations parameters

| Keyword               | Type    | Section | [ ] 1 | [ ] 2 | [ ] 3 | [ ] 4 | [ ] 5 |
| --------------------- | ------- | ------- | ----- | ----- | ----- | ----- | ----- |
| SPECTRAL_TASK         | String  | Param   | [x]   | [ ]   | [ ]   | [ ]   | [ ]   |
| TDDFT_NUM_STATES      | Integer | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| TDDFT_POSITION_METHOD | String  | Param   | [x]   | [ ]   | [ ]   | [ ]   | [ ]   |
| TDDFT_SELECTED_STATE  | Integer | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |

## Electronic parameters

| Keyword          | Type    | Section | [ ] 1 | [ ] 2 | [ ] 3 | [ ] 4 | [ ] 5 |
| ---------------- | ------- | ------- | ----- | ----- | ----- | ----- | ----- |
| PERC_EXTRA_BANDS | Real    | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| SEDC_APPLY       | Logical | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| SEDC_D_G06       | Real    | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| SEDC_D_JCHS      | Real    | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| SEDC_D_TS        | Real    | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| SEDC_LAMBDA_OBS  | Real    | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| SEDC_N_OBS       | Real    | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| SEDC_S6_G06      | Real    | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| SEDC_S6_JCHS     | Real    | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| SEDC_SR_JCHS     | Real    | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| SEDC_SR_TS       | Real    | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| SEDC_SCHEME      | String  | Param   | [X]   | [ ]   | [ ]   | [ ]   | [ ]   |

## Exchange-correlation parameters

| Keyword                                               | Type    | Section | [ ] 1 | [ ] 2 | [ ] 3 | [ ] 4 | [ ] 5 |
| ----------------------------------------------------- | ------- | ------- | ----- | ----- | ----- | ----- | ----- |
| NLXC_EXCHANGE_REFLECT_KPTS, EXCHANGE_REFLECT_KPTS     | Logical | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| NLXC_IMPOSE_TRS, IMPOSE_TRS                           | Logical | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| NLXC_K_SCRN_AVERAGING_SCHEME, K_SCRN_AVERAGING_SCHEME | String  | Param   | [x]   | [ ]   | [ ]   | [ ]   | [ ]   |
| NLXC_PAGE_EX_POT, PAGE_EX_POT                         | Integer | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| NLXC_PPD_SIZE_X, PPD_SIZE_X                           | Integer | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| NLXC_PPD_SIZE_Y, PPD_SIZE_Y                           | Integer | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| NLXC_PPD_SIZE_Z, PPD_SIZE_Z                           | Integer | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| NLXC_PPD_INTEGRAL, PPD_INTEGRAL                       | Logical | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| NLXC_RE_EST_K_SCRN, RE_EST_K_SCRN                     | Logical | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| XC_DEFINITION                                         | Block   | Param   | [ ]   | [ ]   | [ ]   | [x]   | [ ]   |

## General parameters

| Keyword     | Type   | Section | [ ] 1 | [ ] 2 | [ ] 3 | [ ] 4 | [ ] 5 |
| ----------- | ------ | ------- | ----- | ----- | ----- | ----- | ----- |
| CHARGE_UNIT | String | Param   | [x]   | [ ]   | [ ]   | [ ]   | [ ]   |

## Ionic positions

| Keyword                     | Type      | Section | [ ] 1 | [ ] 2 | [ ] 3 | [ ] 4 | [ ] 5 |
| --------------------------- | --------- | ------- | ----- | ----- | ----- | ----- | ----- |
| MAGMOM                      | Qualifier | Cell    | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| MIXTURE                     | Qualifier | Cell    | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| POSITIONS_ABS               | Block     | Cell    | [ ]   | [ ]   | [ ]   | [x]   | [ ]   |
| POSITIONS_ABS_INTERMEDIATE  | Block     | Cell    | [ ]   | [ ]   | [ ]   | [x]   | [ ]   |
| POSITIONS_ABS_PRODUCT       | Block     | Cell    | [ ]   | [ ]   | [ ]   | [x]   | [ ]   |
| POSITIONS_FRAC_INTERMEDIATE | Block     | Cell    | [ ]   | [ ]   | [ ]   | [x]   | [ ]   |
| POSITIONS_FRAC_PRODUCT      | Block     | Cell    | [ ]   | [ ]   | [ ]   | [x]   | [ ]   |

`MIXTURE` has a format need to specially handle. The format of the MIXTURE entry
contains the index of the mixture atom and the weight associated with the given
component. Therefore it should be a tuple of `(u32, f64)`

## Ionic velocities

| Keyword          | Type  | Section | [ ] 1 | [ ] 2 | [ ] 3 | [ ] 4 | [ ] 5 |
| ---------------- | ----- | ------- | ----- | ----- | ----- | ----- | ----- |
| IONIC_VELOCITIES | Block | Cell    | [ ]   | [ ]   | [ ]   | [x]   | [ ]   |

## Optics

| Keyword                 | Type    | Section | [ ] 1 | [ ] 2 | [ ] 3 | [ ] 4 | [ ] 5 |
| ----------------------- | ------- | ------- | ----- | ----- | ----- | ----- | ----- |
| OPTICS_XC_FUNCTIONAL    | String  | Param   | [x]   | [ ]   | [ ]   | [ ]   | [ ]   |
| OPTICS_NBANDS           | Integer | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| OPTICS_NEXTRA_BANDS     | Integer | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| OPTICS_PERC_EXTRA_BANDS | Real    | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |

## Phonon parameters

| Keyword       | Type   | Section | [ ] 1 | [ ] 2 | [ ] 3 | [ ] 4 | [ ] 5 |
| ------------- | ------ | ------- | ----- | ----- | ----- | ----- | ----- |
| PHONON_METHOD | String | Param   | [x]   | [ ]   | [ ]   | [ ]   | [ ]   |

## Solvation energy parameters

| Keyword                          | Type    | Section | [ ] 1 | [ ] 2 | [ ] 3 | [ ] 4 | [ ] 5 |
| -------------------------------- | ------- | ------- | ----- | ----- | ----- | ----- | ----- |
| BOUNDARY_TYPE                    | String  | Param   | [x]   | [ ]   | [ ]   | [ ]   | [ ]   |
| DIELEC_EMB_BULK_PERMITTIVITY     | Real    | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| DIELEC_EMB_FUNC_METHOD           | String  | Param   | [x]   | [ ]   | [ ]   | [ ]   | [ ]   |
| IMPLICIT_SOLVENT_APOLAR_FACTOR   | Real    | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| IMPLICIT_SOLVENT_APOLAR_TERM     | Logical | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| IMPLICIT_SOLVENT_SURFACE_TENSION | Real    | Param   | [ ]   | [ ]   | [x]   | [ ]   | [ ]   |
| USE_SMEARED_IONS                 | Logical | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |

`BOUNDARY_TYPE` has options: PERIODIC, OPEN or ZERO.
`DIELEC_EMB_FUNC_METHOD` has options: FG or UNIFORM. Default: FG if TASK is set
to AUTOSOLVATION.
`IMPLICIT_SOLVENT_SURFACE_TENSION` is `f64` + `force_constant_unit`

## Species characteristics

| Keyword            | Type  | Section | [ ] 1 | [ ] 2 | [ ] 3 | [ ] 4 | [ ] 5 |
| ------------------ | ----- | ------- | ----- | ----- | ----- | ----- | ----- |
| SPECIES_Q          | Block | Cell    | [ ]   | [ ]   | [ ]   | [x]   | [ ]   |
| SEDC_CUSTOM_PARAMS | Block | Cell    | [ ]   | [ ]   | [ ]   | [x]   | [ ]   |

## Transition state search

| Keyword                  | Type    | Section | [ ] 1 | [ ] 2 | [ ] 3 | [ ] 4 | [ ] 5 |
| ------------------------ | ------- | ------- | ----- | ----- | ----- | ----- | ----- |
| TSSEARCH_CG_MAX_ITER     | Integer | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| TSSEARCH_DISP_TOL        | Real    | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| TSSEARCH_ENERGY_TOL      | Real    | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| TSSEARCH_FORCE_TOL       | Real    | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| TSSEARCH_LSTQST_PROTOCOL | String  | Param   | [x]   | [ ]   | [ ]   | [ ]   | [ ]   |
| TSSEARCH_MAX_PATH_POINTS | Integer | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |
| TSSEARCH_METHOD          | String  | Param   | [x]   | [ ]   | [ ]   | [ ]   | [ ]   |
| TSSEARCH_QST_MAX_ITER    | Integer | Param   | [ ]   | [x]   | [ ]   | [ ]   | [ ]   |

## Units

| Keyword     | Type   | Section | [ ] 1 | [ ] 2 | [ ] 3 | [ ] 4 | [ ] 5 |
| ----------- | ------ | ------- | ----- | ----- | ----- | ----- | ----- |
| LENGTH_UNIT | String | Param   | [x]   | [ ]   | [ ]   | [ ]   | [ ]   |

## q-vectors for phonon calculations

| Keyword                      | Type  | Section | [ ] 1 | [ ] 2 | [ ] 3 | [ ] 4 | [ ] 5 |
| ---------------------------- | ----- | ------- | ----- | ----- | ----- | ----- | ----- |
| PHONON_KPOINT_PATH_SPACING   | Real  | Cell    | [ ]   | [ ]   | [x]   | [ ]   | [ ]   |
| PHONON_GAMMA_DIRECTIONS      | Block | Cell    | [ ]   | [ ]   | [ ]   | [x]   | [ ]   |
| PHONON_KPOINT_PATH           | Block | Cell    | [ ]   | [ ]   | [ ]   | [x]   | [ ]   |
| PHONON_KPOINT_LIST           | Block | Cell    | [ ]   | [ ]   | [ ]   | [x]   | [ ]   |
| PHONON_FINE_KPOINT_LIST      | Block | Cell    | [ ]   | [ ]   | [ ]   | [x]   | [ ]   |
| PHONON_SUPERCELL_MATRIX      | Block | Cell    | [ ]   | [ ]   | [ ]   | [x]   | [ ]   |
| SUPERCELL_KPOINT_LIST_CASTEP | Block | Cell    | [ ]   | [ ]   | [ ]   | [x]   | [ ]   |

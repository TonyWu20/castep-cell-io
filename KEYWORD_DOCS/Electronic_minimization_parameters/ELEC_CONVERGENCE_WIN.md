# ELEC_CONVERGENCE_WIN

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_elec_convergence_win_castep.htm

**Group:** Electronic minimization parameters

---

# ELEC\_CONVERGENCE\_WIN (.param)

## Keyword type

Integer

## Description

This keyword determines the size of the convergence window during a electronic minimization run.

The total energy or eigenvalue must lie within [ELEC\_ENERGY\_TOL](k_elec_energy_tol_castep.htm) or
[ELEC\_EIGENVALUE\_TOL](k_elec_eigenvalue_tol_castep.htm) respectively, for the last ELEC\_CONVERGENCE\_WIN iterations for the
convergence criteria to be met.

The value of ELEC\_CONVERGENCE\_WIN must be greater than or equal to 2.

## Default

3

## Example

```

ELEC_CONVERGENCE_WIN : 4
```

###### See Also:

[ELEC\_ENERGY\_TOL](k_elec_energy_tol_castep.htm)
  
[ELEC\_EIGENVALUE\_TOL](k_elec_eigenvalue_tol_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
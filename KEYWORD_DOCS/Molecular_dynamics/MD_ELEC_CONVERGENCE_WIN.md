# MD_ELEC_CONVERGENCE_WIN

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_md_elec_convergence_win_castep.htm

**Group:** Molecular dynamics

---

# MD\_ELEC\_CONVERGENCE\_WIN (.param)

## Keyword type

Integer

## Description

This keyword determines the size of the convergence window for electronic minimization during a molecular dynamics run.

The total energy or eigenvalue must lie within [MD\_ELEC\_ENERGY\_TOL](k_md_elec_energy_tol_castep.htm) or
[MD\_ELEC\_EIGENVALUE\_TOL](k_md_elec_eigenvalue_tol_castep.htm) respectively, for the last MD\_ELEC\_CONVERGENCE\_WIN iterations for the
convergence criteria to be met.

The value of MD\_ELEC\_CONVERGENCE\_WIN must be greater than or equal to 2.

## Default

By default this has the same value as [ELEC\_CONVERGENCE\_WIN](k_elec_convergence_win_castep.htm).

## Example

```

MD_ELEC_CONVERGENCE_WIN : 4
```

###### See Also:

[MD\_ELEC\_ENERGY\_TOL](k_md_elec_energy_tol_castep.htm)
  
[MD\_ELEC\_EIGENVALUE\_TOL](k_md_elec_eigenvalue_tol_castep.htm)
  
[ELEC\_CONVERGENCE\_WIN](k_elec_convergence_win_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

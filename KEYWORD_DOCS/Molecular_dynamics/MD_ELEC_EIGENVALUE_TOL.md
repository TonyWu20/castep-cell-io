# MD_ELEC_EIGENVALUE_TOL

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_md_elec_eigenvalue_tol_castep.htm

**Group:** Molecular dynamics

---

# MD\_ELEC\_EIGENVALUE\_TOL (.param)

## Keyword type

Real

## Description

This keyword controls the tolerance for accepting convergence of a single eigenvalue in a DM/DIIS minimization during a molecular dynamics
run.

The difference between maximum and minimum eigenvalues over
[MD\_ELEC\_CONVERGENCE\_WIN](k_md_elec_convergence_win_castep.htm) iterations must be less than this value.

## Default

By default this has the same value as [ELEC\_EIGENVALUE\_TOL](k_elec_eigenvalue_tol_castep.htm).

## Example

```

MD_ELEC_EIGENVALUE_TOL : 0.000007 eV
```

###### See Also:

[ELEC\_EIGENVALUE\_TOL](k_elec_eigenvalue_tol_castep.htm)
  
[MD\_ELEC\_CONVERGENCE\_WIN](k_md_elec_convergence_win_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
# BS_EIGENVALUE_TOL

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_bs_eigenvalue_tol_castep.htm

**Group:** Band structure parameters

---

# BS\_EIGENVALUE\_TOL (.param)

## Keyword type

Real

## Description

This keyword controls the tolerance for accepting convergence of a single eigenvalue or band during a band structure calculation.

The difference between maximum and minimum eigenvalue for every band over
[ELEC\_CONVERGENCE\_WIN](k_elec_convergence_win_castep.htm) iterations must be less than this value.

## Default

1×10-6 eV.

## Example

```

BS_EIGENVALUE_TOL = 1.0e-5 Ha
```

###### See Also:

[ELEC\_CONVERGENCE\_WIN](k_elec_convergence_win_castep.htm)
  
[BS\_MAX\_ITER](k_bs_max_iter_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

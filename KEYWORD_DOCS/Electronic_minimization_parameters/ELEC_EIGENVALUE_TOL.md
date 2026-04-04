# ELEC_EIGENVALUE_TOL

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_elec_eigenvalue_tol_castep.htm

**Group:** Electronic minimization parameters

---

# ELEC\_EIGENVALUE\_TOL (.param)

## Keyword type

Real

## Description

This keyword controls the tolerance for accepting convergence of a single eigenvalue during density mixing minimization.

The difference between maximum and minimum eigenvalues over
[ELEC\_CONVERGENCE\_WIN](k_elec_convergence_win_castep.htm) iterations must be less than this value.

## Default

The default value is the lower of 1x10-6 eV and
[ELEC\_ENERGY\_TOL](k_elec_energy_tol_castep.htm)\*NATOMS/[NBANDS](k_nbands_castep.htm), where NATOMS is the total number of
atoms in the unit cell.

## Example

```

ELEC_EIGENVALUE_TOL : 0.000007 eV
```

###### See Also:

[ELEC\_CONVERGENCE\_WIN](k_elec_convergence_win_castep.htm)
  
[ELEC\_ENERGY\_TOL](k_elec_energy_tol_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
# MD_ELEC_ENERGY_TOL

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_md_elec_energy_tol_castep.htm

**Group:** Molecular dynamics

---

# MD\_ELEC\_ENERGY\_TOL (.param)

## Keyword type

Real

## Description

This keyword controls the tolerance for accepting convergence of the total energy in an electronic minimization during a molecular dynamics
run.

The difference between maximum and minimum energies over
[MD\_ELEC\_CONVERGENCE\_WIN](k_md_elec_convergence_win_castep.htm) iterations must be less than this value.

## Default

By default this has the same value as [ELEC\_ENERGY\_TOL](k_elec_energy_tol_castep.htm).

## Example

```

MD_ELEC_ENERGY_TOL : 0.00007 eV
```

###### See Also:

[ELEC\_ENERGY\_TOL](k_elec_energy_tol_castep.htm)
  
[MD\_ELEC\_CONVERGENCE\_WIN](k_md_elec_convergence_win_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
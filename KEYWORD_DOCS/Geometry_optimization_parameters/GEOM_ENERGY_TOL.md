# GEOM_ENERGY_TOL

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_geom_energy_tol_castep.htm

**Group:** Geometry optimization parameters

---

# GEOM\_ENERGY\_TOL (.param)

## Keyword type

Real

## Description

This keyword controls the tolerance for accepting convergence of the free energy per atom during a geometry optimization.

The difference between maximum and minimum values of the free energy over
[GEOM\_CONVERGENCE\_WIN](k_geom_convergence_win_castep.htm) iterations must be less than this value.

## Default

2×10-5 eV per atom

## Example

```

GEOM_ENERGY_TOL : 0.00005 eV
```

###### See Also:

[GEOM\_CONVERGENCE\_WIN](k_geom_convergence_win_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
# EFIELD_ENERGY_TOL

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_efield_energy_tol_castep.htm

**Group:** Electric field parameters

---

# EFIELD\_ENERGY\_TOL (.param)

## Keyword type

Real

## Description

This keyword controls the tolerance for accepting convergence of the field constants during a linear response to electric field calculation.
This parameter has the units of volume.

The difference between maximum and minimum second order energies over
[EFIELD\_CONVERGENCE\_WIN](k_efield_convergence_win_castep.htm) iterations must be less than this value.

## Default

1×10-5 Å

## Example

```

EFIELD_ENERGY_TOL : 0.000002 ANG**3
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

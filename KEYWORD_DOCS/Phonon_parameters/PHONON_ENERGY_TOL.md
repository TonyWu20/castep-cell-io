# PHONON_ENERGY_TOL

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_phonon_energy_tol_castep.htm

**Group:** Phonon parameters

---

# PHONON\_ENERGY\_TOL (.param)

## Keyword type

Real

## Description

This keyword controls the tolerance for accepting convergence of the force constants during a phonon calculation.

The difference between maximum and minimum second order energies over
[PHONON\_CONVERGENCE\_WIN](k_phonon_convergence_win_castep.htm) iterations must be less than this value.

## Default

The default value is the same as [ELEC\_ENERGY\_TOL](k_elec_energy_tol_castep.htm).

## Example

```

PHONON_ENERGY_TOL : 0.00007 eV
```

###### See Also:

[ELEC\_ENERGY\_TOL](k_elec_energy_tol_castep.htm)
  
[PHONON\_CONVERGENCE\_WIN](k_phonon_convergence_win_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
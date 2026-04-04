# TASK

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_task_castep.htm

**Group:** General parameters

---

# TASK (.param)

## Keyword type

String

## Description

This keyword defines the type of calculation to perform. Available options are:

* SinglePoint - performs a single-point energy calculation.
* BandStructure - calculates band structure properties.
* GeometryOptimization - searches for a minimum energy structure.
* MolecularDynamics - performs a molecular dynamics calculation.
* Optics - calculates optical properties.
* Phonon - performs a linear response calculation to determine phonon frequencies and eigenvectors.
* Efield - performs an electric field linear response calculation to determine dielectric permittivity and
  polarizability.
* Phonon+Efield - performs a linear response calculation to determine phonon frequencies and eigenvectors,
  and an electric field linear response calculation to determine dielectric permittivity and polarizability.
* TransitionStateSearch - performs a transition-state search.
* MagRes - performs an NMR calculation.
* Elnes - performs core level spectroscopy calculation.
* ElectronicSpectroscopy - performs electronic spectroscopy calculation.
* Autosolvation - performs a free energy of solvation calculation.

NMR in CASTEP is part of the separately licensed module NMR CASTEP. NMR calculations can only be performed if
you have purchased this module.

## Default

SinglePoint

## Example

```

TASK : optics
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
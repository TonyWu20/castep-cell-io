# BASIS_PRECISION

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_basis_precision_castep.htm

**Group:** Basis set parameters

---

# BASIS\_PRECISION (.param)

## Keyword type

String

## Description

This keywords specifies the precision of the basis set by choosing the level of convergence of atomic energies with respect to the plane wave
cutoff energy for the pseudopotentials used in the calculation. Available options are:

* COARSE - convergence of about 1 eV/atom
* MEDIUM - convergence of about 0.3 eV/atom
* FINE - convergence of about 0.1 eV/atom
* PRECISE - 1.2 × FINE cutoff energy
* EXTREME - 1.6 × FINE cutoff energy, convergence of about 0.01 eV/atom

If the BASIS\_PRECISION is defined, the [CUT\_OFF\_ENERGY](k_cut_off_energy_castep.htm) will be equal to the highest of the cutoff
energies associated with the chosen level of accuracy, for the pseudopotentials used in the calculation.

It is not possible to specify both the BASIS\_PRECISION and the
[CUT\_OFF\_ENERGY](k_cut_off_energy_castep.htm) in a single file.

## Default

FINE

## Example

```

BASIS_PRECISION : MEDIUM
```

###### See Also:

[CUT\_OFF\_ENERGY](k_cut_off_energy_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

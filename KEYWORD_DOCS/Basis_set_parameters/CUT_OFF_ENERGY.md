# CUT_OFF_ENERGY

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_cut_off_energy_castep.htm

**Group:** Basis set parameters

---

# CUT\_OFF\_ENERGY (.param)

## Keyword type

Real

## Description

This keyword specifies the cutoff energy for the plane wave basis sets that will be used in the calculation.

If the [BASIS\_PRECISION](k_basis_precision_castep.htm) is defined, the cutoff energy will be equal to the highest of the cutoff
energies associated with the chosen level of accuracy, for the pseudopotentials used in the calculation.

If neither the [BASIS\_PRECISION](k_basis_precision_castep.htm) nor the CUT\_OFF\_ENERGY are defined, the default cutoff energy is
that associated with the FINE level of accuracy, for the pseudopotentials in the calculation.

It is not possible to specify both the [BASIS\_PRECISION](k_basis_precision_castep.htm)
and the CUT\_OFF\_ENERGY in a single file.

## Example

```

CUT_OFF_ENERGY = 125 eV
```

###### See Also:

[BASIS\_PRECISION](k_basis_precision_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

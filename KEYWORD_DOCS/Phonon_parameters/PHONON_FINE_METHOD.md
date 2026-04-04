# PHONON_FINE_METHOD

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_phonon_fine_method_castep.htm

**Group:** Phonon parameters

---

# PHONON\_FINE\_METHOD (.param)

## Keyword type

String

## Description

This keyword selects which calculation method to use for phonon calculation on a fine grid:

* NONE - No interpolation, direct calculations
* INTERPOLATE - Use Linear response (density functional perturbation theory) method
* SUPERCELL - Use Finite displacement method

## Default

NONE

## Example

```

PHONON_FINE_METHOD : SUPERCELL
```

###### See Also:

[PHONON\_SUPERCELL\_MATRIX](k_phonon_supercell_matrix_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
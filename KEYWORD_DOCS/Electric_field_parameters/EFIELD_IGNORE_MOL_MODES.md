# EFIELD_IGNORE_MOL_MODES

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_efield_ignore_mol_modes_castep.htm

**Group:** Electric field parameters

---

# EFIELD\_IGNORE\_MOL\_MODES (.param)

## Keyword type

String

## Description

This keyword specifies how many of the lowest lying modes to ignore when computing the ionic contribution to the permittivity and
polarizability. Available options are:

* Crystal - Ignore the three lowest lying modes
* Molecule - Ignore the six lowest lying modes
* Linear\_molecule - Ignore the five lowest lying modes

## Default

Crystal

## Example

```

EFIELD_IGNORE_MOL_MODES : Molecule
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
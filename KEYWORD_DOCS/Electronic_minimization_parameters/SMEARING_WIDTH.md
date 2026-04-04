# SMEARING_WIDTH

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_smearing_width_castep.htm

**Group:** Electronic minimization parameters

---

# SMEARING\_WIDTH (.param)

## Keyword type

Real

## Description

This keyword determines the width of the Fermi-surface smearing if the system is being treated as a metal.

This parameter is used only if [FIX\_OCCUPANCY](k_fix_occupancy_castep.htm) : FALSE.

## Default

0.2 eV

## Example

```

SMEARING_WIDTH : 0.1 eV
```

###### See Also:

[FIX\_OCCUPANCY](k_fix_occupancy_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
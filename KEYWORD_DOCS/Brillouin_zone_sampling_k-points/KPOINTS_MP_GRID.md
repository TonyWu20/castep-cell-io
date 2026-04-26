# KPOINTS_MP_GRID

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_kpoints_mp_grid_castep.htm

**Group:** Brillouin zone sampling k-points

---

# KPOINTS\_MP\_GRID or KPOINT\_MP\_GRID (.cell)

## Keyword type

Block

## Description

This data block contains Monkhorst-Pack grid parameters for generating the SCF k-points and their weights. According to the crystal symmetry, CASTEP symmetrizes the points after generation.

## Default

Generate the Monkhorst-Pack grid parameters from the KPOINTS\_MP\_SPACING value.

## Example

```
kpoints_mp_grid 3 4 6
```

###### See Also:

[KPOINTS\_MP\_SPACING](k_kpoints_mp_spacing_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

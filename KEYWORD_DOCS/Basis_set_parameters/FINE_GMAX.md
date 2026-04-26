# FINE_GMAX

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_fine_gmax_castep.htm

**Group:** Basis set parameters

---

# FINE\_GMAX (.param)

## Keyword type

Real

## Description

This keyword determines the maximum size of the g-vectors included in the fine grid.

The fine grid is set up such that all g-vectors with |g| less than or equal to FINE\_GMAX are included.

## Default

-1 a0-1 - this results in the fine and standard grids being identical

## Example

```

FINE_GMAX : 20 1/ang
```

There is a more convenient way of dealing with fine grid using dimensionless parameter
[FINE\_GRID\_SCALE](k_fine_grid_scale_castep.htm) which defines `FINE_GMAX` in terms of standard grid
`GMAX`.

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

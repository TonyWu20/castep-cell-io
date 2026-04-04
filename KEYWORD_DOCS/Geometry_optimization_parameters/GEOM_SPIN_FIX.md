# GEOM_SPIN_FIX

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_geom_spin_fix_castep.htm

**Group:** Geometry optimization parameters

---

# GEOM\_SPIN\_FIX (.param)

## Keyword type

Integer

## Description

This keyword determines the number of geometry optimization steps for which the total spin is fixed. If GEOM\_SPIN\_FIX is less than
0, the spin will be fixed to the value found at the end of the SCF calculation for the initial structure
for the duration of the calculation.

This parameter is only used if [FIX\_OCCUPANCY](k_fix_occupancy_castep.htm) =
FALSE. So for insulators the spin is fixed regardless of the value of GEOM\_SPIN\_FIX.

The default value for this parameter is 0, so spin is allowed to vary.

## Example

```

GEOM_SPIN_FIX : 5
```

###### See Also:

[FIX\_OCCUPANCY](k_fix_occupancy_castep.htm)
  
[SPIN\_FIX](k_spin_fix_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
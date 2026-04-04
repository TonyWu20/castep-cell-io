# EFIELD_MAX_CYCLES

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_efield_max_cycles_castep.htm

**Group:** Electric field parameters

---

# EFIELD\_MAX\_CYCLES (.param)

## Keyword type

Integer

## Description

This keyword controls the maximum number of iterations in a linear response to electric field calculation.

It may be necessary to increase this value if a low value of
[EFIELD\_MAX\_CG\_STEPS](k_efield_max_cg_steps_castep.htm) is used.

## Default

50

## Example

```

EFIELD_MAX_CYCLES : 100
```

###### See Also:

[EFIELD\_MAX\_CG\_STEPS](k_efield_max_cg_steps_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
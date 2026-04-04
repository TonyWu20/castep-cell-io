# EFIELD_MAX_CG_STEPS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_efield_max_cg_steps_castep.htm

**Group:** Electric field parameters

---

# EFIELD\_MAX\_CG\_STEPS (.param)

## Keyword type

Integer

## Description

This keyword controls the maximum number of conjugate gradient steps taken for each electronic band in the electronic minimizer before
resetting to the steepest descent direction, during a linear response to electric field calculation.

## Default

20

## Example

```

EFIELD_MAX_CG_STEPS : 30
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
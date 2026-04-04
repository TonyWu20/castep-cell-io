# ELECTRONIC_MINIMIZER

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_electronic_minimizer_castep.htm

**Group:** Electronic minimization parameters

---

# ELECTRONIC\_MINIMIZER (.param)

## Keyword type

String

## Description

This keyword controls the method used to minimize electronic states. Available options are:

* SD - minimizer takes up to 10 SD steps.
* CG - minimizer takes one SD step followed by up to 4 CG steps.

The default values for the number of steps can be overwritten using the [MAX\_SD\_STEPS](k_max_sd_steps_castep.htm) and
[MAX\_CG\_STEPS](k_max_cg_steps_castep.htm) keywords.

## Default

CG

## Example

```

ELECTRONIC_MINIMIZER : SD
```

###### See Also:

[MAX\_SD\_STEPS](k_max_sd_steps_castep.htm)
  
[MAX\_CG\_STEPS](k_max_cg_steps_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
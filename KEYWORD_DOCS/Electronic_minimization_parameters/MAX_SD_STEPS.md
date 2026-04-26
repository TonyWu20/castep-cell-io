# MAX_SD_STEPS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_max_sd_steps_castep.htm

**Group:** Electronic minimization parameters

---

# MAX\_SD\_STEPS (.param)

## Keyword type

Integer

## Description

This keyword determines the maximum number of steepest descent steps in an SCF cycle.

## Default

The default depends on the value of
[ELECTRONIC\_MINIMIZER](k_electronic_minimizer_castep.htm):

* SD then MAX\_SD\_STEPS : 10
* CG then MAX\_SD\_STEPS : 1
* RMM/DIIS then MAX\_SD\_STEPS : 1

If [ELECTRONIC\_MINIMIZER](k_electronic_minimizer_castep.htm) is not defined, the default is
1.

## Example

```

MAX_SD_STEPS : 5
```

###### See Also:

[ELECTRONIC\_MINIMIZER](k_electronic_minimizer_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

# MAX_CG_STEPS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_max_cg_steps_castep.htm

**Group:** Electronic minimization parameters

---

# MAX\_CG\_STEPS (.param)

## Keyword type

Integer

## Description

This keyword determines the maximum number of conjugate gradient steps in an SCF cycle.

## Default

The default depends on the value of
[ELECTRONIC\_MINIMIZER](k_electronic_minimizer_castep.htm):

* SD then MAX\_CG\_STEPS : 0
* CG then MAX\_CG\_STEPS : 4
* RMM/DIIS then MAX\_CG\_STEPS : 2

If [ELECTRONIC\_MINIMIZER](k_electronic_minimizer_castep.htm) is not defined, the default is
4.

## Example

```

MAX_CG_STEPS : 5
```

###### See Also:

[ELECTRONIC\_MINIMIZER](k_electronic_minimizer_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

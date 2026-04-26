# MD_OPT_DAMPED_DELTA_T

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_md_opt_damped_delta_t_castep.htm

**Group:** Molecular dynamics

---

# MD\_OPT\_DAMPED\_DELTA\_T (.param)

## Keyword type

Logical

## Description

This keyword controls whether or not the optimal time step will be calculated after each damped molecular dynamics step.

If MD\_OPT\_DAMPED\_DELTA\_T : TRUE, the optimal time step will be calculated after each damped molecular dynamics
step. Otherwise, a fixed time step will be used.

## Default

FALSE

## Example

```

MD_OPT_DAMPED_DELTA_T : TRUE
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

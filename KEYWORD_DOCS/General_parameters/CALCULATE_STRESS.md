# CALCULATE_STRESS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_calculate_stress_castep.htm

**Group:** General parameters

---

# CALCULATE\_STRESS (.param)

## Keyword type

Logical

## Description

This keyword controls whether or not a stress calculation will be performed.

If CALCULATE\_STRESS is set to TRUE, the stress tensor will be calculated, no matter which
[TASK](k_task_castep.htm) is selected. If it is set to FALSE, the stress tensor will only be calculated
if it is required, for example during a cell geometry optimization with cell relaxation.

## Default

FALSE

## Example

```

CALCULATE_STRESS : TRUE
```

###### See Also:

[TASK](k_task_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

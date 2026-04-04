# PHONON_MAX_CYCLES

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_phonon_max_cycles_castep.htm

**Group:** Phonon parameters

---

# PHONON\_MAX\_CYCLES (.param)

## Keyword type

Integer

## Description

This keyword controls the maximum number of iterations in a phonon calculation.

It might be necessary to increase this value if a low
[PHONON\_MAX\_CG\_STEPS](k_phonon_max_cg_steps_castep.htm) is used.

## Default

50

## Example

```

PHONON_MAX_CYCLES : 30
```

###### See Also:

[PHONON\_MAX\_CG\_STEPS](k_phonon_max_cg_steps_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
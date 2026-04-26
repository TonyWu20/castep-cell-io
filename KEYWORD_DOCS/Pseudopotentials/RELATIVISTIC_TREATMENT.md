# RELATIVISTIC_TREATMENT

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_relativistic_treatment.htm

**Group:** Pseudopotentials

---

# RELATIVISTIC\_TREATMENT (.param)

## Keyword type

String

## Description

Selects the method used to treat relativistic effects. This functionality is implemented for on-the-fly generated pseudopotentials, so this keyword has no effect when pseudopotentials are read from a file. Available options are:

* SCHROEDINGER - this option produces completely non-relativistic pseudopotentials.
* ZORA - scalar relativistic treatment, which is equivalent to the zeroth-order expansion of the Koelling-Harmon equation.
* KOELLING-HARMON - scalar relativistic treatment.
* DIRAC - fully relativistic treatment.

## Default

KOELLING-HARMON

## Example

```
RELATIVISTIC_TREATMENT : ZORA
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

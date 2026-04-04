# TDDFT_POSITION_METHOD

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_tddft_position_method.htm

**Group:** Electronic excitations parameters

---

# TDDFT\_POSITION\_METHOD (.param)

## Keyword type

String

## Description

Selects which method to use for the position operator in TDDFT (required for optical matrix element evaluation, and ultimately for the optical properties calculation):

* MOLECULAR - this option is appropriate for the "molecule in a box" geometry.
* CRYSTAL - this is the only option applicable for calculating optical matrix elements for true periodic solids.

## Default

MOLECULAR

## Example

```
TDDFT_POSITION_METHOD : CRYSTAL
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
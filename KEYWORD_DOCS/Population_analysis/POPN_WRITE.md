# POPN_WRITE

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_popn_write_castep.htm

**Group:** Population analysis

---

# POPN\_WRITE (.param)

## Keyword type

String

## Description

This keyword specifies the verbosity of reporting of population analysis results.

The available values are:

| Value | Meaning |
| --- | --- |
| NONE | No output |
| MINIMAL | Summary only |
| SUMMARY | Same as MINIMAL |
| ENHANCED | Summary and orbital-resolved PDOS components |
| VERBOSE | As ENHANCED and S and T matrices |

## Default

ENHANCED

## Example

```
POPN_WRITE : SUMMARY
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

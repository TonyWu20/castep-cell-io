# TSSEARCH_DISP_TOL

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_tssearch_disp_tol_castep.htm

**Group:** Transition state search

---

# TSSEARCH\_DISP\_TOL (.param)

## Keyword type

Real

## Description

This keyword controls the tolerance for accepting convergence of the ionic displacement during a transition state search.

This parameter must be greater than 0.

The maximum value of the ionic displacement during the last iteration must be less than
this value for convergence.

## Default

0.01 Bohr

## Example

```

TSSEARCH_DISP_TOL : 0.012 Bohr
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
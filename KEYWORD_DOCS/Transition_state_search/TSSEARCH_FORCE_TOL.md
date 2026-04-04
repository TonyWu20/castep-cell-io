# TSSEARCH_FORCE_TOL

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_tssearch_force_tol_castep.htm

**Group:** Transition state search

---

# TSSEARCH\_FORCE\_TOL (.param)

## Keyword type

Real

## Description

This keyword controls the tolerance for accepting convergence of the ionic force during a transition state search.

This parameter must be greater than 0.

The maximum value of the ionic force during the last iteration must be less than this value
for convergence.

## Default

0.005 Hartree Bohr-1

## Example

```

TSSEARCH_FORCE_TOL : 0.007 hartree/bohr
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
# IMPLICIT_SOLVENT_APOLAR_FACTOR

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_implicit_solvent_apolar_factor.htm

**Group:** Solvation energy parameters

---

# IMPLICIT\_SOLVENT\_APOLAR\_FACTOR (.param)

## Keyword type

Real

## Description

This keyword specifies the scaling factor for apolar terms in the solvation model. The default value gives excellent results for hydrocarbons in water. It gives good results for other solutes in water, almost universally improving on results obtained without the dispersion-repulsion term. The expected accuracy for other solvents is lower, but still it is worthwhile including.

## Default

0.281075 (dimensionless)

## Example

```
IMPLICIT_SOLVENT_APOLAR_FACTOR : 0.5
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
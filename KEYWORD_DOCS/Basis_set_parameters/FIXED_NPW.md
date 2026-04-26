# FIXED_NPW

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_fixed_npw_castep.htm

**Group:** Basis set parameters

---

# FIXED\_NPW (.param)

## Keyword type

Logical

## Description

This keyword determines whether a fixed number of plane waves (fixed size basis : TRUE) or a fixed plane wave cutoff energy (fixed quality
basis : FALSE) will be used when doing a variable cell calculation.

This setting affects geometry optimization with variable cell parameters and molecular dynamics with NPT or NPH ensembles.

You should turn off finite basis set correction when using a fixed size basis (see
[FINITE\_BASIS\_CORR](k_finite_basis_corr_castep.htm)).

## Default

FALSE

## Example

```

FIXED_NPW : TRUE
```

###### See Also:

[FINITE\_BASIS\_CORR](k_finite_basis_corr_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

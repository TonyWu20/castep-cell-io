# SEDC_S6_G06

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_sedc_s6_g06_castep.htm

**Group:** Electronic parameters

---

# SEDC\_S6\_G06 (.param)

## Keyword type

Real

## Description

This keyword controls the s6 value for the damping function in the Grimme (G06) dispersion correction scheme.

This parameter is used only if [SEDC\_APPLY](k_sedc_apply_castep.htm) : TRUE and [SEDC\_SCHEME](k_sedc_scheme_castep.htm) : G06.

## Default

The default value for this parameter depends on the value of [XC\_FUNCTIONAL](k_xc_functional_castep.htm). Please use the scientific literature for additional guidance.

## Example

```

SEDC_S6_G06 : 0.75
```

###### See Also:

[SEDC\_SCHEME](k_sedc_scheme_castep.htm)
  
[SEDC\_APPLY](k_sedc_apply_castep.htm)
  
Dispersion correction for DFT  
[CASTEP keyword glossary](k_glossary_castep.htm)  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

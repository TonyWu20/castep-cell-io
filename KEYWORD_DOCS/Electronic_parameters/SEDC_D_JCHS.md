# SEDC_D_JCHS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_sedc_d_jchs_castep.htm

**Group:** Electronic parameters

---

# SEDC\_D\_JCHS (.param)

## Keyword type

Real

## Description

This keyword controls the s6 value for the damping function in the JCHS dispersion correction scheme.

This parameter is used only if [SEDC\_APPLY](k_sedc_apply_castep.htm) : TRUE and [SEDC\_SCHEME](k_sedc_scheme_castep.htm) : JCHS.

## Default

The default value for this parameter depends on the value of [XC\_FUNCTIONAL](k_xc_functional_castep.htm). Please use the scientific literature for additional guidance.

## Example

```

SEDC_D_JCHS : 1.0
```

###### See Also:

[SEDC\_SCHEME](k_sedc_scheme_castep.htm)
  
[SEDC\_APPLY](k_sedc_apply_castep.htm)
  
Dispersion correction for DFT  
[CASTEP keyword glossary](k_glossary_castep.htm)  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

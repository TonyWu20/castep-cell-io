# SEDC_N_OBS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_sedc_n_obs_castep.htm

**Group:** Electronic parameters

---

# SEDC\_N\_OBS (.param)

## Keyword type

Real

## Description

This keyword controls the n value for the damping function in the Ortmann-Bechstedt-Schmidt (OBS) dispersion correction scheme.

This parameter is used only if [SEDC\_APPLY](k_sedc_apply_castep.htm) : TRUE and [SEDC\_SCHEME](k_sedc_scheme_castep.htm) : OBS.

## Default

The default value for this parameter depends on the value of [XC\_FUNCTIONAL](k_xc_functional_castep.htm). Please use the scientific literature for additional guidance.

## Example

```

SEDC_N_OBS : 8.0
```

###### See Also:

[SEDC\_SCHEME](k_sedc_scheme_castep.htm)
  
[SEDC\_APPLY](k_sedc_apply_castep.htm)
  
Dispersion correction for DFT  
[CASTEP keyword glossary](k_glossary_castep.htm)  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

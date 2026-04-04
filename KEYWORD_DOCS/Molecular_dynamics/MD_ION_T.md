# MD_ION_T

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_md_ion_t_castep.htm

**Group:** Molecular dynamics

---

# MD\_ION\_T (.param)

## Keyword type

Real

## Description

This keyword is used to set the relevant MD thermostat parameters, for example Nosé-Hoover thermostat mass or Langevin damping time.

This keyword is used only if [MD\_ENSEMBLE](k_md_ensemble_castep.htm) =
NVT or NPT.

## Default

10 × [MD\_DELTA\_T](k_md_delta_t_castep.htm) if
[MD\_THERMOSTAT](k_md_thermostat_castep.htm) : Nosé-Hoover

100 × [MD\_DELTA\_T](k_md_delta_t_castep.htm) if [MD\_THERMOSTAT](k_md_thermostat_castep.htm) :
Langevin

## Example

```

MD_ION_T : 0.5 ps
```

###### See Also:

[MD\_ENSEMBLE](k_md_ensemble_castep.htm)
  
[MD\_DELTA\_T](k_md_delta_t_castep.htm)
  
[MD\_THERMOSTAT](k_md_thermostat_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
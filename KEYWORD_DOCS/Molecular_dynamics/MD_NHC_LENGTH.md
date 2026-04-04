# MD_NHC_LENGTH

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_md_nhc_length_castep.htm

**Group:** Molecular dynamics

---

# MD\_NHC\_LENGTH (.param)

## Keyword type

Integer

## Description

This keyword is used to control the length of Nosé-Hoover thermostat chains. A chain of Nosé-Hoover thermostats of a given
length may be used to thermostat an NVT run or two separate chains (ions and cell) may be used to thermostat an NPT run.

This keyword is used only if [MD\_ENSEMBLE](k_md_ensemble_castep.htm) : NVT or NPT and [MD\_THERMOSTAT](k_md_thermostat_castep.htm) :
Nosé-Hoover.

## Default

5

## Example

```

MD_NHC_LENGTH : 3
```

###### See Also:

[MD\_ENSEMBLE](k_md_ensemble_castep.htm)
  
[MD\_THERMOSTAT](k_md_thermostat_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
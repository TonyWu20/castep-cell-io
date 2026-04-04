# MD_THERMOSTAT

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_md_thermostat_castep.htm

**Group:** Molecular dynamics

---

# MD\_THERMOSTAT (.param)

## Keyword type

String

## Description

This keyword determines the thermostat used for a molecular dynamics calculation. Available options are:

* Nose-Hoover
* Langevin

This parameter is used only if [MD\_ENSEMBLE](k_md_ensemble_castep.htm) : NVT.

## Default

The default value for this parameter is Nose-Hoover.

## Example

```

MD_THERMOSTAT : Langevin
```

###### See Also:

[MD\_ENSEMBLE](k_md_ensemble_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
# MD_EQM_METHOD

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_md_eqm_method_castep.htm

**Group:** Molecular dynamics

---

# MD\_EQM\_METHOD (.param)

## Keyword type

String

## Description

This advanced keyword determines the scheme to be used for enhanced MD equilibration. Equilibration is achieved by turning on a weakly
coupled thermostat and/or barostat (depending on the setting for [MD\_ENSEMBLE](k_md_ensemble_castep.htm)). The use of this option
allows you to speed up equilibration in the way that does not necessarily conform to any ensemble. Available options are:

* NONE
* BERENDSEN

The time scale of the ionic thermostat is controlled by [MD\_EQM\_ION\_T](k_md_eqm_ion_t_castep.htm), the cell barostat is controlled
by [MD\_EQM\_CELL\_T](k_md_eqm_cell_t_castep.htm). The weak coupling will be turned off after
[MD\_EQM\_T](k_md_eqm_t_castep.htm) seconds.

## Default

NONE

## Example

```

MD_EQM_METHOD : BERENDSEN
```

###### See Also:

[MD\_EQM\_CELL\_T](k_md_eqm_cell_t_castep.htm)
  
[MD\_EQM\_ION\_T](k_md_eqm_ion_t_castep.htm)
  
[MD\_EQM\_T](k_md_eqm_t_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
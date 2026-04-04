# MD_CELL_T

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_md_cell_t_castep.htm

**Group:** Molecular dynamics

---

# MD\_CELL\_T (.param)

## Keyword type

Real

## Description

This keyword is used to set the relevant MD barostat parameters, for example Nosé-Hoover barostat mass or Langevin damping time.

This keyword is used only if [MD\_ENSEMBLE](k_md_ensemble_castep.htm)
: NPT or NPH.

## Default

10 × [MD\_ION\_T](k_md_ion_t_castep.htm)

## Example

```

MD_CELL_T : 2 ps
```

###### See Also:

[MD\_ENSEMBLE](k_md_ensemble_castep.htm)
  
[MD\_ION\_T](k_md_ion_t_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
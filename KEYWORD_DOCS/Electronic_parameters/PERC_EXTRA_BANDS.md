# PERC_EXTRA_BANDS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_perc_extra_bands_castep.htm

**Group:** Electronic parameters

---

# PERC\_EXTRA\_BANDS (.param)

## Keyword type

Real

## Description

This keyword controls the percentage of extra bands in addition to the number of occupied bands. These extra bands are necessary for metals
or finite temperature insulators.

It is not possible to have both the [NBANDS](k_nbands_castep.htm) keyword and
either the [NEXTRA\_BANDS](k_nextra_bands_castep.htm) or BS\_PERC\_EXTRA\_BANDS keywords present in the same input file.

## Default

0

## Example

```

PERC_EXTRA_BANDS : 60.0
```

###### See Also:

[NBANDS](k_nbands_castep.htm)
  
[NEXTRA\_BANDS](k_nextra_bands_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
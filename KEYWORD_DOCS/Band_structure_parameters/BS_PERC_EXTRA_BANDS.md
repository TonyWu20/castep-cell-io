# BS_PERC_EXTRA_BANDS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_bs_perc_extra_bands_castep.htm

**Group:** Band structure parameters

---

# BS\_PERC\_EXTRA\_BANDS (.param)

## Keyword type

Real

## Description

This keyword controls the percentage of extra bands at each k-point in addition to the number of occupied bands, when performing a band
structure calculation.

It is not possible to have both the [BS\_NBANDS](k_bs_nbands_castep.htm) keyword
and either the [BS\_NEXTRA\_BANDS](k_bs_nextra_bands_castep.htm) or BS\_PERC\_EXTRA\_BANDS keywords present in the same input file.

## Default

0

The default value corresponds to a calculation of the band structure for the valence band only.
To calculate band structure for the conduction band use positive values of BS\_PERC\_EXTRA\_BANDS.

## Example

```

BS_PERC_EXTRA_BANDS : 60.0
```

###### See Also:

[BS\_NBANDS](k_bs_nbands_castep.htm)
  
[BS\_NEXTRA\_BANDS](k_bs_nextra_bands_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

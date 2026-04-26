# BS_NBANDS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_bs_nbands_castep.htm

**Group:** Band structure parameters

---

# BS\_NBANDS (.param)

## Keyword type

Integer

## Description

This keyword determines the number of bands at each k-point when performing a band structure calculation.

There are three ways in which you can specify the number of bands at each k-point:

1. Directly, using BS\_NBANDS.
2. Indirectly, by specifying the number of extra bands in addition to the number of occupied bands using
   [BS\_NEXTRA\_BANDS](k_bs_nextra_bands_castep.htm).
3. Indirectly, by specifying the number of extra bands in addition to the number of occupied bands as a percentage of the latter value using
   [BS\_PERC\_EXTRA\_BANDS](k_bs_perc_extra_bands_castep.htm).

It is not possible to have both the BS\_NBANDS keyword and either the
[BS\_NEXTRA\_BANDS](k_bs_nextra_bands_castep.htm) or [BS\_PERC\_EXTRA\_BANDS](k_bs_perc_extra_bands_castep.htm) keywords present
in the same input file.

## Default

NBANDS + 5√NBANDS

## Example

```

BS_NBANDS : 64
```

###### See Also:

[BS\_NEXTRA\_BANDS](k_bs_nextra_bands_castep.htm)
  
[BS\_PERC\_EXTRA\_BANDS](k_bs_perc_extra_bands_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)\n

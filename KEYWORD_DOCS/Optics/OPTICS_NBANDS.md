# OPTICS_NBANDS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_optics_nbands_castep.htm

**Group:** Optics

---

# OPTICS\_NBANDS (.param)

## Keyword type

Integer

## Description

This keyword determines the number of bands at each k-point when performing an optical spectrum calculation.

There are three ways in which you can specify the number of bands at each k-point:

1. Directly, using OPTICS\_NBANDS.
2. Indirectly, by specifying the number of extra bands in addition to the number of occupied bands using
   [OPTICS\_NEXTRA\_BANDS](k_optics_nextra_bands_castep.htm).

   This is the method used in the CASTEP interface.
3. Indirectly, by specifying the number of extra bands in addition to the number of occupied bands as a percentage of the latter value
   using [OPTICS\_PERC\_EXTRA\_BANDS](k_optics_perc_extra_bands_castep.htm).

It is not possible to have both the OPTICS\_NBANDS keyword and either the
[OPTICS\_NEXTRA\_BANDS](k_optics_nextra_bands_castep.htm) or [OPTICS\_PERC\_EXTRA\_BANDS](k_optics_perc_extra_bands_castep.htm)
keywords present in the same input file.

## Default

If OPTICS\_NBANDS is present in the parameter file, the value of [BS\_NBANDS](k_bs_nbands_castep.htm) is determined by this keyword.

If [OPTICS\_NEXTRA\_BANDS](k_optics_nextra_bands_castep.htm) is present in the parameter file:

```
	BS_NBANDS : max(NUP, NDOWN) + OPTICS_NEXTRA_BANDS.
```

If [OPTICS\_PERC\_EXTRA\_BANDS](k_optics_perc_extra_bands_castep.htm) is present in the parameter file:

```
	BS_NBANDS : max(NUP, NDOWN) x [1 + OPTICS_PERC_EXTRA_BANDS/100)].
```

The default value of this parameter is 3 × [NBANDS](k_nbands_castep.htm).

## Example

```

OPTICS_NBANDS : 192
```

###### See Also:

[OPTICS\_NEXTRA\_BANDS](k_optics_nextra_bands_castep.htm)
  
[OPTICS\_PERC\_EXTRA\_BANDS](k_optics_perc_extra_bands_castep.htm)
  
[BS\_NBANDS](k_bs_nbands_castep.htm)
  
[NUP](k_nup_castep.htm)
  
[NDOWN](k_ndown_castep.htm)
  
[NBANDS](k_nbands_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
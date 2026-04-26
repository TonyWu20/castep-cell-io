# NBANDS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_nbands_castep.htm

**Group:** Electronic parameters

---

# NBANDS (.param)

## Keyword type

Integer

## Description

This keyword determines the maximum number of bands at any k-point and spin.

There are three ways in which you can specify the maximum number of bands at any k-point and spin:

1. Directly, using NBANDS.
2. Indirectly, by specifying the number of extra bands in addition to the number of occupied bands using
   [NEXTRA\_BANDS](k_nextra_bands_castep.htm).

   This is the method used in the CASTEP interface.
3. Indirectly, by specifying the number of extra bands in addition to the number of occupied bands as a percentage of the latter value
   using [PERC\_EXTRA\_BANDS](k_perc_extra_bands_castep.htm).

It is not possible to have both the NBANDS keyword and either the
[NEXTRA\_BANDS](k_nextra_bands_castep.htm) or [PERC\_EXTRA\_BANDS](k_perc_extra_bands_castep.htm) keywords present in the
same input file.

## Default

If [NEXTRA\_BANDS](k_nextra_bands_castep.htm) is specified and [SPIN\_POLARIZED](k_spin_polarized_castep.htm) : FALSE:

```
	NBANDS : (NELECTRONS/2) + NEXTRA_BANDS
```

Or, if [SPIN\_POLARIZED](k_spin_polarized_castep.htm) : TRUE:

```

	NBANDS : max(NUP, NDOWN) + NEXTRA_BANDS.
```

If [PERC\_EXTRA\_BANDS](k_perc_extra_bands_castep.htm) is specified and [SPIN\_POLARIZED](k_spin_polarized_castep.htm) : FALSE:

```

	NBANDS : (NELECTRONS/2) × [ 1 + (PERC_EXTRA_BANDS/100)]
```

Or, if [SPIN\_POLARIZED](k_spin_polarized_castep.htm) : TRUE:

```

	NBANDS : max(NUP, NDOWN) × [ 1 + (PERC_EXTRA_BANDS/100)]
```

If NBANDS, [NEXTRA\_BANDS](k_nextra_bands_castep.htm) and [PERC\_EXTRA\_BANDS](k_perc_extra_bands_castep.htm) are not
specified and [FIX\_OCCUPANCY](k_fix_occupancy_castep.htm) : TRUE, then, if
[SPIN\_POLARIZED](k_spin_polarized_castep.htm) : FALSE:

```

	NBANDS : NELECTRONS/2. 
```

Or, if [SPIN\_POLARIZED](k_spin_polarized_castep.htm) : TRUE:

```

	NBANDS : max(NUP, NDOWN)
```

If [FIX\_OCCUPANCY](k_fix_occupancy_castep.htm) : FALSE, these default values of NBANDS will be multiplied by 1.2.

## Example

```

NBANDS : 64
```

###### See Also:

[NEXTRA\_BANDS](k_nextra_bands_castep.htm)
  
[PERC\_EXTRA\_BANDS](k_perc_extra_bands_castep.htm)
  
[SPIN\_POLARIZED](k_spin_polarized_castep.htm)
  
[NELECTRONS](k_nelectrons_castep.htm)
  
[NUP](k_nup_castep.htm)
  
[NDOWN](k_ndown_castep.htm)
  
[FIX\_OCCUPANCY](k_fix_occupancy_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

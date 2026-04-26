# OPTICS_NEXTRA_BANDS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_optics_nextra_bands_castep.htm

**Group:** Optics

---

# OPTICS\_NEXTRA\_BANDS (.param)

## Keyword type

Integer

## Description

This keyword controls the number of extra bands at each k-point in addition to the number of occupied bands, when performing an optical
spectrum calculation.

It is not possible to have both the [OPTICS\_NBANDS](k_optics_nbands_castep.htm)
keyword and either the OPTICS\_NEXTRA\_BANDS or [OPTICS\_PERC\_EXTRA\_BANDS](k_optics_perc_extra_bands_castep.htm) keywords present in
the same input file.

## Default

0

The default value corresponds to a calculation of the optical spectrum for the valence band
only. To calculate optical spectra for the conduction band use positive values of OPTICS\_NEXTRA\_BANDS.

## Example

```

OPTICS_NEXTRA_BANDS : 36
```

###### See Also:

[OPTICS\_NBANDS](k_optics_nbands_castep.htm)
  
[OPTICS\_PERC\_EXTRA\_BANDS](k_optics_perc_extra_bands_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

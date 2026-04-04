# CALCULATE_BORN_CHARGES

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_calculate_born_charges_castep.htm

**Group:** Phonon parameters

---

# CALCULATE\_BORN\_CHARGES (.param)

## Keyword type

Logical

## Description

This keyword specifies whether to compute Born effective charge tensors as part of a phonon or electric field linear-response calculation.

TRUE means that the calculation of the dielectric permittivity is carried out using a linear response
scheme for an applied electric field. The Born effective charges are also evaluated.

This keyword should be set to TRUE if
[PHONON\_CALC\_LO\_TO\_SPLITTING](k_phonon_calc_lo_to_splitting_castep.htm) : TRUE or
[EFIELD\_CALC\_ION\_PERMITTIVITY](k_efield_calc_ion_permittivity_castep.htm) : TRUE.

## Default

TRUE

## Example

```

CALCULATE_BORN_CHARGES : FALSE
```

###### See Also:

[PHONON\_CALC\_LO\_TO\_SPLITTING](k_phonon_calc_lo_to_splitting_castep.htm)
  
[EFIELD\_CALC\_ION\_PERMITTIVITY](k_efield_calc_ion_permittivity_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
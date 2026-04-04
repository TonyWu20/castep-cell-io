# PHONON_CALC_LO_TO_SPLITTING

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_phonon_calc_lo_to_splitting_castep.htm

**Group:** Phonon parameters

---

# PHONON\_CALC\_LO\_TO\_SPLITTING (.param)

## Keyword type

Logical

## Description

This keyword specifies whether to calculate the non-analytical contribution to the dynamical matrix from long-range electric field effects,
which are responsible for the LO/TO splitting of phonon frequencies at the gamma point.

When this keyword is TRUE, the calculation of the dielectric permittivity is carried out using a linear response
scheme for an applied electric field. The Born effective charges are also evaluated.

You should set this keyword to FALSE for molecule in a box systems,
where there is no physical LO-TO splitting of phonon frequencies.

The value of the LO-TO splitting depends on the approach direction to gamma. It is possible
to specify multiple directions using the [PHONON\_GAMMA\_DIRECTIONS](k_phonon_gamma_directions_castep.htm) block.

## Default

TRUE

## Example

```

PHONON_CALC_LO_TO_SPLITTING : FALSE
```

###### See Also:

[PHONON\_GAMMA\_DIRECTIONS](k_phonon_gamma_directions_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
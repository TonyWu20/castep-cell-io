# PHONON_GAMMA_DIRECTIONS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_phonon_gamma_directions_castep.htm

**Group:** q-vectors for phonon calculations

---

# PHONON\_GAMMA\_DIRECTIONS (.cell)

## Keyword type

Block

## Description

This data block contains a list of directions of approach to gamma point. These are used to evaluate the non-analytical part of the LO/TO
phonon splitting. It has the following format:

```

%BLOCK PHONON_GAMMA_DIRECTIONS
	R1i     R1j     R1k 
	R2i     R2j     R2k 
	.
	.
	.
%ENDBLOCK PHONON_GAMMA_DIRECTIONS
```

The three numbers on each line are the fractional positions of the k-point relative to the reciprocal space lattice vectors. The k-points
define a continuous sequence of straight line segments. The path will be open unless the first and last point in the list are identical.

This block affects the calculated phonon frequencies only if the
[PHONON\_CALC\_LO\_TO\_SPLITTING](k_phonon_calc_lo_to_splitting_castep.htm) keyword in the `.param` file is set to
TRUE.

## Example

```

%BLOCK PHONON_GAMMA_DIRECTIONS
	1.0 0.0 0.0
	0.0 0.0 1.0
%ENDBLOCK PHONON_GAMMA_DIRECTIONS
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
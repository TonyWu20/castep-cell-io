# POSITIONS_FRAC

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_positions_frac_castep.htm

**Group:** Ionic positions

---

# POSITIONS\_FRAC (.cell)

## Keyword type

Block

## Description

This data block contains the ionic positions in fractional coordinates.

The POSITIONS\_FRAC or [POSITIONS\_ABS](k_positions_abs_castep.htm) data block is
interpreted as the reactant coordinate data for a transition state search.

The data block has the following format:

```

%BLOCK POSITIONS_FRAC
	CCC1/I1 	    R1i     R1j     R1k
	CCC2/I2 	    R2i     R2j     R2k
	.
	.
	.
%ENDBLOCK POSITIONS_FRAC	
```

The first entry on a line is the symbol or atomic number of the ionic species. The correct symbol will be added automatically for atomic
species if the atomic number is specified. A symbol can have a maximum of three characters.

The next three entries are real numbers representing the position of the ion in fractions of the unit cell lattice vectors.

Only one of POSITIONS\_FRAC and [POSITIONS\_ABS](k_positions_abs_castep.htm)
may be present in a single cell file.

## Example

```

%BLOCK POSITIONS_FRAC
	.
	.
    O     0.7500000000   -0.2500000000    0.2500000000
    Al    0.6955000000    0.2500000000    0.9186000000
	.
	.
%ENDBLOCK POSITIONS_FRAC
```

###### See Also:

[POSITIONS\_ABS](k_positions_abs_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

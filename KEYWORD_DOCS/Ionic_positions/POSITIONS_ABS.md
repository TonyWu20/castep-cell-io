# POSITIONS_ABS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_positions_abs_castep.htm

**Group:** Ionic positions

---

# POSITIONS\_ABS (.cell)

## Keyword type

Block

## Description

This data block contains the ionic positions in absolute coordinates.

The POSITIONS\_ABS or [POSITIONS\_FRAC](k_positions_frac_castep.htm) data block is
interpreted as the reactant coordinate data for a transition state search.

The data block has the following format:

```

%BLOCK POSITIONS_ABS
[units]
	CCC1/I1     	R1x     R1y     R1z
	CCC2/I2 	    R2x     R2y     R2z
	.
	.
	.
%ENDBLOCK POSITIONS_ABS	
```

The first entry on a line is the symbol or atomic number of the ionic species. The correct symbol will be added automatically for atomic
species if the atomic number is specified. A symbol can have a maximum of three characters.

The next three entries are real numbers representing the position of the ion in Cartesian coordinates.

[units] specifies the units in which the positions are defined. If no units are given, the default of Å is used.

Only one of [POSITIONS\_FRAC](k_positions_frac_castep.htm) and POSITIONS\_ABS may
be present in a single cell file.

## Example

```

%BLOCK POSITIONS_ABS
	.
	.
    O     6.2450000000   -2.3470000000    3.0440000000
    Al    5.7110000000    2.3470000000    5.0230000000
	.
	.
%ENDBLOCK POSITIONS_ABS
```

###### See Also:

[POSITIONS\_FRAC](k_positions_frac_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
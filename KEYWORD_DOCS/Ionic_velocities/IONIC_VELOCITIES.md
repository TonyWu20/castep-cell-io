# IONIC_VELOCITIES

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_ionic_velocities_castep.htm

**Group:** Ionic velocities

---

# IONIC\_VELOCITIES (.cell)

## Keyword type

Block

## Description

This data block contains the initial velocities of the ions, in Cartesian coordinates. It has the following format:

```

%BLOCK IONIC_VELOCITIES
[units]
	V1x     V1y     V1z
	V2x     V2y     V2z
	.
	.
	.
%ENDBLOCK IONIC_VELOCITIES
```

The entries are real numbers representing the velocity of the ion in Cartesian coordinates.

[units] specifies the units in which the velocities are defined. If no units are given, the default of Å/ps is used.

If this keyword is not present and a molecular dynamics calculation is performed, the ionic velocities will be randomly initialized with
respect to the simulation temperature.

## Example

```

%BLOCK IONIC_VELOCITIES
ang/ps
 1.000  2.000  1.000
 2.000  3.000  3.000
-2.000 -3.000 -3.000
-1.000 -2.000 -1.000 
%ENDBLOCK IONIC_VELOCITIES
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
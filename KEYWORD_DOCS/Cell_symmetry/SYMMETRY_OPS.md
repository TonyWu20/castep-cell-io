# SYMMETRY_OPS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_symmetry_ops_castep.htm

**Group:** Cell symmetry

---

# SYMMETRY\_OPS (.cell)

## Keyword type

Block

## Description

This data block contains the symmetry operations under which the unit cell is invariant. Each operation is represented by a 3 × 3 array.
The data block has the following format:

```

%BLOCK SYMMETRY_OPS
	R11        R21        R31
	R12        R22        R32
	R13        R23        R33
	T1         T2         T3
	R11        R21        R31
	R12        R22        R32
	R13        R23        R33
	T1         T2         T3
	.
	.
	.
%ENDBLOCK SYMMETRY_OPS
```

Each of the first three lines contains 3 entries representing a row of a 3 × 3 array. These represent one symmetry rotation. The three
entries on the following line contain the translation associated with this rotation.

If no symmetry is specified in the cell definition file, the default is for no symmetry to
be applied.

## Example

```

%BLOCK SYMMETRY_OPS
	.
	.
	.
   -1.0000000000    0.0000000000    0.0000000000
    0.0000000000   -1.0000000000    0.0000000000
    0.0000000000    0.0000000000    1.0000000000
    0.5000000000    0.0000000000    0.5000000000
	.
	.
	.
%ENDBLOCK SYMMETRY_OPS
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
# NONLINEAR_CONSTRAINTS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_nonlinear_constraints_castep.htm

**Group:** Constraints

---

# NONLINEAR\_CONSTRAINTS (.cell)

## Keyword type

Block

## Description

This data block defines constrained internal coordinates (bonds, angles, torsions). These internals are held fixed at the values of the
initial structure.

The data block has the following format:

```

%BLOCK NONLINEAR_CONSTRAINTS
	CONSTRAIN_TYPE atom1 atom2 (atom3 atom4)
	CONSTRAIN_TYPE atom1 atom2 (atom3 atom4)
	.
	.
	.
%ENDBLOCK NONLINEAR_CONSTRAINTS	
```

The first element on each line is a constraint type (distance, bend or torsion). Then depending on the type of the constraint two, three or
for fields describing atoms involved in the constraint appears. Each field has the following format:

```

CCC  I N1 N2 N3
```

The first entry is the symbol species of the ion to which this constraint applies. The second element is the number of the ion within the
species. The final three numbers are integer numbers specifying the periodic image where the atom is located.

The ordering of the ions in a species is the order in which they appear in the [POSITIONS\_FRAC](k_positions_frac_castep.htm) or
[POSITIONS\_ABS](k_positions_abs_castep.htm) block in the cell definition file.

## Example

```

%BLOCK NONLINEAR_CONSTRAINTS
distance       H  4  0  0  0       O  2  0  1  0
    bend       H  5  0  0  0       C  1  1  0  1       H  2  0  0  0
 torsion       H  6  0  0  0       H  3  1  0  0       H  1  0  0  1       H  9  1  1  0
%ENDBLOCK NONLINEAR_CONSTRAINTS
```

This input fixes:

* the distance between the fourth hydrogen atom and second oxygen atom in the adjacent ( 0 1 0) unit cell
* the bend defined by the fifth hydrogen atom, the first carbon atom in the (1 0 1) cell, and the second hydrogen atom
* the torsion defined by hydrogen atoms: the sixth in the original cell, the third in the cell (1 0 0), the first in the cell
  (0 0 1), and the ninth in the (1 1 0)

###### See Also:

[POSITIONS\_FRAC](k_positions_frac_castep.htm)
  
[POSITIONS\_ABS](k_positions_abs_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

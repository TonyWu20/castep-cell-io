# IONIC_CONSTRAINTS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_ionic_constraints_castep.htm

**Group:** Constraints

---

# IONIC\_CONSTRAINTS (.cell)

## Keyword type

Block

## Description

This data block defines constraints on changes to the Cartesian ionic positions during relaxation or molecular dynamics. These are specified
as a set of linear constraints. Each constraint is made up of a series of coefficients aijk such that:

![](../../../generatedimages/equations/equation123.png)

Where:

x is the number of species in the system  
y is the number of ions in species k

The data block has the following format:

```

%BLOCK IONIC_CONSTRAINTS
	I1 CCC1s/I1s In1 R1i R1j R1k
	I2 CCC2s/I2s In2 R2i R2j R2k
	.
	.
	.
%ENDBLOCK IONIC_CONSTRAINTS	
```

The first element on each line is an integer specifying the number of the constraint being specified. The second entry is either the symbol
or atomic number of the species of the ion to which this constraint applies. The third element is the number of the ion within the species.

The ordering of the ions in a species is the order in which they appear in the [POSITIONS\_FRAC](k_positions_frac_castep.htm)
or [POSITIONS\_ABS](k_positions_abs_castep.htm) block in the cell definition file.

The final three numbers are real numbers representing the coefficients of the Cartesian coordinates of the ionic position in the constraint
sum. All coefficients in the sum that are not explicitly specified will be set to zero.

On reading this data, the matrix of ionic constraints will be orthogonalized.

Fixing Cartesian coordinates together with cell optimization is not well defined procedure,
as it is not clear what exactly should be kept (each atom has an infinite set of periodically repeated images and if translation vector is
changed under optimization step Cartesian coordinates of all images cannot be kept the same). At the same time fixing all fractional coordinates
of an atom is still perfectly valid operation under this circumstances, thus if all the Cartesian coordinate of an atom are asked to be fixed
CASTEP will fix fractional coordinates of that atom instead.

## Example

To fix the Cartesian coordinates of the first four tungsten atoms in a cell:

```

%BLOCK IONIC_CONSTRAINTS
       1       W       1    1.0000000000    0.0000000000    0.0000000000
       2       W       1    0.0000000000    1.0000000000    0.0000000000
       3       W       1    0.0000000000    0.0000000000    1.0000000000
       4       W       2    1.0000000000    0.0000000000    0.0000000000
       5       W       2    0.0000000000    1.0000000000    0.0000000000
       6       W       2    0.0000000000    0.0000000000    1.0000000000
       7       W       3    1.0000000000    0.0000000000    0.0000000000
       8       W       3    0.0000000000    1.0000000000    0.0000000000
       9       W       3    0.0000000000    0.0000000000    1.0000000000
      10       W       4    1.0000000000    0.0000000000    0.0000000000
      11       W       4    0.0000000000    1.0000000000    0.0000000000
      12       W       4    0.0000000000    0.0000000000    1.0000000000
%ENDBLOCK IONIC_CONSTRAINTS
```

###### See Also:

[POSITIONS\_FRAC](k_positions_frac_castep.htm)
  
[POSITIONS\_ABS](k_positions_abs_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

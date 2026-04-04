# CELL_CONSTRAINTS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_cell_constraints_castep.htm

**Group:** Constraints

---

# CELL\_CONSTRAINTS (.cell)

## Keyword type

Block

## Description

This data block defines constraints on changes to the cell during relaxation or molecular dynamics. It has the following format:

```

%BLOCK CELL_CONSTRAINTS
	Ia Ib Ic
	Iα Iβ Iγ
%ENDBLOCK CELL_CONSTRAINTS
```

The first three entries relate to the magnitude of the three lattice parameters a, b and c and the second set of three entries to the angles
α, β and γ.

If the value of the entry corresponding to a magnitude or angle is zero, this quantity will remain fixed. If two or three entries contain
the same integer, the corresponding quantities will be constrained to have the same value. If a positive integer greater than 0 occurs in
entries 1 through 3 the same integer cannot occur in entries 4 through 6 as this would imply that a vector length and angle must have the same
value.

## Example

To allow independent optimization of the lattice parameters a, b and c while keeping all three angles fixed, specify:

```

%BLOCK CELL_CONSTRAINTS
       1       2       3
       0       0       0
%ENDBLOCK CELL_CONSTRAINTS
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
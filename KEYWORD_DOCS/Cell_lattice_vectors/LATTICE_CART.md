# LATTICE_CART

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_lattice_cart_castep.htm

**Group:** Cell lattice vectors

---

# LATTICE\_CART (.cell)

## Keyword type

Block

## Description

This data block contains the cell lattice vectors in Cartesian coordinates. It has the following format:

```

%BLOCK LATTICE_CART
[units]
	R1x R1y R1z
	R2x R2y R2z
	R3x R3y R3z
%ENDBLOCK LATTICE_CART
```

Where R1x is the x-component of the first lattice vector, R2y is the y-component of the second lattice vector, and so on.

[units] specifies the units in which the lattice vectors are defined. If no units are given, the default of Å is used.

Only one of LATTICE\_CART and [LATTICE\_ABC](k_lattice_abc_castep.htm) may be
present in a single cell file.

## Example

```

%BLOCK LATTICE_CART
    8.9780000000    0.0000000000    0.0000000000
    0.0000000000    5.7400000000    0.0000000000
    0.0000000000    0.0000000000    9.9690000000
%ENDBLOCK LATTICE_CART
```

###### See Also:

[LATTICE\_ABC](k_lattice_abc_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
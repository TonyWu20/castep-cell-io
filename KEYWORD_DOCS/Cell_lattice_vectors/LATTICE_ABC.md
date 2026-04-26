# LATTICE_ABC

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_lattice_abc_castep.htm

**Group:** Cell lattice vectors

---

# LATTICE\_ABC (.cell)

## Keyword type

Block

## Description

This data block contains the cell lattice vectors in terms of the lattice vector magnitudes and the angles between them (a, b, c, α,
β and γ). It has the following format:

```

%BLOCK LATTICE_ABC
[units]
	Ra Rb Rc
	Rα Rβ Rγ
%ENDBLOCK LATTICE_ABC
```

Where Ra is the value of a, Rγ is the value of γ, and so on.

[units] specifies the units in which the lattice vector magnitudes are defined. If no units are given, the default of Å
is used. Angles should be specified in degrees.

Only one of [LATTICE\_CART](k_lattice_cart_castep.htm) and LATTICE\_ABC may be
present in a single cell file.

## Example

```

%BLOCK LATTICE_ABC
    8.9780000000    5.7400000000	9.9690000000
    90.000000000    90.000000000	90.000000000
%ENDBLOCK LATTICE_ABC
```

###### See Also:

[LATTICE\_CART](k_lattice_cart_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

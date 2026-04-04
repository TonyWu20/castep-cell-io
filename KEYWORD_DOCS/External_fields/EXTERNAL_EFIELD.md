# EXTERNAL_EFIELD

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_external_efield_castep.htm

**Group:** External fields

---

# EXTERNAL\_EFIELD (.cell)

## Keyword type

Block

## Description

This data block contains electric field vector in Cartesian coordinates.

The data block has the following format:

```

%BLOCK EXTERNAL_EFIELD 
[units]
Ex Ey Ez 
%ENDBLOCK EXTERNAL_EFIELD 
```

The first line is optional and contains electric field units. The default unit is eV/Å/electron (eV/Ang/e in CASTEP notation).
The next line contains three real numbers representing electric field vector components in Cartesian coordinates.

The external electric field is applied in a sawtooth formalism. Electric field discontinuity
is on the cell boundary, so the only sensible systems to study using this approach are molecules or slabs (the field must be along the surface
normal in the latter case). In both cases atoms should be positioned in the middle of the cell.

## Example

```

%BLOCK EXTERNAL_EFIELD 
HARTREE/BOHR/E
0.0 0.0 0.1
%ENDBLOCK EXTERNAL_EFIELD 
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
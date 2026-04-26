# SPECIES_Q

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_species_q_castep.htm

**Group:** Species characteristics

---

# SPECIES\_Q (.cell)

## Keyword type

Block

## Description

This data block contains the nuclear electric quadrupole moment of each atomic species. It has the following format:

```

%BLOCK SPECIES_Q
[units]
	CCC1/I1 	R1
	CCC2/I2 	R2
	.
	.
	.
%ENDBLOCK SPECIES_Q
```

The first entry on a line is the symbol or atomic number of the species. This must correspond with the species symbol or atomic number of
the species in the [POSITIONS\_FRAC](k_positions_frac_castep.htm) or [POSITIONS\_ABS](k_positions_abs_castep.htm) block.
The second entry on each line is the quadrupole moment of that species.

Not all of the species present in the system have to appear in the SPECIES\_Q block. Any that are not present will be assigned the default
quadrupole moment for that species. However, if the symbol specified for a species is not a standard symbol in the periodic table, the
quadrupole moment of the species must be specified.

The *units* entry specifies the units in which the quadrupole moment is defined. If no units are given, the default unit of atomic
quadrupole moment, Barn, is used. The only other supported unit is fm2, that is,
femtometer2.

## Example

```

%BLOCK SPECIES_Q
Barn
       B      0.0405900000
       N      0.0201000000
%ENDBLOCK SPECIES_Q
```

###### See Also:

[POSITIONS\_FRAC](k_positions_frac_castep.htm)
  
[POSITIONS\_ABS](k_positions_abs_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

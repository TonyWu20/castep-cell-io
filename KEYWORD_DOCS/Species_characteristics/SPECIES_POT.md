# SPECIES_POT

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_species_pot_castep.htm

**Group:** Species characteristics

---

# SPECIES\_POT (.cell)

## Keyword type

Block

## Description

This data block contains the names of the pseudopotential files associated with each species. It has the following format:

```

%BLOCK SPECIES_POT
	CCC1/I1 	[filename]
	CCC2/I2 	[filename]
	.
	.
	.
%ENDBLOCK SPECIES_POT
```

The first entry on a line is the symbol or atomic number of the species. This must correspond with the species symbol or atomic number of
the species in the [POSITIONS\_FRAC](k_positions_frac_castep.htm) or [POSITIONS\_ABS](k_positions_abs_castep.htm) block.
The second entry on each line is the name of the file containing the definition of the pseudopotential representing that ionic species.

The *filename* qualifier in the SPECIES\_POT block may be either the name of the file that contains the tabulated pseudopotential on a
grid (can be either ultrasoft or norm-conserving) or a text string containing the definition of the pseudopotential which is to be generated at
runtime (on-the-fly generation).

Not all of the species present in the system have to appear in the SPECIES\_POT block. Any that are not present will be assigned the default
pseudopotential for that species, which will be generated on the fly. However, if the symbol specified for a species is not a standard symbol
in the periodic table, the potential for the species must be specified.

The charge on the ion for each species will be derived from the pseudopotential corresponding to that ion.

## Example

```

%BLOCK SPECIES_POT
       O  O_00.usp
      Al  Al_00.usp
      Ti  Ti_00.uspcc
      Cs  Cs_00.usp
%ENDBLOCK SPECIES_POT
```

###### See Also:

[POSITIONS\_FRAC](k_positions_frac_castep.htm)
  
[POSITIONS\_ABS](k_positions_abs_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
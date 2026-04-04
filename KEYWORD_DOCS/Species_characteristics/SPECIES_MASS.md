# SPECIES_MASS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_species_mass_castep.htm

**Group:** Species characteristics

---

# SPECIES\_MASS (.cell)

## Keyword type

Block

## Description

This data block contains mass of each atomic species. It has the following format:

```

%BLOCK SPECIES_MASS
[units]
	CCC1/I1 	R1
	CCC2/I2 	R2
	.
	.
	.
%ENDBLOCK SPECIES_MASS
```

The first entry on a line is the symbol or atomic number of the species. This must correspond with the species symbol or atomic number of the
species in the [POSITIONS\_FRAC](k_positions_frac_castep.htm) or [POSITIONS\_ABS](k_positions_abs_castep.htm) block. The
second entry on each line is the mass of that species.

Not all of the species present in the system have to appear in the SPECIES\_MASS block. Any that are not present will be assigned the default
mass for that species. However, if the symbol specified for a species is not a standard symbol in the periodic table, the mass of the species
must be specified.

[units] specifies the units in which the masses are defined. If no units are given, the default of atomic mass units is used.

## Example

```

%BLOCK SPECIES_MASS
       O     15.9989995956
      Al     26.9820003510
      Ti     47.9000015259
      Cs    132.9049987793
%ENDBLOCK SPECIES_MASS
```

###### See Also:

[POSITIONS\_FRAC](k_positions_frac_castep.htm)
  
[POSITIONS\_ABS](k_positions_abs_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
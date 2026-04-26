# SPECIES_LCAO_STATES

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_species_lcao_states_castep.htm

**Group:** Species characteristics

---

# SPECIES\_LCAO\_STATES (.cell)

## Keyword type

Block

## Description

This data block defines the size of the LCAO basis set used for population analysis. It has the following format:

```

%BLOCK SPECIES_LCAO_STATES
	CCC1/I1 	IB1
	CCC2/I2 	IB2
	.
	.
	.
%ENDBLOCK SPECIES_LCAO_STATES
```

The first entry on a line is the symbol or atomic number of the species. This must correspond with the species symbol or atomic number of
the species in the [POSITIONS\_FRAC](k_positions_frac_castep.htm) or [POSITIONS\_ABS](k_positions_abs_castep.htm) block.
The second entry is the number of angular momentum channels to use in the LCAO basis set for the species when performing population analysis.

For example, to use the 2s and 2p states for C (the 1s state is a core state) this value should be 2. By default, the number of states will
be the appropriate number to complete the valence shell to the next noble gas. If shallow core states are excluded from a pseudopotential, the
value of SPECIES\_LCAO\_STATES for that species should be included in the cell file to ensure a meaningful basis set is used.

## Example

```

%BLOCK SPECIES_LCAO_STATES
       O         2
      Al         2
      Ti         3
      Cs         4
%ENDBLOCK SPECIES_LCAO_STATES
```

###### See Also:

[POSITIONS\_FRAC](k_positions_frac_castep.htm)
  
[POSITIONS\_ABS](k_positions_abs_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

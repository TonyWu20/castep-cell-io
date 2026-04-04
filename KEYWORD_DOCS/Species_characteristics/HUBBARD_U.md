# HUBBARD_U

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_hubbard_u_castep.htm

**Group:** Species characteristics

---

# HUBBARD\_U (.cell)

## Keyword type

Block

## Description

This data block defines the Hubbard U values to use for specific orbitals, with the following format:

```
%BLOCK HUBBARD_U
 UNITS
	atom1  orbital1 orbital2 ....
	atom2  orbital1 orbital2 ....
	.
	.
	.
%ENDBLOCK HUBBARD_U
```

The first (optional) line declares the units to use for Hubbard U values, eV (default) or Ha. Each following line describes the orbitals
affected by the Hubbard term.

The first element on each obligatory line (atom) describes the ion and can contain either only the species symbol for the ion (in which case the same
Hubbard U values will be applied to all ions with this name) or both the species symbol for the ion and its number within the species.

The order of the ions for a species is the order in which they appear in the [POSITIONS\_FRAC](k_positions_frac_castep.htm) or
[POSITIONS\_ABS](k_positions_abs_castep.htm) block in the cell definition file.

After an ion is defined, the Hubbard U value for each orbital of the ion can be defined, with the format:

```
l: U
```

where l represents the s, p, d, or f orbital and U is a real number specifying the value of U, for example:

```
d: 2.3
```

All the values which are not defined are taken to be zero. If all the values for all orbitals are zero, LDA+U is turned off.

## Example

```
%BLOCK HUBBARD_U
  eV
    Sm 1   f: 6.1
    Ni     d: 2.4  
    U 2  d: 1.2 f: 2.1
%ENDBLOCK HUBBARD_U
```

This input defines the following Hubbard U values:

* f orbitals of the first Sm ion are 6.1 eV
* d orbitals of all Ni ions are 2.4 eV
* d orbitals of second U ion are 1.2 eV and its f orbitals are 2.1 eV

###### See Also:

[POSITIONS\_FRAC](k_positions_frac_castep.htm)
  
[POSITIONS\_ABS](k_positions_abs_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
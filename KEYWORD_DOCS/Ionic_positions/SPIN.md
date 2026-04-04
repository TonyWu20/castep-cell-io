# SPIN

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_spin_cell_castep.htm

**Group:** Ionic positions

---

# SPIN (.cell)

## Keyword type

Qualifier

## Description

This keyword can be added to any line containing atomic coordinates in either [POSITIONS\_ABS](k_positions_abs_castep.htm) or
[POSITIONS\_FRAC](k_positions_frac_castep.htm) data blocks, to specify the initial spin polarization on a given atom. The value of
atomic spin polarization is determined from the electronic configuration of the atom simply as:

Nup-Ndown

where Nup and Ndown are the numbers of electrons with spin up and down, respectively.

For example, a Cr atom with 4 up and 2 down electrons would have a polarization of 2. If the SPIN keyword is found in the list of atoms at
least once, CASTEP assumes zero spin polarization for all atoms without the qualifier.

The SPIN keyword is a substitute for the deprecated [MAGMOM](k_magmom_castep.htm)
keyword.

The SPIN keyword should be specified only in the `.cell` file for the main SCF
calculation.

## Example

```

%BLOCK POSITIONS_FRAC
    Fe 0.00    0.00    0.00
    Fe 0.50    0.50    0.00 SPIN= 4
    Fe 0.50    0.00    0.50 SPIN : -2
    Fe 0.00    0.50    0.50 SPIN 4
%ENDBLOCK POSITIONS_FRAC
```

###### See Also:

[MAGMOM](k_magmom_castep.htm)
  
[POSITIONS\_ABS](k_positions_abs_castep.htm)
  
[POSITIONS\_FRAC](k_positions_frac_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
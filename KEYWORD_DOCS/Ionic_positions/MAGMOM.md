# MAGMOM

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_magmom_castep.htm

**Group:** Ionic positions

---

# MAGMOM (.cell)

## Keyword type

Qualifier

## Description

This keyword can be added to any line containing atomic coordinates in either [POSITIONS\_ABS](k_positions_abs_castep.htm) or
[POSITIONS\_FRAC](k_positions_frac_castep.htm) data blocks, to specify the initial spin polarization on a given atom. The value of
atomic spin polarization is determined from the electronic configuration of the atom as:

(Nup-Ndown)/(Nup+Ndown)

Where Nup and Ndown are the numbers of electrons with spin up and down, respectively.

For example, a Cr atom with 4 up and 2 down electrons would have a polarization of (4-2)/6 = 0.3333. If the MAGMOM keyword is found in the
list of atoms at least once, CASTEP assumes zero spin polarization for all atoms without the qualifier.

The MAGMOM keyword should be specified only in the `.cell` file for the main SCF
calculation.

The MAGMOM keyword is deprecated; an easier to use substitute is the
[SPIN](k_spin_castep.htm) keyword.

## Example

```

%BLOCK POSITIONS_FRAC
    Fe 0.00    0.00    0.00
    Fe 0.50    0.50    0.00 MAGMOM= 0.5
    Fe 0.50    0.00    0.50 MAGMOM : -0.2
    Fe 0.00    0.50    0.50 MAGMOM 0.5
%ENDBLOCK POSITIONS_FRAC
```

###### See Also:

[POSITIONS\_ABS](k_positions_abs_castep.htm)
  
[POSITIONS\_FRAC](k_positions_frac_castep.htm)
  
[SPIN](k_spin_cell_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP cell keywords and data blocks](k_main_structure.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

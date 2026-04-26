# SPIN

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_spin_castep.htm

**Group:** Electronic parameters

---

# SPIN (.param)

## Keyword type

Real

## Description

This keyword determines the initial value for the number of unpaired electrons in a spin-polarized calculation. This value may be optimized
during the CASTEP calculation depending on the values of [SPIN\_FIX](k_spin_fix_castep.htm) and
[FIX\_OCCUPANCY](k_fix_occupancy_castep.htm).

The SPIN keyword cannot be used in conjunction with either of
[NUP](k_nup_castep.htm) or [NDOWN](k_ndown_castep.htm) keywords.

## Default

0 when the total number of electrons in the system is even.

1 when the total number of electrons in the system is odd.

## Example

```

SPIN : 3
```

###### See Also:

[SPIN\_FIX](k_spin_fix_castep.htm)
  
[FIX\_OCCUPANCY](k_fix_occupancy_castep.htm)
  
[NUP](k_nup_castep.htm)
  
[NDOWN](k_ndown_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

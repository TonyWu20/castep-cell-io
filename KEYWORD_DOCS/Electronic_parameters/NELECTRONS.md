# NELECTRONS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_nelectrons_castep.htm

**Group:** Electronic parameters

---

# NELECTRONS (.param)

## Keyword type

Real

## Description

This keyword specifies the total number of electrons in the system.

## Default

If the [CHARGE](k_charge_castep.htm) keyword is specified, NELECTRONS will be chosen such that the total system charge is equal
to the argument of [CHARGE](k_charge_castep.htm).

Alternatively, if [NUP](k_nup_castep.htm) and [NDOWN](k_ndown_castep.htm) are specified, NELECTRONS will be the sum of
the arguments of [NUP](k_nup_castep.htm) and [NDOWN](k_ndown_castep.htm).

If the number of electrons is not specified, a default value will be chosen such that the system is charge neutral.

## Example

```

NELECTRONS : 3
```

###### See Also:

[CHARGE](k_charge_castep.htm)
  
[NUP](k_nup_castep.htm)
  
[NDOWN](k_ndown_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
# NDOWN

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_ndown_castep.htm

**Group:** Electronic parameters

---

# NDOWN (.param)

## Keyword type

Real

## Description

This keyword determines the total number of down-spin electrons.

This parameter is used only if [SPIN\_POLARIZED](k_spin_polarized_castep.htm) = TRUE.

## Default

If the [SPIN](k_spin_castep.htm) has been specified then:

```

	NDOWN : (NELECTRONS - SPIN)/2
```

If neither NDOWN nor [SPIN](k_spin_castep.htm) have been specified:

```

	NDOWN : NELECTRONS/2
```

## Example

```

NDOWN : 12
```

###### See Also:

[SPIN](k_spin_castep.htm)
  
[SPIN\_POLARIZED](k_spin_polarized_castep.htm)
  
[NELECTRONS](k_nelectrons_castep.htm)
  
[NUP](k_nup_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
# PSPOT_BETA_PHI_TYPE

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_pspot_beta_phi_type_castep.htm

**Group:** Pseudopotentials

---

# PSPOT\_BETA\_PHI\_TYPE (.param)

## Keyword type

String (expert)

## Description

This keyword controls the representation of the nonlocal part of the pseudopotential used for calculation of the
<β|ϕ> overlaps. Available options are:

* RECIPROCAL - reciprocal space nonlocal pseudopotentials
* REAL - real space nonlocal pseudopotentials

This parameter can only take the value REAL if
[PSPOT\_NONLOCAL\_TYPE](k_pspot_nonlocal_type_castep.htm) is also REAL.

## Default

The default is the value of [PSPOT\_NONLOCAL\_TYPE](k_pspot_nonlocal_type_castep.htm).

## Example

```

PSPOT_BETA_PHI_TYPE : REAL
```

###### See Also:

[PSPOT\_NONLOCAL\_TYPE](k_pspot_nonlocal_type_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
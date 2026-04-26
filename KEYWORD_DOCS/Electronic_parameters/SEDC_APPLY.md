# SEDC_APPLY

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_sedc_apply_castep.htm

**Group:** Electronic parameters

---

# SEDC\_APPLY (.param)

## Keyword type

Logical

## Description

This keyword specifies whether to apply a semi-empirical dispersion correction scheme to account for
van der Waals interactions in the system.

TRUE specifies a correction from the
SEDC\_SCHEME keyword will be applied and the energies and derivatives (forces and stresses) reported
in the output file [seed.castep](../expcastepfilecastep.htm) will be corrected accordingly.

## Default

FALSE

## Example

```

SEDC_APPLY : TRUE
```

###### See Also:

[SEDC\_SCHEME](k_sedc_scheme_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

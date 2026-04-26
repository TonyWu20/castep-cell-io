# MD_EXTRAP

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_md_extrap_castep.htm

**Group:** Molecular dynamics

---

# MD\_EXTRAP (.param)

## Keyword type

String

## Description

This keyword determines the wavefunction extrapolation scheme used for a molecular dynamics calculation. The same scheme is also used for
charge density extrapolation in density mixing calculations. Available options are:

* None - No extrapolation used.
* First - First order extrapolation.
* Second - Second order extrapolation.
* Mixed - Alternating first and second order extrapolation.

## Default

First

## Example

```

MD_EXTRAP : Mixed
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

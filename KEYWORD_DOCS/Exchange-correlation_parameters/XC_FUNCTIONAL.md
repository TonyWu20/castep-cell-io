# XC_FUNCTIONAL

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_xc_functional_castep.htm

**Group:** Exchange-correlation parameters

---

# XC\_FUNCTIONAL (.param)

## Keyword type

String

## Description

This keyword controls which functional is used to calculate the exchange-correlation potential. Available options are:

* LDA - Local Density Approximation
* PW91 - Perdew Wang '91 GGA
* PBE - Perdew Burke Ernzerhof
* RPBE - Revised Perdew Burke Ernzerhof
* WC - Wu-Cohen
* PBESOL - PBEsol, PBE functional for solids
* BLYP - Becke Lee Young Parr
* HF - exact exchange, no correlation
* HF-LDA - exact exchange, LDA correlation
* sX - screened exchange, no correlation
* sX-LDA - screened exchange, LDA correlation
* PBE0 - PBE0 hybrid functional
* B3LYP - B3LYP hybrid functional
* HSE03 - HSE03 hybrid functional
* HSE06 - HSE06 hybrid functional
* RSCAN - regularized SCAN meta-GGA functional

## Default

LDA

## Example

```

XC_FUNCTIONAL : PW91
```

###### See Also:

[XC\_DEFINITION](k_xc_definition_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

# BS_XC_FUNCTIONAL

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_bs_xc_functional_castep.htm

**Group:** Band structure parameters

---

# BS\_XC\_FUNCTIONAL (.param)

## Keyword type

String

## Description

This keyword controls which functional is used to determine the exchange-correlation potential during a band structure calculation. This
option allows you to apply screened and exact exchange functionals non self-consistently to obtain more accurate band gaps than with LDA or GGA
functionals. Available options are:

* LDA - Local Density Approximation
* PW91 - Perdew Wang '91 GGA
* PBE - Perdew Burke Ernzerhof
* RPBE - Revised Perdew Burke Ernzerhof
* WC - Wu-Cohen
* PBESOL - PBEsol, PBE functional for solids
* HF - exact exchange, no correlation
* HF-LDA - exact exchange, LDA correlation
* SHF - screened exchange, no correlation
* SHF-LDA - screened exchange, LDA correlation
* PBE0 - PBE0 hybrid functional
* B3LYP - B3LYP hybrid functional
* HSE03 - HSE03 hybrid functional
* HSE06 - HSE06 hybrid functional
* RSCAN - regularized SCAN meta-GGA functional

## Default

The default value is derived from the value for [XC\_FUNCTIONAL](k_xc_functional_castep.htm).

## Example

```

BS_XC_FUNCTIONAL : PW91
```

###### See Also:

[XC\_FUNCTIONAL](k_xc_functional_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
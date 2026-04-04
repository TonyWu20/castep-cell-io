# XC_DEFINITION

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_xc_definition_castep.htm

**Group:** Exchange-correlation parameters

---

# XC\_DEFINITION (.param)

## Keyword type

Block

## Description

This block allows you to construct a bespoke exchange-correlation potential by mixing contributions from various local and nonlocal functional forms.

The data block has the following format, where the number of functionals is limited only by the number of implemented functional forms and by various constraints described below:

```

%BLOCK xc_definition
<xc-functional> weight
NLXC_SCREENING_LENGTH  length [optional]
NLXC_SCREENING_FUNCTION function [optional]
NLXC_PPD_INT  ON/OFF  [optional]
NLXC_DIVERGENCE_CORR ON/OFF [optional] 
%ENDBLOCK xc_definition
```

Weights for various forms should add up to 1.0 for both exchange and correlation.

Parameters with names that begin with NLXC\_ apply to the way CASTEP calculates nonlocal exchange terms.

The NLXC\_SCREENING\_LENGTH parameter can be used to set the screening length value in Bohr-1. Default value is 1.437 Å-1 = 0.76 Bohr-1.

NLXC\_SCREENING\_FUNCTION values can be set to either THOMAS-FERMI (default) or ERRORFUNCTION.

NLXC\_DIVERGENCE\_CORR can be set to either ON or OFF. Divergence correction is ON by default when using unscreened Hartree-Fock (HF) exchange, including such functionals as B3LYP or PBE0; and OFF by default when using screened HF exchange (functionals like SX-LDA, HSE03, HSE06).

NLXC\_PPD\_INT can be set to either ON or OFF. This setting invokes integration of Coulomb terms over a parallelepiped; default value is OFF for HF exchange and ON for screened HF.

It is not possible to set both NLXC\_DIVERGENCE\_CORR and NLXC\_PPD\_INT to ON simultaneously; CASTEP can only use one of these techniques at a time.

NLXC\_DIVERGENCE\_CORR can be applied only for insulators, when fixed occupancies are used.

Available options are:

* LDA - Local Density Approximation
* LDA-X - Local Density Approximation, exchange only
* LDA-C - Local Density Approximation, correlation only
* PW91 - Perdew Wang '91 GGA
* PBE - Perdew Burke Ernzerhof GGA functional
* PBE\_X - Perdew Burke Ernzerhof GGA functional, exchange only
* PBE\_C - Perdew Burke Ernzerhof GGA functional, correlation only
* RPBE - Revised Perdew Burke Ernzerhof GGA functional
* WC - Wu-Cohen GGA functional
* PBESOL - PBEsol, PBE functional for solids
* PBESOL\_X - PBEsol, PBE functional for solids, exchange only
* PBESOL\_C - PBEsol, PBE functional for solids, correlation only
* B88\_X - [Becke (1988)](../refscastep.htm#becke_1988) GGA functional , exchange only
* LYP\_C - [Lee-Yang-Parr (1988)](../refscastep.htm#lee_1988) GGA functional, correlation only
* HF - exact exchange, no correlation
* HF-LDA - exact exchange, LDA correlation
* SX - screened exchange, no correlation
* SX-LDA - screened exchange, LDA correlation
* PBE0 - PBE0 hybrid functional
* B3LYP - B3LYP hybrid functional
* HSE03 - HSE03 hybrid functional
* HSE06 - HSE06 hybrid functional

It is not possible to add more than one exact or screened exchange functional to the mix; for example, combining HF and PBE0 is not allowed. On the other hand, any number of local forms is allowed as long as the weights add up to unity.

## Default

None

## Example

The block below corresponds to the B3LYP exchange-correlation functional:

```
%BLOCK xc_definition
LDA-X   0.08
B88_X   0.72
LYP_C   0.81
LDA-C   0.19
HF      0.2
%ENDBLOCK xc_definition
```

###### See Also:

[XC\_FUNCTIONAL](k_xc_functional_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
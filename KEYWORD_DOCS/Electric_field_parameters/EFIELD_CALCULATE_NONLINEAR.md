# EFIELD_CALCULATE_NONLINEAR

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_efield_calculate_nonlinear.htm

**Group:** Electric field parameters

---

# EFIELD\_CALCULATE\_NONLINEAR (.param)

## Keyword type

String

## Description

Specifies which non-linear optical property to calculate during a TASK=EFIELD calculation. Available options are:

* CHI2 - produces second harmonic generation coefficients.
* NONE - non-linear optical properties are not calculated.

## Default

NONE

## Example

```
EFIELD_CALCULATE_NONLINEAR : CHI2
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

# SEDC_SCHEME

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_sedc_scheme_castep.htm

**Group:** Electronic parameters

---

# SEDC\_SCHEME (.param)

## Keyword type

String

## Description

This keyword specifies the semi-empirical dispersion correction scheme to be used to account for
van der Waals interactions in the system.

Available options for the keyword values are:

* TS - Tkatchenko-Scheffler
* OBS - Ortmann, Bechstedt, and Schmidt
* G06 - Grimme 2006
* JCHS - Jurecka et al.
* MBD\* - manybody dispersion correction, Tkatchenko et al.

A more detailed description of these schemes can be found in the
Dispersion correction for DFT topic.

This parameter is only used when [SEDC\_APPLY](k_sedc_apply_castep.htm) : TRUE.

## Default

TS

## Example

```

SEDC_SCHEME : TS
```

###### See Also:

[SEDC\_APPLY](k_sedc_apply_castep.htm)
  
Dispersion correction for DFT  
[CASTEP keyword glossary](k_glossary_castep.htm)  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

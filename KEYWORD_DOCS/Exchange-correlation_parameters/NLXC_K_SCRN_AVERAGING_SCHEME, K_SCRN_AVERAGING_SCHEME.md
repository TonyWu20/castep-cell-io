# NLXC_K_SCRN_AVERAGING_SCHEME, K_SCRN_AVERAGING_SCHEME

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_k_scrn_averaging_scheme_castep.htm

**Group:** Exchange-correlation parameters

---

# NLXC\_K\_SCRN\_AVERAGING\_SCHEME or K\_SCRN\_AVERAGING\_SCHEME (.param)

## Keyword type

String

## Description

This keyword determines the averaging scheme which is used for estimating the Thomas-Fermi screening length. The screening length
is an important parameter for calculating the screened nonlocal exchange. Available options are:

* AVE\_DEN - Use averaged charge density. This option may be reasonable for 3D solids but not for systems
  containing large regions of vacuum.
* SWA\_DEN - Evaluate the self-weighted average charge density of the system and use the screening length
  associated with this value. The self-weighted average is given by the integral of the density squared divided by the integral of the
  density. This removes the problems connected with large regions of vacuum.
* SWA\_ESX - Similar to SWA\_DEN but instead the self-weighted average of
  the screened-exchange energy density is estimated.

NLXC\_K\_SCRN\_AVERAGING\_SCHEME is the latest form of this keyword, while the obsolete version, K\_SCRN\_AVERAGING\_SCHEME, is supported for
backward compatibility.

This keyword is relevant only for sX and
sX-LDA exchange correlation functionals.

## Default

AVE\_DEN

## Example

```

NLXC_K_SCRN_AVERAGING_SCHEME : SWA_DEN
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
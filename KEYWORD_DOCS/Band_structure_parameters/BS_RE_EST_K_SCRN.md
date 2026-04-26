# BS_RE_EST_K_SCRN

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_bs_re_est_k_scrn_castep.htm

**Group:** Band structure parameters

---

# BS\_RE\_EST\_K\_SCRN (.param)

## Keyword type

Logical

## Description

This keyword determines whether or not to update the estimate of the Thomas-Fermi screening length in the screened exchange formalism before
the start of a band structure calculation.

This keyword is not relevant if [RE\_EST\_K\_SCRN](k_re_est_k_scrn_castep.htm) : TRUE, since the reevaluation will happen automatically in this case.

## Default

FALSE

## Example

```

BS_RE_EST_K_SCRN : TRUE
```

###### See Also:

[RE\_EST\_K\_SCRN](k_re_est_k_scrn_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

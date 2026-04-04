# SMEARING_SCHEME

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_smearing_scheme_castep.htm

**Group:** Electronic minimization parameters

---

# SMEARING\_SCHEME (.param)

## Keyword type

String

## Description

This keyword determines the Fermi-surface smearing scheme to be used if the system is being treated as a metal. Available options are:

* Gaussian
* GaussianSplines
* FermiDirac
* HermitePolynomials
* ColdSmearing

This parameter is used only if [FIX\_OCCUPANCY](k_fix_occupancy_castep.htm) : FALSE.

## Default

Gaussian

## Example

```

SMEARING_SCHEME : ColdSmearing
```

###### See Also:

[FIX\_OCCUPANCY](k_fix_occupancy_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
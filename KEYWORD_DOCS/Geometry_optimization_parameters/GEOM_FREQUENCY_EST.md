# GEOM_FREQUENCY_EST

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_geom_frequency_est_castep.htm

**Group:** Geometry optimization parameters

---

# GEOM\_FREQUENCY\_EST (.param)

## Keyword type

Real

## Description

This keyword provides an estimate of the average phonon frequency at the gamma point. It is used to initialize the Hessian for BFGS geometry
optimization with ionic relaxation. One expects to achieve faster convergence of the geometry optimization if the value is realistic. The idea
is that this single number represents the whole of the Hessian in a compact form.

The value of the estimated frequency is provided in the CASTEP output after the geometry optimization run, and this value can be used as an
input parameter for a new calculation on a similar system. An example would be a study of different configurations of a self-interstitial defect
in silicon.

The value of the estimated frequency as returned by CASTEP should not be interpreted as an
actual frequency of any real vibrational mode.

## Default

50 THz

## Example

```

GEOM_FREQUENCY_EST : 17.54 THz
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
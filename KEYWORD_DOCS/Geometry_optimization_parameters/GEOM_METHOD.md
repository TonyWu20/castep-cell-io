# GEOM_METHOD

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_geom_method_castep.htm

**Group:** Geometry optimization parameters

---

# GEOM\_METHOD (.param)

## Keyword type

String

## Description

This keyword determines the method used for geometry optimization. Available options are:

* BFGS - BFGS minimization.
* LBFGS - low-memory BFGS minimization.
* Delocalized (or Delocalised) - BFGS minimization using delocalized internal
  coordinates instead of Cartesian coordinates.
* DampedMD - Damped molecular dynamics.
* TPSD - Two-point steepest descent.

The Delocalized and DampedMD options
are available only for geometry optimization with a fixed cell.

## Default

BFGS

## Example

```

GEOM_METHOD : DampedMD
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
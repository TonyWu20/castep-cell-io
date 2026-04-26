# GEOM_PRECONDITIONER

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_geom_precond_castep.htm

**Group:** Geometry optimization parameters

---

# GEOM\_PRECONDITIONER (.param)

## Keyword type

String

## Description

This keyword selects the preconditioner used for LBFGS geometry optimization. Available options are:

* ID - identity; LBFGS is used without a preconditioner.
* EXP - exponential preconditioner.
* FF- forcefield based preconditioner using the scheme of [Lindh *et al.* (1995)](../refscastep.htm#lindh_1995).

The ID option's only advantage over the BFGS minimizer is lower memory requirements. The EXP option is generally the best in terms of performance gains. The forcefield based preconditioner FF can sometimes provide further gains, although it is less stable and might require more steps to converge.

## Default

ID

## Example

```

GEOM_PRECONDITIONER : EXP
```

###### See Also:

[GEOM\_METHOD](k_geom_method_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

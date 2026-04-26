# MD_DAMPING_SCHEME

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_md_damping_scheme_castep.htm

**Group:** Molecular dynamics

---

# MD\_DAMPING\_SCHEME (.param)

## Keyword type

String

## Description

This keyword controls the damping scheme used during damped molecular dynamics for geometry optimization. Available options are:

* Independent - calculates optimal damping parameters according to the independent modes algorithm.
* Coupled - calculates optimal damping parameters according to the coupled modes algorithm.
* SteepestDescents - performs steepest descent dynamics.

## Default

Independent

## Example

```

MD_DAMPING_SCHEME : Coupled
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

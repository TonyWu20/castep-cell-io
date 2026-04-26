# MAGRES_TASK

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_magres_task_castep.htm

**Group:** NMR parameters

---

# MAGRES\_TASK (.param)

## Keyword type

String

## Description

This keyword defines the type of NMR calculation to be performed.

NMR in CASTEP is part of the separately licensed module NMR CASTEP. NMR calculations can only be performed if
you have purchased this module.

Available options for the `MAGRES_TASK` keyword are:

* Shielding - performs a calculation of the NMR shielding tensor for all atoms in the model.
* EFG - performs a calculation of the electric field gradient tensor for all atoms in the model.
* NMR - performs a calculation of both the NMR shielding tensor and the electric field gradient tensor for all
  atoms in the model.

## Default

Shielding

## Example

```

MAGRES_TASK : NMR
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

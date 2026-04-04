# MAGRES_MAX_CG_STEPS

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_magres_max_cg_steps_castep.htm

**Group:** NMR parameters

---

# MAGRES\_MAX\_CG\_STEPS (.param)

## Keyword type

Integer

## Description

This keyword controls the maximum number of conjugate gradient steps taken for each electronic band in the electronic minimizer during an
NMR calculation of first-order perturbed wavefunctions.

NMR in CASTEP is part of the separately licensed module NMR CASTEP. NMR calculations can only be performed if
you have purchased this module.

Apart from the initial steepest descent step, no further steepest descent steps are performed
during a calculation of first-order perturbed wavefunctions.

## Default

250

## Example

```

MAGRES_MAX_CG_STEPS : 300
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)
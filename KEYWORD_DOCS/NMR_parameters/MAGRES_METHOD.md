# MAGRES_METHOD

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_magres_method_castep.htm

**Group:** NMR parameters

---

# MAGRES\_METHOD (.param)

## Keyword type

String

## Description

This keyword selects the method used by CASTEP for the evaluation of the quantum-mechanical position operator. Available options are:

* Crystal - uses the reciprocal space representation; applicable for both truly periodic crystals and a
  "molecule in a box" supercell representation of a molecular system.
* Molecular - applicable only for a "molecule in a box" supercell representation of a molecular system. For
  such a system, the Molecular option produces a noticeably faster calculation than the
  Crystal option.

## Default

Crystal

## Example

```

MAGRES_METHOD : molecular
```

###### See Also:

[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)

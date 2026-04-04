# DIELEC_EMB_FUNC_METHOD

**Source:** https://www.tcm.phy.cam.ac.uk/castep/documentation/WebHelp/content/modules/castep/keywords/k_dielec_emb_func_method.htm

**Group:** Solvation energy parameters

---

# DIELEC\_EMB\_FUNC\_METHOD (.param)

## Keyword type

String

## Description

This keyword determines the type of dielectric function used in electrostatic embedding for implicit solvation model. The values supported are FG (Fattebert-Gygi for density-dependent dielectric) or UNIFORM (for vacuum).

## Default

FG if TASK is set to AUTOSOLVATION

UNIFORM otherwise

## Example

```

DIELEC_EMB_FUNC_METHOD : FG
```

###### See Also:

[TASK](k_task_castep.htm)
  
[CASTEP keyword glossary](k_glossary_castep.htm)
  
[CASTEP parameters keywords](k_main_parameters.htm)

BIOVIA Materials Studio 2023 Help: Monday, April 24, 2023

[Legal Notices](../../../guide/copyright_page.htm)